use std::cmp::min;

use super::char::IPAChar;

/// Possible errors occured.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IPAStringError {
    IsNotAscii,
}

/// String containing IPA pronounciation.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IPAString {
    pub chars: Vec<IPAChar>,
}

impl IPAString {
    /// Create an empty string.
    pub fn new() -> Self {
        Self { chars: Vec::new() }
    }

    /// Convert X-SAMPA encoding into IPA.
    pub fn from_xsampa(s: &str) -> Result<Self, IPAStringError> {
        if !s.is_ascii() {
            return Err(IPAStringError::IsNotAscii);
        }

        let mut ret = Self::new();
        let mut i = 0;
        let s_len = s.len();

        while i < s_len {
            let mut j = 0;
            let mut c = IPAChar::None;
            for k in 1..min(5, s_len - i + 1) {
                let _c = IPAChar::from_xsampa(s.get(i..i + k).unwrap_or_default());
                if _c != IPAChar::None {
                    j = k;
                    c = _c;
                }
            }
            if j > 0 {
                ret.chars.push(c);
                i += j;
            } else {
                i += 1;
            }
        }
        Ok(ret)
    }

    /// Count syllables.
    pub fn syllable_count(&self) -> usize {
        self.chars
            .iter()
            .filter(|&ic| *ic == IPAChar::SyllableBreak)
            .count()
            + 1
    }
}

impl Default for IPAString {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&str> for IPAString {
    fn from(s: &str) -> Self {
        let it = s.chars().into_iter().map(|c| IPAChar::from(c));
        Self {
            chars: it.collect(),
        }
    }
}

impl From<&String> for IPAString {
    fn from(s: &String) -> Self {
        Self::from(s as &str)
    }
}

impl From<String> for IPAString {
    fn from(s: String) -> Self {
        Self::from(&s as &str)
    }
}

impl From<&IPAString> for String {
    fn from(ipa_string: &IPAString) -> Self {
        let it = ipa_string.chars.iter().map(|&ic| char::from(ic));
        String::from_iter(it)
    }
}

impl From<IPAString> for String {
    fn from(ipa_string: IPAString) -> Self {
        String::from(&ipa_string)
    }
}
