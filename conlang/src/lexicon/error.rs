use std::string::FromUtf8Error;

use super::xml::XmlError;

/// Error type for `Lexicon`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Filesystem error
    #[error("Filesystem error: {0}")]
    Fs(#[from] std::io::Error),
    /// XML error
    #[error("XML: {0}")]
    Xml(#[from] XmlError),
    /// Converting from Utf8 
    #[error("Error in converting to string from UTF-8: {0}")]
    FromUtf8(#[from] FromUtf8Error),
}
