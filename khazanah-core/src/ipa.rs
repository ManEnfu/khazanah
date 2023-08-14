//! IPA symbols and related functions.

mod error;
mod xsampa;

mod symbol;

use std::str::FromStr;

pub use error::Error;
pub use symbol::{
    Backness, Delimiter, Diacritic, DiacriticPosition, Height, Ipa, Manner, Mechanism, Phonation,
    Place, Rounding, Suprasegmental, Tone, IPA_CHAR_MAP, IPA_CHAR_MAP_MAX_PATTERN_LEN,
};
pub use xsampa::{transliterate_xsampa, XSAMPA_CHAR_MAP};

use crate::utils::transliterate;

/// Parse string of IPA pronunciation to vector of `Ipa`.
pub fn parse_str(s: &str) -> Vec<Ipa> {
    transliterate(s, *IPA_CHAR_MAP_MAX_PATTERN_LEN, |s| Ipa::from_str(s).ok())
}

/// Collect vector of `Ipa` into a string.
pub fn collect_to_str(v: &[Ipa]) -> String {
    String::from_iter(v.iter().map(|c| c.symbol().unwrap_or_default()))
}
