use zip::result::ZipError;

use crate::language;

use crate::xml::XmlError;

/// Error type for `Project`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error at language domain.
    #[error("Language error: {0}")]
    Language(#[from] language::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ArchiveError {
    /// Filesystem error.
    #[error("Filesystem error: {0}")]
    Fs(#[from] std::io::Error),
    /// Error at ZIP operation.
    #[error("Error at ZIP operation: {0}")]
    Zip(#[from] ZipError),
    /// Error at Xml parsing.
    #[error("Error at XML parsing: {0}")]
    Xml(#[from] XmlError<Error>),
    /// Wrong MIME type.
    #[error("This file has wrong MIME type")]
    WrongMimeType,
}
