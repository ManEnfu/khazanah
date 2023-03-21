use std::{fmt::Display, str::FromStr};

use bimap::BiBTreeMap;
use lazy_static::lazy_static;

/// Phonation of a consonant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Phonation {
    Voiced,
    Voiceless,
}

impl Display for Phonation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Voiced => write!(f, "voiced"),
            Self::Voiceless => write!(f, "voiceless"),
        }
    }
}

/// Place of articulation of a consonant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PlaceOfArticulation {
    Bilabial,
    Labiodental,
    Dental,
    Alveolar,
    PostAlveolar,
    Retroflex,
    Palatal,
    Velar,
    Uvular,
    Pharyngeal,
    Glottal,

    LabialAlveolar,
    LabialVelar,
    LabialPalatal,
    UvularPharyngeal,
    SjSound,
}

impl Display for PlaceOfArticulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaceOfArticulation::Bilabial => write!(f, "bilabial"),
            PlaceOfArticulation::Labiodental => write!(f, "labiodental"),
            PlaceOfArticulation::Dental => write!(f, "dental"),
            PlaceOfArticulation::Alveolar => write!(f, "alveolar"),
            PlaceOfArticulation::PostAlveolar => write!(f, "post-alveolar"),
            PlaceOfArticulation::Retroflex => write!(f, "retroflex"),
            PlaceOfArticulation::Palatal => write!(f, "palatal"),
            PlaceOfArticulation::Velar => write!(f, "velar"),
            PlaceOfArticulation::Uvular => write!(f, "uvular"),
            PlaceOfArticulation::Pharyngeal => write!(f, "pharyngeal"),
            PlaceOfArticulation::Glottal => write!(f, "glottal"),
            PlaceOfArticulation::LabialAlveolar => write!(f, "labial-alveolar"),
            PlaceOfArticulation::LabialVelar => write!(f, "labial-velar"),
            PlaceOfArticulation::LabialPalatal => write!(f, "labial-palatal"),
            PlaceOfArticulation::UvularPharyngeal => write!(f, "uvular-pharyngeal"),
            PlaceOfArticulation::SjSound => write!(f, "sj-sound"),
        }
    }
}

/// Manner of articulation of a consonant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum MannerOfArticulation {
    Plosive,
    Nasal,
    Trill,
    Flap,
    LateralFlap,
    Fricative(FricativeVariant),
    Affricate(FricativeVariant),
    LateralFricative,
    LateralAffricate,
    Approximant,
    LateralApproximant,
}

impl Display for MannerOfArticulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MannerOfArticulation::Plosive => write!(f, "plosive"),
            MannerOfArticulation::Nasal => write!(f, "nasal"),
            MannerOfArticulation::Trill => write!(f, "trill"),
            MannerOfArticulation::Flap => write!(f, "tap/flap"),
            MannerOfArticulation::LateralFlap => write!(f, "lateral tap/flap"),
            MannerOfArticulation::Fricative(s) => write!(f, "{} fricative", s),
            MannerOfArticulation::Affricate(s) => write!(f, "{} affricate", s),
            MannerOfArticulation::LateralFricative => write!(f, "lateral fricative"),
            MannerOfArticulation::LateralAffricate => write!(f, "lateral affricate"),
            MannerOfArticulation::Approximant => write!(f, "approximant"),
            MannerOfArticulation::LateralApproximant => write!(f, "lateral approximant"),
        }
    }
}

/// Manner of articulation of a click consonant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ClickMannerOfArticulation {
    Tenuis,
    Voiced,
    Nasal,
    TenuisLateral,
    VoicedLateral,
    NasalLateral,
}

impl Display for ClickMannerOfArticulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tenuis => write!(f, "tenuis"),
            Self::Voiced => write!(f, "voiced"),
            Self::Nasal => write!(f, "nasal"),
            Self::TenuisLateral => write!(f, "tenuis lateral"),
            Self::VoicedLateral => write!(f, "voiced lateral"),
            Self::NasalLateral => write!(f, "nasal lateral"),
        }
    }
}

/// Variants of fricative consonants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum FricativeVariant {
    Sibilant,
    NonSibilant,
}

impl Display for FricativeVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sibilant => write!(f, "sibilant"),
            Self::NonSibilant => write!(f, "non-sibilant"),
        }
    }
}

/// Height of a vowel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum VowelHeight {
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open,
}

impl Display for VowelHeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VowelHeight::Close => write!(f, "close"),
            VowelHeight::NearClose => write!(f, "near-close"),
            VowelHeight::CloseMid => write!(f, "close-mid"),
            VowelHeight::Mid => write!(f, "mid"),
            VowelHeight::OpenMid => write!(f, "open-mid"),
            VowelHeight::NearOpen => write!(f, "near-open"),
            VowelHeight::Open => write!(f, "open"),
        }
    }
}

/// Backness of a vowel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum VowelBackness {
    Front,
    Central,
    Back,
}

impl Display for VowelBackness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VowelBackness::Front => write!(f, "front"),
            VowelBackness::Central => write!(f, "central"),
            VowelBackness::Back => write!(f, "back"),
        }
    }
}

/// Rounding of a vowel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum VowelRounding {
    Unrounded,
    Rounded,
}

