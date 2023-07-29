//! Module for lexicon and related data structures.

pub use dictionary::Dictionary;
pub use error::Error;
pub use pos::{PartOfSpeech, ALL_PARTS_OF_SPEECH};
pub use word::{Word, WordBuilder};

mod dictionary;
mod error;
mod pos;
mod word;
