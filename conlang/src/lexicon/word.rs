use crate::ipa;

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
    /// Which part of speech this word belongs to.
    pub part_of_speech: Option<PartOfSpeech>,
    /// X-SAMPA pronunciation of the word, if exists.
    pub xsampa_pronunciation: Option<String>,
}

impl Word {
    /// Creates a new word.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets X-SAMPA pronunciation of the word and converts it to IPA pronunciation.
    pub fn set_xsampa_pronunciation(&mut self, s: Option<String>) {
        if let Some(s) = &s {
            self.pronunciation = ipa::transliterate_xsampa(s);
        }
        self.xsampa_pronunciation = s;
    }
}
