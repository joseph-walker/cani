use crate::data::{read_datafile, print_feature};
use crate::errors::Error;

pub fn r#use(feature: String) -> Result<(), Error> {
    let caniuse = read_datafile()?;

    if let Some(feature) = caniuse.data.get(&feature) {
        print_feature(&caniuse, feature).map_err(|_| Error::WriteError)?;

        Ok(())
    } else {
        Err(Error::ReadError) // Implement correct error here
    }
}
