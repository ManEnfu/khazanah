#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PartOfSpeech {
    Abbreviation,
    Adjective,
    Adposition,
    Adverb,
    Affix,
    Auxiliary,
    Conjunction,
    Determinative,
    Interjection,
    Noun,
    Numeral,
    Particle,
    Phrase,
    Pronoun,
    ProperNoun,
    Verb,
    // Custom(String),
}

impl PartOfSpeech {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Abbreviation => "Abbreviation",
            Self::Adjective => "Adjective",
            Self::Adposition => "Adposition",
            Self::Adverb => "Adverb",
            Self::Affix => "Affix",
            Self::Auxiliary => "Auxiliary",
            Self::Conjunction => "Conjunction",
            Self::Determinative => "Determinative",
            Self::Interjection => "Interjection",
            Self::Noun => "Noun",
            Self::Numeral => "Numeral",
            Self::Particle => "Particle",
            Self::Phrase => "Phrase",
            Self::Pronoun => "Pronoun",
            Self::ProperNoun => "Proper Noun",
            Self::Verb => "Verb",
            // Self::Custom(s) => s,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Abbreviation => "abbv.",
            Self::Adjective => "adj.",
            Self::Adposition => "adpos.",
            Self::Adverb => "Adverb",
            Self::Affix => "Affix",
            Self::Auxiliary => "aux.",
            Self::Conjunction => "conj.",
            Self::Determinative => "det.",
            Self::Interjection => "Interjection",
            Self::Noun => "n.",
            Self::Numeral => "num.",
            Self::Particle => "Particle",
            Self::Phrase => "Phrase",
            Self::Pronoun => "pron.",
            Self::ProperNoun => "ProperNoun",
            Self::Verb => "v.",
            // Self::Custom(s) => s,
        }
    }
}

impl From<&str> for PartOfSpeech {
    fn from(value: &str) -> Self {
        match value {
            "Abbreviation" => Self::Abbreviation,
            "Adjective" => Self::Adjective,
            "Adposition" => Self::Adposition,
            "Affix" => Self::Affix,
            "Auxiliary" => Self::Auxiliary,
            "Conjunction" => Self::Conjunction,
            "Interjection" => Self::Interjection,
            "Noun" => Self::Noun,
            "Numeral" => Self::Numeral,
            "Particle" => Self::Particle,
            "Phrase" => Self::Phrase,
            "Pronoun" => Self::Pronoun,
            "Proper Noun" => Self::ProperNoun,
            "Verb" => Self::Verb,
            _ => Self::Noun,
            // s => Self::Custom(s.to_owned()),
        }
    }
}

impl From<PartOfSpeech> for String {
    fn from(value: PartOfSpeech) -> Self {
        value.name().to_owned()
    }
}

pub const ALL_PARTS_OF_SPEECH: &[Option<PartOfSpeech>] = &[
    None,
    Some(PartOfSpeech::Abbreviation),
    Some(PartOfSpeech::Adjective),
    Some(PartOfSpeech::Adposition),
    Some(PartOfSpeech::Adverb),
    Some(PartOfSpeech::Affix),
    Some(PartOfSpeech::Auxiliary),
    Some(PartOfSpeech::Conjunction),
    Some(PartOfSpeech::Determinative),
    Some(PartOfSpeech::Interjection),
    Some(PartOfSpeech::Noun),
    Some(PartOfSpeech::Numeral),
    Some(PartOfSpeech::Particle),
    Some(PartOfSpeech::Phrase),
    Some(PartOfSpeech::Pronoun),
    Some(PartOfSpeech::ProperNoun),
    Some(PartOfSpeech::Verb),
];
