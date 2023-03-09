#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum PartOfSpeech {
    #[default]
    None,
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
    Custom(String),
}

impl PartOfSpeech {
    pub fn name(&self) -> &str {
        match self {
            Self::None => "---",
            Self::Abbreviation => "Abbreviation",
            Self::Adjective => "Adjective",
            Self::Adposition => "Adposition",
            Self::Affix => "Affix",
            Self::Auxiliary => "Auxiliary",
            Self::Conjunction => "Conjunction",
            Self::Interjection => "Interjection",
            Self::Noun => "Noun",
            Self::Numeral => "Numeral",
            Self::Particle => "Particle",
            Self::Phrase => "Phrase",
            Self::Pronoun => "Pronoun",
            Self::ProperNoun => "Proper Noun",
            Self::Verb => "Verb",
            Self::Custom(s) => s,
            _ => "Unknown",
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::Abbreviation => "abbv.",
            Self::Adjective => "adj.",
            Self::Adposition => "adpos.",
            Self::Affix => "Affix",
            Self::Auxiliary => "aux.",
            Self::Conjunction => "Conjunction",
            Self::Interjection => "Interjection",
            Self::Noun => "n.",
            Self::Numeral => "num.",
            Self::Particle => "Particle",
            Self::Phrase => "Phrase",
            Self::Pronoun => "pron.",
            Self::ProperNoun => "ProperNoun",
            Self::Verb => "v.",
            Self::Custom(s) => s,
            _ => "Unknown",
        }
    }
}

impl From<&str> for PartOfSpeech {
    fn from(value: &str) -> Self {
        match value {
            "---" => Self::None,
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
            s => Self::Custom(s.to_owned()),
        }
    }
}

impl From<PartOfSpeech> for String {
    fn from(value: PartOfSpeech) -> Self {
        value.name().to_owned()
    }
}

pub const ALL_PARTS_OF_SPEECH: &[PartOfSpeech] = &[
    PartOfSpeech::None,
    PartOfSpeech::Abbreviation,
    PartOfSpeech::Adjective,
    PartOfSpeech::Adposition,
    PartOfSpeech::Adverb,
    PartOfSpeech::Affix,
    PartOfSpeech::Auxiliary,
    PartOfSpeech::Conjunction,
    PartOfSpeech::Determinative,
    PartOfSpeech::Interjection,
    PartOfSpeech::Noun,
    PartOfSpeech::Numeral,
    PartOfSpeech::Particle,
    PartOfSpeech::Phrase,
    PartOfSpeech::Pronoun,
    PartOfSpeech::ProperNoun,
    PartOfSpeech::Verb,
];
