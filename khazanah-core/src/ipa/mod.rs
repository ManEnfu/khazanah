//! IPA symbols and related functions.

mod enums;
mod error;
mod xsampa;

use std::str::FromStr;

pub use enums::{
    ClickMannerOfArticulation, Delimiter, Diacritic, DiacriticPosition, FricativeVariant, Ipa,
    MannerOfArticulation, Phonation, PlaceOfArticulation, Prosody, Tone, VowelBackness,
    VowelHeight, VowelRounding, IPA_BASE_PHONEMES, IPA_CHAR_MAP, IPA_CHAR_MAP_MAX_PATTERN_LEN,
};
pub use error::Error;
pub use xsampa::{transliterate_xsampa, XSAMPA_CHAR_MAP};

use crate::utils::transliterate;

/// Parse string of IPA pronunciation to vector of `Ipa`.
pub fn parse_str(s: &str) -> Vec<Ipa> {
    transliterate(s, *IPA_CHAR_MAP_MAX_PATTERN_LEN, |s| Ipa::from_str(s).ok())
}

/// Collect vector of `Ipa` into a string.
pub fn collect_to_str(v: &[Ipa]) -> String {
    String::from_iter(v.iter().map(|c| c.to_str().unwrap_or_default()))
}
