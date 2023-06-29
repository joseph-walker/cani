use dirs::home_dir;
use serde_json::from_str;
use std::path::PathBuf;
use std::fs::read_to_string;

use crate::errors::{Error, into_write_error};
use crate::{STORAGE_FILE, STORAGE_DIRECTORY};

use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    title: String,
    description: String,
    spec: String,
    status: String,
    usage_percent_y: Option<u8>,
    usage_percent_a: Option<u8>,
    ucprefix: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CanIUse {
    pub data: BTreeMap<String, Feature>,
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