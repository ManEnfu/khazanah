//! A Library for constructed language management.

pub use ipa::{IPAChar, IPAString, IPAStringError};
pub use lexicon::{Lexicon, PartOfSpeech, Word};
pub use project::Project;

pub mod ipa;
pub mod lexicon;
pub mod phonology;
pub mod project;

#[cfg(test)]
mod tests;
