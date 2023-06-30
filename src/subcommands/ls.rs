use crate::{data::read_datafile, errors::Error};

pub fn ls() -> Result<(), Error> {
    let caniuse = read_datafile()?;

    for k in caniuse.data.keys() {
        let feature = caniuse.data.get(k).unwrap();
        let title = &feature.title;

        println!("{}: {}", k, title);
    }

    Ok(())
}
