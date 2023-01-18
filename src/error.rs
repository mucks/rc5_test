use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    KeySizeError(String),
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}
