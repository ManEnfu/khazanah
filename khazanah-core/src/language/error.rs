use crate::{lexicon, phonology};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Lexicon error: {0}")]
    Lexicon(#[from] lexicon::Error),
    #[error("Phonology error: {0}")]
    Phonology(#[from] phonology::Error),
}
