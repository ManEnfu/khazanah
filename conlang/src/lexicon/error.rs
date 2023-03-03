use std::string::FromUtf8Error;

use super::xml::XmlError;

/// Error type for `Lexicon`.
#[derive(Debug)]
pub enum Error {
    Fs(std::io::Error),
    Xml(XmlError),
    FromUtf8(FromUtf8Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Fs(value)
    }
}

impl From<XmlError> for Error {
    fn from(value: XmlError) -> Self {
        Self::Xml(value)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Self::FromUtf8(value)
    }
}
