use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fmt;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::io::Write;
use std::path::PathBuf;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use crate::errors::{into_write_error, Error};
use crate::{STORAGE_DIRECTORY, STORAGE_FILE};

macro_rules! color_write {
    ($buf:expr, $color:expr, $($args:tt)*) => {{
        $buf.set_color($color)?;
        write!($buf, $($args)*)?;
        $buf.reset()?;
    }};
}

type BrowserStats = BTreeMap<String, String>;
type FeatureStats = BTreeMap<String, BrowserStats>;

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureLink {
    url: String,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    pub title: String,
    description: String,
    spec: String,
    status: String,
    links: Vec<FeatureLink>,
    categories: Vec<String>,
    stats: FeatureStats,
    notes: String,
    notes_by_num: BTreeMap<String, String>,
    usage_perc_y: f32,
    usage_perc_a: f32,
    ucprefix: bool,
    parent: String,
    keywords: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Browser {
    browser: String,
    long_name: String,
    abbr: String,
    prefix: String,
    r#type: String,
    usage_global: BTreeMap<String, f64>,
    versions: Vec<Option<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CanIUse {
    pub data: BTreeMap<String, Feature>,
    pub agents: BTreeMap<String, Browser>,
}

#[derive(Debug)]
struct VersionGridCell<'ver> {
    start: &'ver String,
    stop: Option<&'ver String>,
    support: &'ver String,
}

impl <'ver>fmt::Display for VersionGridCell<'ver> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(stop) = self.stop {
            write!(f, "{} → {} = {}", self.start, stop, self.support)
        } else {
            write!(f, "{} = {}", self.start, self.support)
        }
    }
}

pub fn read_datafile() -> Result<CanIUse, Error> {
    if let Some(home) = home_dir() {
        let mut storage_path = PathBuf::new();

        storage_path.push(home);
        storage_path.push(STORAGE_DIRECTORY);
        storage_path.push(STORAGE_FILE);

        let datafile = read_to_string(storage_path).map_err(into_write_error)?;
        let parsed = from_str::<CanIUse>(&datafile).map_err(|_| Error::ParseError)?;

        Ok(parsed)
    } else {
        Err(Error::ReadError)
    }
}

pub fn print_feature(caniuse: &CanIUse, feature: &Feature) -> Result<(), std::io::Error> {
    let stdout_writer = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = stdout_writer.buffer();

    let mut green = ColorSpec::new();
    green.set_fg(Some(Color::Green));

    let mut yellow = ColorSpec::new();
    yellow.set_fg(Some(Color::Yellow));

    let mut red = ColorSpec::new();
    red.set_fg(Some(Color::Red));

    let buf = &mut buffer;

    // Title
    write!(buf, "Feature: ")?;
    writeln!(buf, "{}", feature.title)?;

    // Spec
    write!(buf, "Spec: ")?;
    writeln!(buf, "{}", feature.spec)?;

    // Usage percentage
    write!(buf, "Global Support: ")?;
    color_write!(buf, &green, "{}%", feature.usage_perc_y);
    write!(buf, " + ")?;
    color_write!(buf, &yellow, "{}%", feature.usage_perc_a);

    let sum_percent = feature.usage_perc_y + feature.usage_perc_a;

    write!(buf, " = {}%\n", sum_percent)?;

    // Usgae percentage key
    write!(buf, " - ")?;
    color_write!(buf, &green, "Full");
    writeln!(buf, " support")?;
    write!(buf, " - ")?;
    color_write!(buf, &yellow, "Partial");
    writeln!(buf, " support")?;

    // Description
    writeln!(buf)?;
    writeln!(buf, "{}", feature.description)?;
    writeln!(buf)?;

    // Browsers
    for (browser_name, browser_stats) in &feature.stats {
        let browser_versions = &caniuse.agents.get(browser_name).unwrap().versions;

        let mut version_grid: Vec<VersionGridCell> = vec![];

        // Use browser_versions to sort browser_stats into a sorted vector
        for version in browser_versions.iter() {
            // Get the version entry. It may be null; if so, continue to the next one
            if let Some(version) = version {
                // If it's a value, get the stat for this browser version
                let stat = browser_stats.get(version).unwrap();

                // Check the last grid cell
                if let Some(version_grid_cell) = version_grid.last_mut() {
                    // If the stat is the same value, update it
                    if version_grid_cell.support == stat {
                        version_grid_cell.stop = Some(version);
                        continue;
                    }
                }

                version_grid.push(VersionGridCell {
                    start: version,
                    stop: None,
                    support: stat,
                })
            } else {
                continue;
            }
        }

        let browser_name = &caniuse.agents.get(browser_name).unwrap().browser;

        writeln!(buf, "{browser_name}")?;

        for cell in version_grid.iter() {
            writeln!(buf, "{cell}")?;
        }

        writeln!(buf)?;
    }

    stdout_writer.print(&buffer)?;

    Ok(())
}
