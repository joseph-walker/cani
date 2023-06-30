use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
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

macro_rules! color_writeln {
    ($buf:expr, $color:expr, $($args:tt)*) => {{
        $buf.set_color($color)?;
        writeln!($buf, $($args)*)?;
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CanIUse {
    pub data: BTreeMap<String, Feature>,
    pub agents: BTreeMap<String, Browser>,
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

pub fn print_feature(feature: &Feature) -> Result<(), std::io::Error> {
    let stdout_writer = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = stdout_writer.buffer();

    let mut green = ColorSpec::new();
    green.set_fg(Some(Color::Green));

    let mut yellow = ColorSpec::new();
    yellow.set_fg(Some(Color::Yellow));

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

    stdout_writer.print(&buffer)?;

    Ok(())
}
