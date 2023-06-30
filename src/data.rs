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

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureLink {
    url: String,
    title: String,
}

type BrowserStats = BTreeMap<String, String>;
type FeatureStats = BTreeMap<String, BrowserStats>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    title: String,
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

pub fn print_feature(feature: &Feature) -> Result<(), std::io::Error> {
    let stdout_writer = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = stdout_writer.buffer();

    let mut green = ColorSpec::new();
    green.set_fg(Some(Color::Green));

    let mut yellow = ColorSpec::new();
    yellow.set_fg(Some(Color::Yellow));

    // Title
    writeln!(&mut buffer, "{}", feature.title)?;
    writeln!(&mut buffer, "{}", feature.spec)?;

    // Usage percentage
    buffer.set_color(&green)?;
    write!(&mut buffer, "{}%", feature.usage_perc_y)?;
    buffer.reset()?;

    write!(&mut buffer, " + ")?;

    buffer.set_color(&yellow)?;
    write!(&mut buffer, "{}%", feature.usage_perc_a)?;
    buffer.reset()?;

    let sum_percent = feature.usage_perc_y + feature.usage_perc_a;

    write!(&mut buffer, " = {}%\n", sum_percent)?;

    // Description
    writeln!(&mut buffer)?;
    writeln!(&mut buffer, "{}", feature.description)?;
    writeln!(&mut buffer)?;

    stdout_writer.print(&buffer)?;

    Ok(())
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
