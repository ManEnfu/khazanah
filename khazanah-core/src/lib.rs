//! A Library for constructed language management.

pub use ipa::Ipa;
pub use language::Language;
pub use lexicon::{Dictionary, PartOfSpeech, Word, ALL_PARTS_OF_SPEECH};
pub use phonology::Phoneme;
pub use project::{Project, PROJECT_FILE_EXT, PROJECT_MIME_TYPE};
pub use store::{IdAble, Store};

pub mod ipa;
pub mod language;
pub mod lexicon;
pub mod phonology;
pub mod project;

pub mod store;
pub mod utils;
pub mod xml;

pub mod prelude {
    pub use crate::xml::{ReadXml, WriteXml};
    pub use crate::IdAble;
    pub use std::io::{BufRead, Write};
}
