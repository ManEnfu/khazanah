use zip::result::ZipError;

use crate::lexicon;

/// Error type for `Project`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Filesystem error.
    #[error("Filesystem error: {0}")]
    Fs(#[from] std::io::Error),
    /// Error at ZIP operation.
    #[error("Error at ZIP operation: {0}")]
    Zip(#[from] ZipError),
    /// Error at lexicon operation.
    #[error("Lexicon: ")]
    Lexicon(#[from] lexicon::Error),
    /// Wrong MIME type.
    #[error("This file has wrong MIME type")]
    WrongMimeType,
}
