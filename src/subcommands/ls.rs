use crate::{data::read_datafile, errors::Error};

pub fn ls() -> Result<(), Error> {
    let caniuse = read_datafile()?;

    for k in caniuse.data.keys() {
        println!("{}", k);
    }

    Ok(())
}
