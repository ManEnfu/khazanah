use zip::result::ZipError;

use crate::lexicon;

/// Error type for `Project`.
#[derive(Debug)]
pub enum Error {
    Fs(std::io::Error),
    Zip(ZipError),
    Lexicon(lexicon::Error),
    WrongMimeType,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Fs(value)
    }
}

impl From<lexicon::Error> for Error {
    fn from(value: lexicon::Error) -> Self {
        Self::Lexicon(value)
    }
}

impl From<ZipError> for Error {
    fn from(value: ZipError) -> Self {
        Self::Zip(value)
    }
}
