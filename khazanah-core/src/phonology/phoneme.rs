use crate::ipa;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Phoneme {
    pub base: ipa::Ipa,
    pub modifiers: Vec<ipa::Diacritic>,
}

impl Phoneme {
    pub fn new(base: ipa::Ipa) -> Self {
        Self {
            base,
            modifiers: Vec::new(),
        }
    }

    pub fn new_with_modifiers(base: ipa::Ipa, modifiers: &[ipa::Diacritic]) -> Self {
        Self {
            base,
            modifiers: modifiers.into(),
        }
    }
}
