/// Error type that can be emitted by reading a `Lexicon` file.
#[derive(Debug, thiserror::Error)]
pub enum Error {
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
