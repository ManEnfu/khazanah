use std::str::FromStr;

use bimap::BiBTreeMap;
use lazy_static::lazy_static;

/// Phonation of a consonant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Phonation {
    Voiced,
    Voiceless,
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
    Implosive,
    Click,
}

/// Variants of fricative consonants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum FricativeVariant {
    Sibilant,
    NonSibilant,
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

/// Backness of a vowel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum VowelBackness {
    Front,
    Central,
    Back,
}

/// Rounding of a vowel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum VowelRounding {
    Unrounded,
    Rounded,
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
    ProsodicClosed,
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
    VoicelesVelarFricativeRelease,
    Ejective,
    DoubleArticulation(DiacriticPosition),
}

/// Position of a diacritic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum DiacriticPosition {
    Top,
    Bottom,
    Inline,
}

/// IPA symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Ipa {
    Consonant(Phonation, PlaceOfArticulation, MannerOfArticulation),
    Vowel(VowelHeight, VowelBackness, VowelRounding),
    Prosody(Prosody),
    Tone(Tone),
    Delimiter(Delimiter),
    Diacritic(Diacritic),
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
        map.insert("ʈ͡ʂ", Consonant(Voiceless, Retroflex, Affricate(Sibilant)));
        map.insert("ɖ͡ʐ", Consonant(Voiced, Retroflex, Affricate(Sibilant)));
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
        map.insert("ˣ", Diacritic(VoicelesVelarFricativeRelease));
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
        map.insert("}", Delimiter(ProsodicClosed));
    }

    map
}
