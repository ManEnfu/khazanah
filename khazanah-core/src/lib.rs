//! A Library for constructed language management.

pub use ipa::Ipa;
pub use lexicon::{Lexicon, PartOfSpeech, Word, ALL_PARTS_OF_SPEECH};
pub use phonology::Phoneme;
pub use project::{Meta, Project, PROJECT_FILE_EXT, PROJECT_MIME_TYPE};

pub mod ipa;
pub mod language;
pub mod lexicon;
pub mod phonology;
pub mod project;

pub mod utils;
pub mod xml;
