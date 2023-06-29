use dirs::home_dir;
use reqwest::blocking::get;
use serde_json::{from_str, to_string};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

use crate::data::CanIUse;
use crate::errors::{Error, into_write_error};
use crate::{STORAGE_FILE, STORAGE_DIRECTORY};

fn fetch_caniuse_data() -> Result<CanIUse, Error> {
    let response = get("https://raw.githubusercontent.com/Fyrd/caniuse/main/data.json")
        .map_err(|_| Error::FetchError)?;

    let body = response.text().map_err(|_| Error::FetchError)?;

    from_str::<CanIUse>(&body).map_err(|_| Error::ParseError)
}

fn write_to_datafile(data: &CanIUse) -> Result<(), Error> {
    if let Some(home) = home_dir() {
        let mut storage_path = PathBuf::new();

        storage_path.push(home);
        storage_path.push(STORAGE_DIRECTORY);

        create_dir_all(storage_path.clone()).map_err(into_write_error)?;

        storage_path.push(STORAGE_FILE);

        let stringified = to_string(data).map_err(into_write_error)?;

        let mut datafile = File::create(storage_path).map_err(into_write_error)?;

        datafile
            .write_all(stringified.as_bytes())
            .map_err(into_write_error)?;

        Ok(())
    } else {
        Err(Error::WriteError)
    }
}

pub fn sync() -> Result<(), Error> {
    fetch_caniuse_data().and_then(|data| write_to_datafile(&data))
}

