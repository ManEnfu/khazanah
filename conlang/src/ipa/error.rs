#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The input string is not an ASCII string.")]
    IsNotAscii,
}
