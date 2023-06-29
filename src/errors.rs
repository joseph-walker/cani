#[derive(Debug)]
pub enum Error {
    FetchError,
    ParseError,
    WriteError,
    ReadError
}

pub fn into_write_error<T>(_: T) -> Error {
    Error::WriteError
}