impl Display for VowelRounding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VowelRounding::Unrounded => write!(f, "unrounded"),
            VowelRounding::Rounded => write!(f, "rounded"),
        }
    }
}

/// Prosody symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Prosody {
    PrimaryStress,
    SecondaryStress,
    Long,
    HalfLong,
    ExtraShort,
    SyllableBreak,
    MinorGroup,
    MajorGroup,
    Linking,
    GlobalRise,
    GlobalFall,
}

impl Display for Prosody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prosody::PrimaryStress => write!(f, "primary stress"),
            Prosody::SecondaryStress => write!(f, "secondary stress"),
            Prosody::Long => write!(f, "long"),
            Prosody::HalfLong => write!(f, "half long"),
            Prosody::ExtraShort => write!(f, "extra short"),
            Prosody::SyllableBreak => write!(f, "syllable break"),
            Prosody::MinorGroup => write!(f, "minor group"),
            Prosody::MajorGroup => write!(f, "major group"),
            Prosody::Linking => write!(f, "linking"),
            Prosody::GlobalRise => write!(f, "global rise"),
            Prosody::GlobalFall => write!(f, "global fall"),
        }
    }
}

/// Tone symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Tone {
    ExtraLow,
    Low,
    Mid,
    High,
    ExtraHigh,
    Downstep,
    Upstep,
}

impl Display for Tone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tone::ExtraLow => write!(f, "extra-low tone"),
            Tone::Low => write!(f, "low tone"),
            Tone::Mid => write!(f, "mid tone"),
            Tone::High => write!(f, "high tone"),
            Tone::ExtraHigh => write!(f, "extra-high tone"),
            Tone::Downstep => write!(f, "downstep"),
            Tone::Upstep => write!(f, "upstep"),
        }
    }
}

/// Delimiters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Delimiter {
    PhoneticOpen,
    PhoneticClose,
    Phonemic,
    SilentOpen,
    SilentClose,
    ObscuredOpen,
    ObscuredClose,
    ProsodicOpen,
    ProsodicClose,
}

impl Display for Delimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Delimiter::PhoneticOpen => write!(f, "phonetic open"),
            Delimiter::PhoneticClose => write!(f, "phonetic close"),
            Delimiter::Phonemic => write!(f, "phonemic"),
            Delimiter::SilentOpen => write!(f, "silent open"),
            Delimiter::SilentClose => write!(f, "silent close"),
            Delimiter::ObscuredOpen => write!(f, "obscured open"),
            Delimiter::ObscuredClose => write!(f, "obscured close"),
            Delimiter::ProsodicOpen => write!(f, "prosodic open"),
            Delimiter::ProsodicClose => write!(f, "prosodic close"),
        }
    }
}

/// IPA Diactritics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Diacritic {
    Voiceless(DiacriticPosition),
    Voiced,
    Aspirated,
    MoreRounded(DiacriticPosition),
    LessRounded(DiacriticPosition),
    Advanced(DiacriticPosition),
    Retracted(DiacriticPosition),
    Centralized,
    MidCentralized,
    Syllabic(DiacriticPosition),
    NonSyllabic(DiacriticPosition),
    Rhoticity,
    BreathyVoiced,
    CreakyVoiced,
    Linguolabial,
    Labialized,
    Palatalized,
    Velarized,
    Pharyngealized,
    Raised(DiacriticPosition),
    Lowered(DiacriticPosition),
    ATR(DiacriticPosition),
    RTR(DiacriticPosition),
    Dental(DiacriticPosition),
    Apical,
    Laminal,
    Nasalized,
    NasalRelease,
    LateralRelease,
    NoAudibleRelease,
    MidCentralVowelRelease,
    VoicelessDentalFricativeRelease,
    VoicelessVelarFricativeRelease,
    Ejective,
    DoubleArticulation(DiacriticPosition),
}

impl Display for Diacritic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Diacritic::Voiceless(_) => write!(f, "voiceless"),
            Diacritic::Voiced => write!(f, "voiced"),
            Diacritic::Aspirated => write!(f, "aspirated"),
            Diacritic::MoreRounded(_) => write!(f, "more rounded"),
            Diacritic::LessRounded(_) => write!(f, "less rounded"),
            Diacritic::Advanced(_) => write!(f, "advanced"),
            Diacritic::Retracted(_) => write!(f, "retracted"),
            Diacritic::Centralized => write!(f, "centralized"),
            Diacritic::MidCentralized => write!(f, "mid-centralized"),
            Diacritic::Syllabic(_) => write!(f, "syllabic"),
            Diacritic::NonSyllabic(_) => write!(f, "non-syllabic"),
            Diacritic::Rhoticity => write!(f, "rhoticity"),
            Diacritic::BreathyVoiced => write!(f, "breathy voiced"),
            Diacritic::CreakyVoiced => write!(f, "creaky voiced"),
            Diacritic::Linguolabial => write!(f, "linguolabial"),
            Diacritic::Labialized => write!(f, "labialized"),
            Diacritic::Palatalized => write!(f, "palatalized"),
            Diacritic::Velarized => write!(f, "velarized"),
            Diacritic::Pharyngealized => write!(f, "pharyngealized"),
            Diacritic::Raised(_) => write!(f, "raised"),
            Diacritic::Lowered(_) => write!(f, "lowered"),
            Diacritic::ATR(_) => write!(f, "ATR"),
            Diacritic::RTR(_) => write!(f, "RTR"),
            Diacritic::Dental(_) => write!(f, "dental"),
            Diacritic::Apical => write!(f, "apical"),
            Diacritic::Laminal => write!(f, "laminal"),
            Diacritic::Nasalized => write!(f, "nasalized"),
            Diacritic::NasalRelease => write!(f, "nasal release"),
            Diacritic::LateralRelease => write!(f, "lateral release"),
            Diacritic::NoAudibleRelease => write!(f, "no audible release"),
            Diacritic::MidCentralVowelRelease => write!(f, "mid-central vowel release"),
            Diacritic::VoicelessDentalFricativeRelease => {
                write!(f, "voiceless dental fricative release")
            }
            Diacritic::VoicelessVelarFricativeRelease => {
                write!(f, "voiceless velar fricative release")
            }
            Diacritic::Ejective => write!(f, "ejective"),
            Diacritic::DoubleArticulation(_) => write!(f, "double articulation"),
        }
    }
}

