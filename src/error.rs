use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    IOError(),
    InvalidData(String),
    TrailingCharacters,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IOError() => write!(f, "io error"),
            Error::InvalidData(msg) => write!(f, "invalid data: {msg}"),
            Error::TrailingCharacters => write!(f, "unexpected trailing characters found"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        let _ = value;
        Self::IOError()
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::InvalidData(msg.to_string())
    }
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::InvalidData(msg.to_string())
    }
}
