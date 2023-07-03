use uuid::Uuid;

use crate::ipa;

/// A Phoneme.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Phoneme {
    /// The id of the phoneme.
    id: Option<Uuid>,
    /// The base IPA symbol of the phoneme.
    base: ipa::Ipa,
    /// The modifiers to apply to the phoneme.
    modifiers: Vec<ipa::Ipa>,
    /// The romanization of the phoneme.
    romanization: String,

    /// The symbol of the phoneme.
    symbol: String,
}

impl Phoneme {
    /// Creates a new phoneme.
    pub fn new(base: ipa::Ipa) -> Self {
        Self {
            id: None,
            base,
            modifiers: Vec::new(),
            romanization: String::default(),

            symbol: generate_symbol(base, &[]),
        }
    }

    // Creates a new phoneme with specified modifiers.
    pub fn new_with_modifiers(base: ipa::Ipa, modifiers: &[ipa::Ipa]) -> Self {
        Self {
            id: None,
            base,
            modifiers: modifiers.into(),
            romanization: String::default(),

            symbol: generate_symbol(base, modifiers),
        }
    }

    // Creates a new phoneme with specified id and modifiers.
    pub fn new_with_id(id: Uuid, base: ipa::Ipa, modifiers: &[ipa::Ipa]) -> Self {
        Self {
            id: Some(id),
            base,
            modifiers: modifiers.into(),
            romanization: String::default(),

            symbol: generate_symbol(base, modifiers),
        }
    }

    /// Gets the id of the phoneme.
    pub fn id(&self) -> Option<Uuid> {
        self.id
    }

    /// Generates new id for the phoneme, and then returns it.
    pub fn generate_id(&mut self) -> Uuid {
        let id = Uuid::new_v4();
        self.id = Some(id);
        id
    }

    /// Gets the base of the phoneme.
    pub fn base(&self) -> ipa::Ipa {
        self.base
    }

    /// Sets the base of the phoneme.
    pub fn set_base(&mut self, value: ipa::Ipa) {
        self.base = value;
        self.symbol = generate_symbol(self.base, &self.modifiers);
    }

    /// Gets the romanization of the phoneme.
    pub fn romanization(&self) -> &str {
        &self.romanization
    }

    /// Sets the romanization of the phoneme.
    pub fn set_romanization(&mut self, value: String) {
        self.romanization = value;
        self.symbol = generate_symbol(self.base, &self.modifiers);
    }

    /// Gets the modifiers of the phoneme.
    pub fn modifiers(&self) -> &[ipa::Ipa] {
        &self.modifiers
    }

    /// Sets the modifiers of the phoneme.
    pub fn set_modifiers(&mut self, value: Vec<ipa::Ipa>) {
        self.modifiers = value;
        self.symbol = generate_symbol(self.base, &self.modifiers);
    }

    /// Gets the symbol of the phoneme.
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

fn generate_symbol(base: ipa::Ipa, modifiers: &[ipa::Ipa]) -> String {
    modifiers
        .iter()
        .fold(base.symbol().unwrap_or_default().to_string(), |acc, x| {
            acc + x.symbol().unwrap_or_default()
        })
}

/// A builder struct of a phoneme.
pub struct PhonemeBuilder {
    inner: Phoneme,
}

impl PhonemeBuilder {
    pub fn new() -> Self {
        Self {
            inner: Phoneme::new(ipa::Ipa::Vowel(
                ipa::VowelHeight::Open,
                ipa::VowelBackness::Front,
                ipa::VowelRounding::Unrounded,
            )),
        }
    }

    pub fn base(mut self, value: ipa::Ipa) -> Self {
        self.inner.set_base(value);
        self
    }

    pub fn romanization(mut self, value: String) -> Self {
        self.inner.set_romanization(value);
        self
    }

    pub fn modifiers(mut self, value: Vec<ipa::Ipa>) -> Self {
        self.inner.set_modifiers(value);
        self
    }

    pub fn build(self) -> Phoneme {
        self.inner
    }
}

impl Default for PhonemeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