/// Position of a diacritic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum DiacriticPosition {
    Top,
    Bottom,
    Inline,
}

impl Display for DiacriticPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiacriticPosition::Top => write!(f, "top"),
            DiacriticPosition::Bottom => write!(f, "bottom"),
            DiacriticPosition::Inline => write!(f, "inline"),
        }
    }
}

/// IPA symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Ipa {
    Consonant(Phonation, PlaceOfArticulation, MannerOfArticulation),
    ImplosiveConsonant(Phonation, PlaceOfArticulation),
    EjectiveConsonant(PlaceOfArticulation, MannerOfArticulation),
    ClickConsonant(PlaceOfArticulation, ClickMannerOfArticulation),
    Vowel(VowelHeight, VowelBackness, VowelRounding),
    Prosody(Prosody),
    Tone(Tone),
    Delimiter(Delimiter),
    Diacritic(Diacritic),
}

impl Display for Ipa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ipa::Consonant(v, p, m) => write!(f, "{} {} {}", v, p, m),
            Ipa::ImplosiveConsonant(v, p) => write!(f, "{} {} implosive", v, p),
            Ipa::EjectiveConsonant(p, m) => write!(f, "{} ejective {}", p, m),
            Ipa::ClickConsonant(p, m) => write!(f, "{} {} click", m, p),
            Ipa::Vowel(h, b, r) => write!(f, "{} {} {} vowel", h, b, r),
            Ipa::Prosody(p) => write!(f, "{}", p),
            Ipa::Tone(t) => write!(f, "{}", t),
            Ipa::Delimiter(d) => write!(f, "{}", d),
            Ipa::Diacritic(d) => write!(f, "{}", d),
        }
    }
}

impl Ipa {
    /// Parse a string to an IPA symbol.
    pub fn parse_str(s: &str) -> Option<Ipa> {
        IPA_CHAR_MAP.get_by_left(s).copied()
    }
    /// Convert an IPA symbol to string.
    pub fn to_str(&self) -> Option<&'static str> {
        IPA_CHAR_MAP.get_by_right(self).copied()
    }
}

impl FromStr for Ipa {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        IPA_CHAR_MAP.get_by_left(s).copied().ok_or(())
    }
}

lazy_static! {
    pub static ref IPA_CHAR_MAP: BiBTreeMap<&'static str, Ipa> = ipa_char_map();
    pub static ref IPA_CHAR_MAP_MAX_PATTERN_LEN: usize = IPA_CHAR_MAP
        .left_values()
        .map(|&r| r.as_bytes().len())
        .max()
        .unwrap_or_default();
}

