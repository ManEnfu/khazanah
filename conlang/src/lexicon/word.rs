use super::PartOfSpeech;
use std::fmt::Debug;

/// Word entry in the lexicon.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Word {
    /// Romanization of the word.
    pub romanization: String,
    /// Translation of the word.
    pub translation: String,
    /// Pronunciation of word in IPA.
    pub pronunciation: String,
    /// Whic part of speech this word belongs to.
    pub part_of_speech: Option<PartOfSpeech>,
}

impl Word {
    /// Creates a new word.
    pub fn new() -> Self {
        Self::default()
    }
}
