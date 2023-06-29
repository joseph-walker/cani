use crate::data::read_datafile;
use crate::errors::Error;

pub fn r#use(feature: String) -> Result<(), Error> {
    let caniuse = read_datafile()?;
    if let Some(feature) = caniuse.data.get(&feature) {
        println!("{}", feature);
        Ok(())
    } else {
        Err(Error::ReadError) // Implement correct error here
    }
}