#[doc(hidden)]
fn ipa_char_map() -> BiBTreeMap<&'static str, Ipa> {
    let mut map = BiBTreeMap::new();

    use Ipa::*;
    // Consonants
    {
        use FricativeVariant::*;
        use MannerOfArticulation::*;
        use Phonation::*;
        use PlaceOfArticulation::*;

        map.insert("p", Consonant(Voiceless, Bilabial, Plosive));
        map.insert("b", Consonant(Voiced, Bilabial, Plosive));
        map.insert("p̪", Consonant(Voiceless, Labiodental, Plosive));
        map.insert("b̪", Consonant(Voiced, Labiodental, Plosive));
        map.insert("t", Consonant(Voiceless, Alveolar, Plosive));
        map.insert("d", Consonant(Voiced, Alveolar, Plosive));
        map.insert("ʈ", Consonant(Voiceless, Retroflex, Plosive));
        map.insert("ɖ", Consonant(Voiced, Retroflex, Plosive));
        map.insert("c", Consonant(Voiceless, Palatal, Plosive));
        map.insert("ɟ", Consonant(Voiced, Palatal, Plosive));
        map.insert("k", Consonant(Voiceless, Velar, Plosive));
        map.insert("ɡ", Consonant(Voiced, Velar, Plosive));
        map.insert("q", Consonant(Voiceless, Uvular, Plosive));
        map.insert("ɢ", Consonant(Voiced, Uvular, Plosive));
        map.insert("ʡ", Consonant(Voiceless, Pharyngeal, Plosive));
        map.insert("ʔ", Consonant(Voiceless, Glottal, Plosive));

        map.insert("m̥", Consonant(Voiceless, Bilabial, Nasal));
        map.insert("m", Consonant(Voiced, Bilabial, Nasal));
        map.insert("ɱ", Consonant(Voiced, Labiodental, Nasal));
        map.insert("n̥", Consonant(Voiceless, Alveolar, Nasal));
        map.insert("n", Consonant(Voiced, Alveolar, Nasal));
        map.insert("ɳ̊", Consonant(Voiceless, Retroflex, Nasal));
        map.insert("ɳ", Consonant(Voiced, Retroflex, Nasal));
        map.insert("ɲ̊", Consonant(Voiceless, Palatal, Nasal));
        map.insert("ɲ", Consonant(Voiced, Palatal, Nasal));
        map.insert("ŋ̊", Consonant(Voiceless, Velar, Nasal));
        map.insert("ŋ", Consonant(Voiced, Velar, Nasal));
        map.insert("ɴ", Consonant(Voiced, Uvular, Nasal));

        map.insert("s", Consonant(Voiceless, Alveolar, Fricative(Sibilant)));
        map.insert("z", Consonant(Voiced, Alveolar, Fricative(Sibilant)));
        map.insert("ʃ", Consonant(Voiceless, PostAlveolar, Fricative(Sibilant)));
        map.insert("ʒ", Consonant(Voiced, PostAlveolar, Fricative(Sibilant)));
        map.insert("ʂ", Consonant(Voiceless, Retroflex, Fricative(Sibilant)));
        map.insert("ʐ", Consonant(Voiced, Retroflex, Fricative(Sibilant)));
        map.insert("ɕ", Consonant(Voiceless, Palatal, Fricative(Sibilant)));
        map.insert("ʑ", Consonant(Voiced, Palatal, Fricative(Sibilant)));

        map.insert("t͡s", Consonant(Voiceless, Alveolar, Affricate(Sibilant)));
        map.insert("d͡z", Consonant(Voiced, Alveolar, Affricate(Sibilant)));
        map.insert(
            "t͡ʃ",
            Consonant(Voiceless, PostAlveolar, Affricate(Sibilant)),
        );
        map.insert("d͡ʒ", Consonant(Voiced, PostAlveolar, Affricate(Sibilant)));
        map.insert("t͡ʂ", Consonant(Voiceless, Retroflex, Affricate(Sibilant)));
        map.insert("d͡ʐ", Consonant(Voiced, Retroflex, Affricate(Sibilant)));
        map.insert("t͡ɕ", Consonant(Voiceless, Palatal, Affricate(Sibilant)));
        map.insert("d͡ʑ", Consonant(Voiced, Palatal, Affricate(Sibilant)));

        map.insert("ɸ", Consonant(Voiceless, Bilabial, Fricative(NonSibilant)));
        map.insert("β", Consonant(Voiced, Bilabial, Fricative(NonSibilant)));
        map.insert(
            "f",
            Consonant(Voiceless, Labiodental, Fricative(NonSibilant)),
        );
        map.insert("v", Consonant(Voiced, Labiodental, Fricative(NonSibilant)));
        map.insert("θ", Consonant(Voiceless, Dental, Fricative(NonSibilant)));
        map.insert("ð", Consonant(Voiced, Dental, Fricative(NonSibilant)));
        map.insert("θ̠", Consonant(Voiceless, Alveolar, Fricative(NonSibilant)));
        map.insert("ð̠", Consonant(Voiced, Alveolar, Fricative(NonSibilant)));
        map.insert(
            "ɹ̠̊˔",
            Consonant(Voiceless, PostAlveolar, Fricative(NonSibilant)),
        );
        map.insert(
            "ɹ̠˔",
            Consonant(Voiced, PostAlveolar, Fricative(NonSibilant)),
        );
        map.insert(
            "ɻ̊˔",
            Consonant(Voiceless, Retroflex, Fricative(NonSibilant)),
        );
        map.insert("ɻ˔", Consonant(Voiced, Retroflex, Fricative(NonSibilant)));
        map.insert("ç", Consonant(Voiceless, Palatal, Fricative(NonSibilant)));
        map.insert("ʝ", Consonant(Voiced, Palatal, Fricative(NonSibilant)));
        map.insert("x", Consonant(Voiceless, Velar, Fricative(NonSibilant)));
        map.insert("ɣ", Consonant(Voiced, Velar, Fricative(NonSibilant)));
        map.insert("ꭓ", Consonant(Voiceless, Uvular, Fricative(NonSibilant)));
        map.insert("ʁ", Consonant(Voiced, Uvular, Fricative(NonSibilant)));
        map.insert(
            "ħ",
            Consonant(Voiceless, Pharyngeal, Fricative(NonSibilant)),
        );
        map.insert("ʕ", Consonant(Voiced, Pharyngeal, Fricative(NonSibilant)));
        map.insert("h", Consonant(Voiceless, Glottal, Fricative(NonSibilant)));
        map.insert("ɦ", Consonant(Voiced, Glottal, Fricative(NonSibilant)));

        map.insert("p͡ɸ", Consonant(Voiceless, Bilabial, Affricate(NonSibilant)));
        map.insert("b͡ꞵ", Consonant(Voiced, Bilabial, Affricate(NonSibilant)));
        map.insert(
            "p̪͡f",
            Consonant(Voiceless, Labiodental, Affricate(NonSibilant)),
        );
        map.insert("b̪͡v", Consonant(Voiced, Labiodental, Affricate(NonSibilant)));
        map.insert("t͡θ", Consonant(Voiceless, Dental, Affricate(NonSibilant)));
        map.insert("d͡ð", Consonant(Voiced, Dental, Affricate(NonSibilant)));
        map.insert("t͡ɹ̝̊", Consonant(Voiceless, Alveolar, Affricate(NonSibilant)));
        map.insert("d͡ɹ̝", Consonant(Voiced, Alveolar, Affricate(NonSibilant)));
        map.insert(
            "t̠͡ɹ̠̊˔",
            Consonant(Voiceless, PostAlveolar, Affricate(NonSibilant)),
        );
        map.insert(
            "d̠͡ɹ̠˔",
            Consonant(Voiced, PostAlveolar, Affricate(NonSibilant)),
        );
        map.insert("c͡ç", Consonant(Voiceless, Palatal, Affricate(NonSibilant)));
        map.insert("ɟ͡ʝ", Consonant(Voiced, Palatal, Affricate(NonSibilant)));
        map.insert("k͡x", Consonant(Voiceless, Velar, Affricate(NonSibilant)));
        map.insert("ɡ͡ɣ", Consonant(Voiced, Velar, Affricate(NonSibilant)));
        map.insert("q͡ꭓ", Consonant(Voiceless, Uvular, Affricate(NonSibilant)));
        map.insert("ɢ͡ʁ", Consonant(Voiced, Uvular, Affricate(NonSibilant)));
        map.insert(
            "ʡ͡ʜ",
            Consonant(Voiceless, Pharyngeal, Affricate(NonSibilant)),
        );
        map.insert("ʡ͡ʢ", Consonant(Voiced, Pharyngeal, Affricate(NonSibilant)));
        map.insert("ʔ͡h", Consonant(Voiceless, Glottal, Affricate(NonSibilant)));

        map.insert("ʋ", Consonant(Voiced, Labiodental, Approximant));
        map.insert("ɹ", Consonant(Voiced, Alveolar, Approximant));
        map.insert("ɻ", Consonant(Voiced, Retroflex, Approximant));
        map.insert("j", Consonant(Voiced, Palatal, Approximant));
        map.insert("ɰ", Consonant(Voiced, Velar, Approximant));
        map.insert("ʔ̞", Consonant(Voiced, Glottal, Approximant));

        map.insert("ⱱ̟", Consonant(Voiced, Bilabial, Flap));
        map.insert("ⱱ", Consonant(Voiced, Labiodental, Flap));
        map.insert("ɾ̥", Consonant(Voiceless, Alveolar, Flap));
        map.insert("ɾ", Consonant(Voiced, Alveolar, Flap));
        map.insert("ɽ̊", Consonant(Voiceless, Retroflex, Flap));
        map.insert("ɽ", Consonant(Voiced, Retroflex, Flap));
        map.insert("ɢ̆", Consonant(Voiced, Uvular, Flap));
        map.insert("ʡ̆", Consonant(Voiced, Pharyngeal, Flap));

        map.insert("ʙ̥", Consonant(Voiceless, Bilabial, Trill));
        map.insert("ʙ", Consonant(Voiced, Bilabial, Trill));
        map.insert("r̥", Consonant(Voiceless, Alveolar, Trill));
        map.insert("r", Consonant(Voiced, Alveolar, Trill));
        map.insert("ɽ̊r̥", Consonant(Voiceless, Retroflex, Trill));
        map.insert("ɽr", Consonant(Voiced, Retroflex, Trill));
        map.insert("ʀ̥", Consonant(Voiceless, Uvular, Trill));
        map.insert("ʀ", Consonant(Voiced, Uvular, Trill));
        map.insert("ʜ", Consonant(Voiceless, Pharyngeal, Trill));
        map.insert("ʢ", Consonant(Voiced, Pharyngeal, Trill));

        map.insert("ɬ", Consonant(Voiceless, Alveolar, LateralFricative));
        map.insert("ɮ", Consonant(Voiced, Alveolar, LateralFricative));
        map.insert("ꞎ", Consonant(Voiceless, Retroflex, LateralFricative));
        map.insert("ɭ˔", Consonant(Voiced, Retroflex, LateralFricative));
        map.insert("ʎ̝̊", Consonant(Voiceless, Palatal, LateralFricative));
        map.insert("ʎ̝", Consonant(Voiced, Palatal, LateralFricative));
        map.insert("ʟ̝̊", Consonant(Voiceless, Velar, LateralFricative));
        map.insert("ʟ̝", Consonant(Voiced, Velar, LateralFricative));

        map.insert("t͡ɬ", Consonant(Voiceless, Alveolar, LateralAffricate));
        map.insert("d͡ɮ", Consonant(Voiced, Alveolar, LateralAffricate));
        map.insert("t͡ꞎ", Consonant(Voiceless, Retroflex, LateralAffricate));
        map.insert("d͡ɭ˔", Consonant(Voiced, Retroflex, LateralAffricate));
        map.insert("c͡ʎ̥˔", Consonant(Voiceless, Palatal, LateralAffricate));
        map.insert("ɟ͡ʎ̝", Consonant(Voiced, Palatal, LateralAffricate));
        map.insert("k͡ʟ̝̊", Consonant(Voiceless, Velar, LateralAffricate));
        map.insert("ɢ͡ʟ̝", Consonant(Voiced, Velar, LateralAffricate));

        map.insert("l", Consonant(Voiced, Alveolar, LateralApproximant));
        map.insert("ɭ", Consonant(Voiced, Retroflex, LateralApproximant));
        map.insert("ʎ", Consonant(Voiced, Palatal, LateralApproximant));
        map.insert("ʟ", Consonant(Voiced, Velar, LateralApproximant));
        map.insert("ʟ̠", Consonant(Voiced, Uvular, LateralApproximant));

        map.insert("ɺ̥", Consonant(Voiceless, Alveolar, LateralFlap));
        map.insert("ɺ", Consonant(Voiced, Alveolar, LateralFlap));
        map.insert("ɭ̥̆", Consonant(Voiceless, Retroflex, LateralFlap));
        map.insert("ɭ̆", Consonant(Voiced, Retroflex, LateralFlap));
        map.insert("ʎ̆", Consonant(Voiced, Palatal, LateralFlap));

        map.insert("n͡m", Consonant(Voiced, LabialAlveolar, Nasal));
        map.insert("ŋ͡m", Consonant(Voiced, LabialVelar, Nasal));
        map.insert(
            "ɥ̊",
            Consonant(Voiceless, LabialPalatal, Fricative(NonSibilant)),
        );
        map.insert("ɥ", Consonant(Voiced, LabialPalatal, Approximant));
        map.insert("ʍ", Consonant(Voiced, LabialVelar, Fricative(NonSibilant)));
        map.insert("w", Consonant(Voiced, LabialVelar, Approximant));
        map.insert("ɧ", Consonant(Voiced, SjSound, Fricative(Sibilant)));

        map.insert("t͡p", Consonant(Voiceless, LabialAlveolar, Plosive));
        map.insert("d͡b", Consonant(Voiced, LabialAlveolar, Plosive));
        map.insert("k͡p", Consonant(Voiceless, LabialVelar, Plosive));
        map.insert("ɡ͡b", Consonant(Voiced, LabialVelar, Plosive));
        map.insert("q͡ʡ", Consonant(Voiceless, UvularPharyngeal, Plosive));

        // Ejectives
        map.insert("pʼ", EjectiveConsonant(Bilabial, Plosive));
        map.insert("tʼ", EjectiveConsonant(Alveolar, Plosive));
        map.insert("ʈʼ", EjectiveConsonant(Retroflex, Plosive));
        map.insert("cʼ", EjectiveConsonant(Palatal, Plosive));
        map.insert("kʼ", EjectiveConsonant(Velar, Plosive));
        map.insert("qʼ", EjectiveConsonant(Uvular, Plosive));
        map.insert("ʡʼ", EjectiveConsonant(Pharyngeal, Plosive));

        map.insert("ɸʼ", EjectiveConsonant(Bilabial, Fricative(NonSibilant)));
        map.insert("fʼ", EjectiveConsonant(Labiodental, Fricative(NonSibilant)));
        map.insert("θʼ", EjectiveConsonant(Dental, Fricative(NonSibilant)));
        map.insert("sʼ", EjectiveConsonant(Alveolar, Fricative(Sibilant)));
        map.insert("ʃʼ", EjectiveConsonant(PostAlveolar, Fricative(Sibilant)));
        map.insert("ʂʼ", EjectiveConsonant(Retroflex, Fricative(Sibilant)));
        map.insert("ɕʼ", EjectiveConsonant(Palatal, Fricative(Sibilant)));
        map.insert("xʼ", EjectiveConsonant(Velar, Fricative(NonSibilant)));
        map.insert("ꭓʼ", EjectiveConsonant(Uvular, Fricative(NonSibilant)));

        map.insert("t͡θʼ", EjectiveConsonant(Dental, Affricate(NonSibilant)));
        map.insert("t͡sʼ", EjectiveConsonant(Alveolar, Affricate(Sibilant)));
        map.insert("t͡ʃʼ", EjectiveConsonant(PostAlveolar, Affricate(Sibilant)));
        map.insert("t͡ʂʼ", EjectiveConsonant(Retroflex, Affricate(Sibilant)));
        map.insert("k͡xʼ", EjectiveConsonant(Velar, Affricate(NonSibilant)));
        map.insert("q͡ꭓʼ", EjectiveConsonant(Uvular, Affricate(NonSibilant)));

        map.insert("ɬʼ", EjectiveConsonant(Alveolar, LateralFricative));

        map.insert("t͡ɬʼ", EjectiveConsonant(Alveolar, LateralAffricate));
        map.insert("c͡ʎ̥ʼ", EjectiveConsonant(Palatal, LateralAffricate));
        map.insert("k͡ʟ̝̊ʼ", EjectiveConsonant(Velar, LateralAffricate));

        // Implosives
        map.insert("ɓ̥", ImplosiveConsonant(Voiceless, Bilabial));
        map.insert("ɗ̥", ImplosiveConsonant(Voiceless, Alveolar));
        map.insert("ᶑ̊", ImplosiveConsonant(Voiceless, Retroflex));
        map.insert("ʄ̊", ImplosiveConsonant(Voiceless, Palatal));
        map.insert("ɠ̊", ImplosiveConsonant(Voiceless, Velar));
        map.insert("ʛ̥", ImplosiveConsonant(Voiceless, Uvular));

        map.insert("ɓ", ImplosiveConsonant(Voiced, Bilabial));
        map.insert("ɗ", ImplosiveConsonant(Voiced, Alveolar));
        map.insert("ᶑ", ImplosiveConsonant(Voiced, Retroflex));
        map.insert("ʄ", ImplosiveConsonant(Voiced, Palatal));
        map.insert("ɠ", ImplosiveConsonant(Voiced, Velar));
        map.insert("ʛ", ImplosiveConsonant(Voiced, Uvular));
    }
    // Clicks
    {
        use ClickMannerOfArticulation::*;
        use PlaceOfArticulation::*;

        map.insert("ʘ", ClickConsonant(Bilabial, Tenuis));
        map.insert("g͡ʘ", ClickConsonant(Bilabial, Voiced));
        map.insert("ŋ͡ʘ", ClickConsonant(Bilabial, Nasal));

        map.insert("ǀ", ClickConsonant(Dental, Tenuis));
        map.insert("g͡ǀ", ClickConsonant(Dental, Voiced));
        map.insert("ŋ͡ǀ", ClickConsonant(Dental, Nasal));

        map.insert("ǃ", ClickConsonant(Alveolar, Tenuis));
        map.insert("g͡ǃ", ClickConsonant(Alveolar, Voiced));
        map.insert("ŋ͡ǃ", ClickConsonant(Alveolar, Nasal));

        map.insert("ǁ", ClickConsonant(Alveolar, TenuisLateral));
        map.insert("g͡ǁ", ClickConsonant(Alveolar, VoicedLateral));
        map.insert("ŋ͡ǁ", ClickConsonant(Alveolar, NasalLateral));

        map.insert("‼", ClickConsonant(Retroflex, Tenuis));
        map.insert("g͡‼", ClickConsonant(Retroflex, Voiced));
        map.insert("ŋ͡‼", ClickConsonant(Retroflex, Nasal));

        map.insert("ǂ", ClickConsonant(Palatal, Tenuis));
        map.insert("g͡ǂ", ClickConsonant(Palatal, Voiced));
        map.insert("ŋ͡ǂ", ClickConsonant(Palatal, Nasal));
    }
    // Vowels
    {
        use VowelBackness::*;
        use VowelHeight::*;
        use VowelRounding::*;

        map.insert("i", Vowel(Close, Front, Unrounded));
        map.insert("y", Vowel(Close, Front, Rounded));
        map.insert("ɨ", Vowel(Close, Central, Unrounded));
        map.insert("ʉ", Vowel(Close, Central, Rounded));
        map.insert("ɯ", Vowel(Close, Back, Unrounded));
        map.insert("u", Vowel(Close, Back, Rounded));

        map.insert("ɪ", Vowel(NearClose, Front, Unrounded));
        map.insert("ʏ", Vowel(NearClose, Front, Rounded));
        map.insert("ʊ", Vowel(NearClose, Back, Rounded));

        map.insert("e", Vowel(CloseMid, Front, Unrounded));
        map.insert("ø", Vowel(CloseMid, Front, Rounded));
        map.insert("ɘ", Vowel(CloseMid, Central, Unrounded));
        map.insert("ɵ", Vowel(CloseMid, Central, Rounded));
        map.insert("ɤ", Vowel(CloseMid, Back, Unrounded));
        map.insert("o", Vowel(CloseMid, Back, Rounded));

        map.insert("ə", Vowel(Mid, Central, Unrounded));

        map.insert("ɛ", Vowel(OpenMid, Front, Unrounded));
        map.insert("œ", Vowel(OpenMid, Front, Rounded));
        map.insert("ɜ", Vowel(OpenMid, Central, Unrounded));
        map.insert("ɞ", Vowel(OpenMid, Central, Rounded));
        map.insert("ʌ", Vowel(OpenMid, Back, Unrounded));
        map.insert("ɔ", Vowel(OpenMid, Back, Rounded));

        map.insert("æ", Vowel(NearOpen, Front, Unrounded));
        map.insert("ɐ", Vowel(NearOpen, Central, Unrounded));

        map.insert("a", Vowel(Open, Front, Unrounded));
        map.insert("ɶ", Vowel(Open, Front, Rounded));
        map.insert("ɑ", Vowel(Open, Back, Unrounded));
        map.insert("ɒ", Vowel(Open, Back, Rounded));
    }
    // Prosody
    {
        use self::Prosody::*;

        map.insert("ˈ", Prosody(PrimaryStress));
        map.insert("ˌ", Prosody(SecondaryStress));
        map.insert("ː", Prosody(Long));
        map.insert("ˑ", Prosody(HalfLong));
        map.insert("\u{0306}", Prosody(ExtraShort));
        map.insert(".", Prosody(SyllableBreak));
        map.insert("|", Prosody(MinorGroup));
        map.insert("‖", Prosody(MajorGroup));
        map.insert("\u{203F}", Prosody(Linking));
        map.insert("↗", Prosody(GlobalRise));
        map.insert("↘", Prosody(GlobalFall));
    }
    // Diacritics
    {
        use self::Diacritic::*;
        use self::DiacriticPosition::*;

        map.insert("\u{030a}", Diacritic(Voiceless(Top)));
        map.insert("\u{0325}", Diacritic(Voiceless(Bottom)));
        map.insert("\u{032c}", Diacritic(Voiced));
        map.insert("ʰ", Diacritic(Aspirated));
        // map.insert("\u{0325}", Diacritic(MoreRounded(Top)));
        map.insert("\u{0339}", Diacritic(MoreRounded(Bottom)));
        map.insert("˒", Diacritic(MoreRounded(Inline)));
        // map.insert("\u{0325}", Diacritic(LessRounded(Top)));
        map.insert("\u{031c}", Diacritic(LessRounded(Bottom)));
        map.insert("˓", Diacritic(LessRounded(Inline)));
        map.insert("\u{031f}", Diacritic(Advanced(Bottom)));
        map.insert("˖", Diacritic(Advanced(Inline)));
        map.insert("\u{0320}", Diacritic(Retracted(Bottom)));
        map.insert("˗", Diacritic(Retracted(Inline)));
        map.insert("\u{0308}", Diacritic(Centralized));
        map.insert("\u{033d}", Diacritic(MidCentralized));
        map.insert("\u{030d}", Diacritic(Syllabic(Top)));
        map.insert("\u{0329}", Diacritic(Syllabic(Bottom)));
        map.insert("\u{0311}", Diacritic(NonSyllabic(Top)));
        map.insert("\u{032f}", Diacritic(NonSyllabic(Bottom)));
        map.insert("˞", Diacritic(Rhoticity));
        map.insert("\u{0324}", Diacritic(BreathyVoiced));
        map.insert("\u{0330}", Diacritic(CreakyVoiced));
        map.insert("\u{033c}", Diacritic(Linguolabial));
        map.insert("ʷ", Diacritic(Labialized));
        map.insert("ʲ", Diacritic(Palatalized));
        map.insert("ˠ", Diacritic(Velarized));
        map.insert("ˤ", Diacritic(Pharyngealized));
        map.insert("\u{031d}", Diacritic(Raised(Bottom)));
        map.insert("˔", Diacritic(Raised(Inline)));
        map.insert("\u{031e}", Diacritic(Lowered(Bottom)));
        map.insert("˕", Diacritic(Lowered(Inline)));
        map.insert("\u{0318}", Diacritic(ATR(Bottom)));
        map.insert("꭪", Diacritic(ATR(Inline)));
        map.insert("\u{0319}", Diacritic(RTR(Bottom)));
        map.insert("꭫", Diacritic(RTR(Inline)));
        map.insert("\u{0346}", Diacritic(Dental(Top)));
        map.insert("\u{032a}", Diacritic(Dental(Bottom)));
        map.insert("\u{033a}", Diacritic(Apical));
        map.insert("\u{033b}", Diacritic(Laminal));
        map.insert("\u{0303}", Diacritic(Nasalized));
        map.insert("ⁿ", Diacritic(NasalRelease));
        map.insert("ˡ", Diacritic(LateralRelease));
        map.insert("\u{031a}", Diacritic(NoAudibleRelease));
        map.insert("ᵊ", Diacritic(MidCentralVowelRelease));
        map.insert("ᶿ", Diacritic(VoicelessDentalFricativeRelease));
        map.insert("ˣ", Diacritic(VoicelessVelarFricativeRelease));
        map.insert("ʼ", Diacritic(Ejective));
        map.insert("\u{0361}", Diacritic(DoubleArticulation(Top)));
        map.insert("\u{035c}", Diacritic(DoubleArticulation(Bottom)));
    }
    // Tones
    {
        use self::Tone::*;
        map.insert("˥", Tone(ExtraHigh));
        map.insert("˦", Tone(High));
        map.insert("˧", Tone(Mid));
        map.insert("˨", Tone(Low));
        map.insert("˩", Tone(ExtraLow));
    }
    // Delimiters
    {
        use self::Delimiter::*;

        map.insert("[", Delimiter(PhoneticOpen));
        map.insert("]", Delimiter(PhoneticClose));
        map.insert("/", Delimiter(Phonemic));
        map.insert("(", Delimiter(SilentOpen));
        map.insert(")", Delimiter(SilentClose));
        map.insert("⸨", Delimiter(ObscuredOpen));
        map.insert("⸩", Delimiter(ObscuredClose));
        map.insert("{", Delimiter(ProsodicOpen));
        map.insert("}", Delimiter(ProsodicClose));
    }

    map
}
