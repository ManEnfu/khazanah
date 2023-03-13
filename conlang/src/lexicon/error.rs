use std::string::FromUtf8Error;

use crate::xml::XmlError;

/// Error type for `Lexicon`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Filesystem error
    #[error("Filesystem error: {0}")]
    Fs(#[from] std::io::Error),
    /// XML error
    #[error("XML: {0}")]
    Xml(#[from] XmlError<ReadError>),
    /// Converting from Utf8
    #[error("Error in converting to string from UTF-8: {0}")]
    FromUtf8(#[from] FromUtf8Error),
}

/// Error type that can be emitted by reading a `Lexicon` file.
#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    /// Trying to set value of a nonexistent word. This should not happen.
    #[error("Reader tried to set value of a nonexistent `Word`")]
    WriteInvalidWord,
    /// A valid tag in a wrong context.
    #[error("tag <{}> should not be inside <{}>", .tag, .ptag)]
    WrongContext { ptag: String, tag: String },
    /// <word> tag doesn't have attribute `id`.
    #[error("<word> tag doesn't have attribute `id`")]
    NoId,
    #[error("Id error: {0}")]
    Id(#[from] uuid::Error),
}
