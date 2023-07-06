/// Error type relating to phonology domain.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// <phoneme> tag doesn't have attribute `id`.
    #[error("<phoneme> tag doesn't have attribute `id`")]
    NoId,
    #[error("Id error: {0}")]
    Id(#[from] uuid::Error),
}
