//! Collection of IPA symbols.
#![allow(clippy::type_complexity)]

use std::{collections::BTreeMap, fmt::Display, str::FromStr};

use once_cell::sync::Lazy;

/// The airstream mechanisms of consonants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Mechanism {
    Pulmonic,
    Ejective,
    Implosive,
    Click,
}

impl Display for Mechanism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mechanism::Pulmonic => write!(f, "pulmonic"),
            Mechanism::Ejective => write!(f, "ejective"),
            Mechanism::Implosive => write!(f, "implosive"),
            Mechanism::Click => write!(f, "click"),
        }
    }
}

/// Manner of articulation of a consonant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Manner {
    Nasal,
    Plosive,
    SibilantFricative,
    NonSibilantFricative,
    SibilantAffricate,
    NonSibilantAffricate,
    LateralFricative,
    LateralAffricate,
    Trill,
    Flap,
    LateralFlap,
    Approximant,
    LateralApproximant,

    Lateral,
    LateralNasal,
}

impl Display for Manner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Manner::Nasal => write!(f, "nasal"),
            Manner::Plosive => write!(f, "plosive"),
            Manner::SibilantFricative => write!(f, "sibilant fricative"),
            Manner::NonSibilantFricative => write!(f, "non-sibilant fricative"),
            Manner::SibilantAffricate => write!(f, "sibilant affricate"),
            Manner::NonSibilantAffricate => write!(f, "non-sibilant affricate"),
            Manner::LateralFricative => write!(f, "lateral fricative"),
            Manner::LateralAffricate => write!(f, "lateral affricate"),
            Manner::Trill => write!(f, "trill"),
            Manner::Flap => write!(f, "flap"),
            Manner::LateralFlap => write!(f, "lateral flap"),
            Manner::Approximant => write!(f, "approximant"),
            Manner::LateralApproximant => write!(f, "lateral approximant"),
            Manner::Lateral => write!(f, "lateral"),
            Manner::LateralNasal => write!(f, "lateral nasal"),
        }
    }
}

/// Place of articulation of a consonant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Place {
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

impl Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Place::Bilabial => write!(f, "bilabial"),
            Place::Labiodental => write!(f, "labiodental"),
            Place::Dental => write!(f, "dental"),
            Place::Alveolar => write!(f, "alveolar"),
            Place::PostAlveolar => write!(f, "post-alveolar"),
            Place::Retroflex => write!(f, "retroflex"),
            Place::Palatal => write!(f, "palatal"),
            Place::Velar => write!(f, "velar"),
            Place::Uvular => write!(f, "uvular"),
            Place::Pharyngeal => write!(f, "pharyngeal"),
            Place::Glottal => write!(f, "glottal"),
            Place::LabialAlveolar => write!(f, "labial-alveolar"),
            Place::LabialVelar => write!(f, "labial-velar"),
            Place::LabialPalatal => write!(f, "labial-palatal"),
            Place::UvularPharyngeal => write!(f, "uvular-pharyngeal"),
            Place::SjSound => write!(f, "sj-sound"),
        }
    }
}

impl Place {
    pub fn short_label(&self) -> &str {
        match self {
            Place::Bilabial => "BL",
            Place::Labiodental => "LD",
            Place::LabialAlveolar => "L-A",
            Place::Dental => "D",
            Place::Alveolar => "A",
            Place::PostAlveolar => "PA",
            Place::Retroflex => "RF",
            Place::LabialPalatal => "L-P",
            Place::Palatal => "P",
            Place::LabialVelar => "L-V",
            Place::Velar => "V",
            Place::Uvular => "UV",
            Place::UvularPharyngeal => "U-EG",
            Place::Pharyngeal => "EG",
            Place::Glottal => "GL",
            Place::SjSound => "SJ",
        }
    }
}

/// Phonation of a consonant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Phonation {
    Voiceless,
    Voiced,
}

impl Display for Phonation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Voiceless => write!(f, "voiceless"),
            Self::Voiced => write!(f, "voiced"),
        }
    }
}

/// Height of a vowel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Height {
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open,
}

impl Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Height::Close => write!(f, "close"),
            Height::NearClose => write!(f, "near-close"),
            Height::CloseMid => write!(f, "close-mid"),
            Height::Mid => write!(f, "mid"),
            Height::OpenMid => write!(f, "open-mid"),
            Height::NearOpen => write!(f, "near-open"),
            Height::Open => write!(f, "open"),
        }
    }
}

/// Backness of a vowel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Backness {
    Front,
    Central,
    Back,
}

impl Display for Backness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Backness::Front => write!(f, "front"),
            Backness::Central => write!(f, "central"),
            Backness::Back => write!(f, "back"),
        }
    }
}

/// Rounding of a vowel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Rounding {
    Unrounded,
    Rounded,
}

impl Display for Rounding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rounding::Unrounded => write!(f, "unrounded"),
            Rounding::Rounded => write!(f, "rounded"),
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

/// IPA Diactritics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Diacritic {
    Voiceless,
    Voiced,
    Aspirated,
    MoreRounded,
    LessRounded,
    Advanced,
    Retracted,
    Centralized,
    MidCentralized,
    Syllabic,
    NonSyllabic,
    Rhoticity,
    BreathyVoiced,
    CreakyVoiced,
    Linguolabial,
    Labialized,
    Palatalized,
    Velarized,
    Pharyngealized,
    Raised,
    Lowered,
    ATR,
    RTR,
    Dental,
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
    DoubleArticulation,
}

impl Display for Diacritic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Diacritic::Voiceless => write!(f, "voiceless"),
            Diacritic::Voiced => write!(f, "voiced"),
            Diacritic::Aspirated => write!(f, "aspirated"),
            Diacritic::MoreRounded => write!(f, "more rounded"),
            Diacritic::LessRounded => write!(f, "less rounded"),
            Diacritic::Advanced => write!(f, "advanced"),
            Diacritic::Retracted => write!(f, "retracted"),
            Diacritic::Centralized => write!(f, "centralized"),
            Diacritic::MidCentralized => write!(f, "mid-centralized"),
            Diacritic::Syllabic => write!(f, "syllabic"),
            Diacritic::NonSyllabic => write!(f, "non-syllabic"),
            Diacritic::Rhoticity => write!(f, "rhoticity"),
            Diacritic::BreathyVoiced => write!(f, "breathy voiced"),
            Diacritic::CreakyVoiced => write!(f, "creaky voiced"),
            Diacritic::Linguolabial => write!(f, "linguolabial"),
            Diacritic::Labialized => write!(f, "labialized"),
            Diacritic::Palatalized => write!(f, "palatalized"),
            Diacritic::Velarized => write!(f, "velarized"),
            Diacritic::Pharyngealized => write!(f, "pharyngealized"),
            Diacritic::Raised => write!(f, "raised"),
            Diacritic::Lowered => write!(f, "lowered"),
            Diacritic::ATR => write!(f, "ATR"),
            Diacritic::RTR => write!(f, "RTR"),
            Diacritic::Dental => write!(f, "dental"),
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
            Diacritic::DoubleArticulation => write!(f, "double articulation"),
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

/// Suprasegmental symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Suprasegmental {
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

impl Display for Suprasegmental {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suprasegmental::PrimaryStress => write!(f, "primary stress"),
            Suprasegmental::SecondaryStress => write!(f, "secondary stress"),
            Suprasegmental::Long => write!(f, "long"),
            Suprasegmental::HalfLong => write!(f, "half long"),
            Suprasegmental::ExtraShort => write!(f, "extra short"),
            Suprasegmental::SyllableBreak => write!(f, "syllable break"),
            Suprasegmental::MinorGroup => write!(f, "minor group"),
            Suprasegmental::MajorGroup => write!(f, "major group"),
            Suprasegmental::Linking => write!(f, "linking"),
            Suprasegmental::GlobalRise => write!(f, "global rise"),
            Suprasegmental::GlobalFall => write!(f, "global fall"),
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

/// IPA symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Ipa {
    Consonant {
        mechanism: Mechanism,
        manner: Manner,
        place: Place,
        phonation: Phonation,
    },
    Vowel {
        height: Height,
        backness: Backness,
        rounding: Rounding,
    },
    Tone(Tone),
    Diacritic {
        diacritic: Diacritic,
        position: DiacriticPosition,
    },
    Suprasegmental(Suprasegmental),
    Delimiter(Delimiter),
}

impl Ipa {
    /// Parses a symbol string to a corresponding IPA symbol.
    /// Returns `None` if symbol string is not a valid IPA symbol.
    /// This method might receive several variants of the same symbol.
    pub fn from_symbol(s: &str) -> Option<Ipa> {
        IPA_CHAR_MAP.1.get(s).copied()
    }

    /// Returns the symbol string of the IPA symbol.
    /// Returns `None` if the symbol is not defined.
    pub fn symbol(&self) -> Option<&'static str> {
        IPA_CHAR_MAP.0.get(self).copied()
    }

    /// Returns the name of the symbol.
    pub fn name(&self) -> String {
        match self {
            Ipa::Consonant {
                mechanism,
                manner,
                place,
                phonation,
            } => Self::consonant_name(mechanism, manner, place, phonation),
            Ipa::Vowel {
                height,
                backness,
                rounding,
            } => format!("{} {} {} vowel", height, backness, rounding),
            Ipa::Tone(t) => format!("{}", t),
            Ipa::Diacritic {
                diacritic,
                position: _,
            } => format!("{}", diacritic),
            Ipa::Suprasegmental(s) => format!("{}", s),
            Ipa::Delimiter(d) => format!("{}", d),
        }
    }

    fn consonant_name(
        mechanism: &Mechanism,
        manner: &Manner,
        place: &Place,
        phonation: &Phonation,
    ) -> String {
        let manner_str = match manner {
            Manner::Plosive => {
                if matches!(mechanism, Mechanism::Implosive | Mechanism::Click) {
                    String::default()
                } else {
                    manner.to_string()
                }
            }
            Manner::SibilantFricative => {
                if matches!(
                    place,
                    Place::Alveolar | Place::PostAlveolar | Place::Retroflex | Place::Palatal
                ) {
                    "fricative".to_string()
                } else {
                    manner.to_string()
                }
            }
            Manner::NonSibilantFricative => {
                if !matches!(
                    place,
                    Place::Alveolar | Place::PostAlveolar | Place::Retroflex
                ) {
                    "fricative".to_string()
                } else {
                    manner.to_string()
                }
            }
            Manner::SibilantAffricate => {
                if matches!(
                    place,
                    Place::Alveolar | Place::PostAlveolar | Place::Retroflex | Place::Palatal
                ) {
                    "affricate".to_string()
                } else {
                    manner.to_string()
                }
            }
            Manner::NonSibilantAffricate => {
                if !matches!(
                    place,
                    Place::Alveolar | Place::PostAlveolar | Place::Retroflex
                ) {
                    "affricate".to_string()
                } else {
                    manner.to_string()
                }
            }
            manner => manner.to_string(),
        };

        let place_str = if matches!(place, Place::Palatal)
            && matches!(
                manner,
                Manner::SibilantAffricate | Manner::SibilantFricative
            ) {
            "alveolo-palatal".to_string()
        } else {
            place.to_string()
        };

        let phonation_str = if matches!(mechanism, Mechanism::Click) {
            if matches!(manner, Manner::Nasal | Manner::LateralNasal) {
                String::default()
            } else if matches!(phonation, Phonation::Voiceless) {
                "tenuis".to_string()
            } else {
                phonation.to_string()
            }
        } else {
            phonation.to_string()
        };

        match mechanism {
            Mechanism::Pulmonic => format!("{} {} {}", phonation_str, place_str, manner_str),
            Mechanism::Ejective => format!("{} ejective {}", place_str, manner_str),
            Mechanism::Implosive => format!("{} {} implosive", phonation_str, place_str),
            Mechanism::Click => format!("{} {} {} click", phonation_str, place_str, manner_str)
                .trim_start()
                .to_string(),
        }
    }

    /// Returns the symbol string of the symbol.
    /// If the symbol is a diacritic, adds a placeholder character `◌`.
    pub fn symbol_with_placeholder(&self) -> String {
        if let Some(v) = self.symbol() {
            if let Ipa::Diacritic { .. } = self {
                format!("◌{v}")
            } else {
                v.to_string()
            }
        } else {
            "?".to_string()
        }
    }

    /// Iterates over all defined symbols.
    pub fn iter_valids() -> impl Iterator<Item = Self> {
        IPA_CHAR_MAP.0.keys().copied()
    }
}

impl FromStr for Ipa {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        IPA_CHAR_MAP.1.get(s).copied().ok_or(())
    }
}

impl Display for Ipa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name(), self.symbol().unwrap_or_default())
    }
}

pub static IPA_CHAR_MAP: Lazy<(BTreeMap<Ipa, &'static str>, BTreeMap<&'static str, Ipa>)> =
    Lazy::new(ipa_char_map);
pub static IPA_CHAR_MAP_MAX_PATTERN_LEN: Lazy<usize> = Lazy::new(|| {
    IPA_CHAR_MAP
        .1
        .keys()
        .map(|&r| r.as_bytes().len())
        .max()
        .unwrap_or_default()
});

macro_rules! map_ipa {
    (
        Consonant => {
            $($mechanism:expr => {
                $($manner:expr => {
                    $($place:expr => {
                        $($phonation:expr => [$consonant_sym:literal $(,)? $($consonant_extra_sym:literal),* $(,)?]),* $(,)?
                    }),* $(,)?
                }),* $(,)?
            }),* $(,)?
        },
        Vowel => {
            $($height:expr => {
                $($backness:expr => {
                    $($rounding:expr => [$vowel_sym:literal $(,)? $($vowel_extra_sym:literal),* $(,)?]),* $(,)?
                }),* $(,)?
            }),* $(,)?
        },
        Tone => {
            $($tone:expr => [$tone_sym:literal $(,)? $($tone_extra_sym:literal),* $(,)?]),* $(,)?
        },
        Diacritic => {
            $($dia:expr => {
                $($dia_pos:expr => [$dia_sym:literal $(,)? $($dia_extra_sym:literal),* $(,)?]),* $(,)?
            }),* $(,)?
        },
        Suprasegmental => {
            $($sup:expr => [$sup_sym:literal $(,)? $($sup_extra_sym:literal),* $(,)?]),* $(,)?
        },
        Delimiter => {
            $($delim:expr => [$delim_sym:literal $(,)? $($delim_extra_sym:literal),* $(,)?]),* $(,)?
        }
    ) => {
        let mut lmap = std::collections::BTreeMap::<$crate::ipa::symbol::Ipa, &'static str>::new();
        let mut rmap = std::collections::BTreeMap::<&'static str, $crate::ipa::symbol::Ipa>::new();

        $($($($(
            lmap.insert($crate::ipa::symbol::Ipa::Consonant {
                mechanism: $mechanism,
                manner: $manner,
                place: $place,
                phonation: $phonation
            }, $consonant_sym);

            rmap.insert($consonant_sym, $crate::ipa::symbol::Ipa::Consonant {
                mechanism: $mechanism,
                manner: $manner,
                place: $place,
                phonation: $phonation
            });

            $(
                rmap.insert($consonant_extra_sym, $crate::ipa::symbol::Ipa::Consonant {
                    mechanism: $mechanism,
                    manner: $manner,
                    place: $place,
                    phonation: $phonation
                });
            )*
        )*)*)*)*

        $($($(
            lmap.insert($crate::ipa::symbol::Ipa::Vowel {
                height: $height,
                backness: $backness,
                rounding: $rounding,
            }, $vowel_sym);

            rmap.insert($vowel_sym, $crate::ipa::symbol::Ipa::Vowel {
                height: $height,
                backness: $backness,
                rounding: $rounding,
            });

            $(
                rmap.insert($vowel_extra_sym, $crate::ipa::symbol::Ipa::Vowel {
                    height: $height,
                    backness: $backness,
                    rounding: $rounding,
                });
            )*
        )*)*)*

        $(
            lmap.insert(crate::ipa::symbol::Ipa::Tone($tone), $tone_sym);
            rmap.insert($tone_sym, crate::ipa::symbol::Ipa::Tone($tone));

            $(
                rmap.insert($tone_extra_sym, crate::ipa::symbol::Ipa::Tone($tone));
            )*
        )*

        $($(
            lmap.insert(crate::ipa::symbol::Ipa::Diacritic {
                diacritic: $dia,
                position: $dia_pos,
            }, $dia_sym);
            rmap.insert($dia_sym, crate::ipa::symbol::Ipa::Diacritic {
                diacritic: $dia,
                position: $dia_pos,
            });

            $(
                rmap.insert($dia_extra_sym, crate::ipa::symbol::Ipa::Diacritic {
                    diacritic: $dia,
                    position: $dia_pos,
                });
            )*
        )*)*

        $(
            lmap.insert(crate::ipa::symbol::Ipa::Suprasegmental($sup), $sup_sym);
            rmap.insert($sup_sym, crate::ipa::symbol::Ipa::Suprasegmental($sup));

            $(
                rmap.insert($sup_extra_sym, crate::ipa::symbol::Ipa::Suprasegmental($sup));
            )*
        )*

        $(
            lmap.insert(crate::ipa::symbol::Ipa::Delimiter($delim), $delim_sym);
            rmap.insert($delim_sym, crate::ipa::symbol::Ipa::Delimiter($delim));

            $(
                rmap.insert($delim_extra_sym, crate::ipa::symbol::Ipa::Delimiter($delim));
            )*
        )*

        (lmap, rmap)
    }
}

#[doc(hidden)]
fn ipa_char_map() -> (BTreeMap<Ipa, &'static str>, BTreeMap<&'static str, Ipa>) {
    use Manner::*;
    use Mechanism::*;
    use Phonation::*;
    use Place::*;

    use Backness::*;
    use Height::*;
    use Rounding::*;

    use DiacriticPosition::*;

    use Delimiter::*;
    use Suprasegmental::*;

    map_ipa! {
        Consonant => {
            Pulmonic => {
                Nasal => {
                    Bilabial => {
                        Voiceless => ["m̥"],
                        Voiced => ["m"],
                    },
                    Labiodental => {
                        Voiced => ["ɱ"],
                    },
                    Alveolar => {
                        Voiceless => ["n̥"],
                        Voiced => ["n"],
                    },
                    Retroflex => {
                        Voiceless => ["ɳ̊"],
                        Voiced => ["ɳ"],
                    },
                    Palatal => {
                        Voiceless => ["ɲ̊"],
                        Voiced => ["ɲ"],
                    },
                    Velar => {
                        Voiceless => ["ŋ̊"],
                        Voiced => ["ŋ"],
                    },
                    Uvular => {
                        Voiced => ["ɴ"],
                    },
                    LabialAlveolar => {
                        Voiced => ["n͡m"],
                    },
                    LabialVelar => {
                        Voiced => ["ŋ͡m"],
                    },
                },
                Plosive => {
                    Bilabial => {
                        Voiceless => ["p"],
                        Voiced => ["b"],
                    },
                    Labiodental => {
                        Voiceless => ["p̪"],
                        Voiced => ["b̪"],
                    },
                    Alveolar => {
                        Voiceless => ["t"],
                        Voiced => ["d"],
                    },
                    Retroflex => {
                        Voiceless => ["ʈ"],
                        Voiced => ["ɖ"],
                    },
                    Palatal => {
                        Voiceless => ["c"],
                        Voiced => ["ɟ"],
                    },
                    Velar => {
                        Voiceless => ["k"],
                        // "ɡ" is the correct symbol here.
                        // however, the ASCII one "g" can also be recognized as
                        // the symbol for voiced velar plosive sound.
                        Voiced => ["ɡ", "g"],
                    },
                    Uvular => {
                        Voiceless => ["q"],
                        Voiced => ["ɢ"],
                    },
                    Pharyngeal => {
                        Voiceless => ["ʡ"],
                    },
                    Glottal => {
                        Voiceless => ["ʔ"],
                    },
                    LabialAlveolar => {
                        Voiceless => ["t͡p"],
                        Voiced => ["d͡b"],
                    },
                    LabialVelar => {
                        Voiceless => ["k͡p"],
                        Voiced => ["ɡ͡b"],
                    },
                    UvularPharyngeal => {
                        Voiceless => ["q͡ʡ"],
                    },
                },
                SibilantFricative => {
                    Alveolar => {
                        Voiceless => ["s"],
                        Voiced => ["z"],
                    },
                    PostAlveolar => {
                        Voiceless => ["ʃ"],
                        Voiced => ["ʒ"],
                    },
                    Retroflex => {
                        Voiceless => ["ʂ"],
                        Voiced => ["ʐ"],
                    },
                    Palatal => {
                        Voiceless => ["ɕ"],
                        Voiced => ["ʑ"],
                    },
                },
                NonSibilantFricative => {
                    Bilabial => {
                        Voiceless => ["ɸ"],
                        Voiced => ["β"],
                    },
                    Labiodental => {
                        Voiceless => ["f"],
                        Voiced => ["v"],
                    },
                    Dental => {
                        Voiceless => ["θ"],
                        Voiced => ["ð"],
                    },
                    Alveolar => {
                        Voiceless => ["θ̠"],
                        Voiced => ["ð̠"],
                    },
                    PostAlveolar => {
                        Voiceless => ["ɹ̠̊˔"],
                        Voiced => ["ɹ̠˔"],
                    },
                    Retroflex => {
                        Voiceless => ["ɻ̊˔"],
                        Voiced => ["ɻ˔"],
                    },
                    Palatal => {
                        Voiceless => ["ç"],
                        Voiced => ["ʝ"],
                    },
                    Velar => {
                        Voiceless => ["x"],
                        Voiced => ["ɣ"],
                    },
                    Uvular => {
                        Voiceless => ["ꭓ"],
                        Voiced => ["ʁ"],
                    },
                    Pharyngeal => {
                        Voiceless => ["ħ"],
                        Voiced => ["ʕ"],
                    },
                    Glottal => {
                        Voiceless => ["h"],
                        Voiced => ["ɦ"],
                    },
                    LabialPalatal => {
                        Voiceless => ["ɥ̊"],
                    },
                    LabialVelar => {
                        Voiceless => ["ʍ"],
                    },
                },
                SibilantAffricate => {
                    Alveolar => {
                        Voiceless => ["t͡s"],
                        Voiced => ["d͡z"],
                    },
                    PostAlveolar => {
                        Voiceless => ["t͡ʃ"],
                        Voiced => ["d͡ʒ"],
                    },
                    Retroflex => {
                        Voiceless => ["ʈ͡ʂ", "t͡ʂ"],
                        Voiced => ["ɖ͡ʐ", "d͡ʐ"],
                    },
                    Palatal => {
                        Voiceless => ["t͡ɕ"],
                        Voiced => ["d͡ʑ"],
                    },
                },
                NonSibilantAffricate => {
                    Bilabial => {
                        Voiceless => ["p͡ɸ"],
                        Voiced => ["b͡ꞵ"],
                    },
                    Labiodental => {
                        Voiceless => ["p̪͡f"],
                        Voiced => ["b̪͡v"],
                    },
                    Dental => {
                        Voiceless => ["t͡θ"],
                        Voiced => ["d͡ð"],
                    },
                    Alveolar => {
                        Voiceless => ["t͡ɹ̝̊"],
                        Voiced => ["d͡ɹ̝"],
                    },
                    PostAlveolar => {
                        Voiceless => ["t̠͡ɹ̠̊˔"],
                        Voiced => ["d̠͡ɹ̠˔"],
                    },
                    Palatal => {
                        Voiceless => ["c͡ç"],
                        Voiced => ["ɟ͡ʝ"],
                    },
                    Velar => {
                        Voiceless => ["k͡x"],
                        Voiced => ["ɡ͡ɣ"],
                    },
                    Uvular => {
                        Voiceless => ["q͡ꭓ"],
                        Voiced => ["ɢ͡ʁ"],
                    },
                    Pharyngeal => {
                        Voiceless => ["ʡ͡ʜ"],
                        Voiced => ["ʡ͡ʢ"],
                    },
                    Glottal => {
                        Voiceless => ["ʔ͡h"],
                    },
                },
                Approximant => {
                    Labiodental => {
                        Voiced => ["ʋ"],
                    },
                    Alveolar => {
                        Voiced => ["ɹ"],
                    },
                    Retroflex => {
                        Voiced => ["ɻ"],
                    },
                    Palatal => {
                        Voiced => ["j"],
                    },
                    Velar => {
                        Voiced => ["ɰ"],
                    },
                    LabialPalatal => {
                        Voiced => ["ɥ"],
                    },
                    LabialVelar => {
                        Voiced => ["w"],
                    },
                },
                Flap => {
                    Bilabial => {
                        Voiced => ["ⱱ̟"],
                    },
                    Labiodental => {
                        Voiced => ["ⱱ"],
                    },
                    Alveolar => {
                        Voiceless => ["ɾ̥"],
                        Voiced => ["ɾ"],
                    },
                    Retroflex => {
                        Voiceless => ["ɽ̊"],
                        Voiced => ["ɽ"],
                    },
                    Uvular => {
                        Voiced => ["ɢ̆"],
                    },
                    Pharyngeal => {
                        Voiced => ["ʡ̆"],
                    },
                },
                Trill => {
                    Bilabial => {
                        Voiceless => ["ʙ̥"],
                        Voiced => ["ʙ"],
                    },
                    Alveolar => {
                        Voiceless => ["r̥"],
                        Voiced => ["r"],
                    },
                    Retroflex => {
                        Voiceless => ["ɽ̊r̥"],
                        Voiced => ["ɽr"],
                    },
                    Uvular => {
                        Voiceless => ["ʀ̥"],
                        Voiced => ["ʀ"],
                    },
                    Pharyngeal => {
                        Voiceless => ["ʜ"],
                        Voiced => ["ʢ"],
                    },
                },
                LateralFricative => {
                    Alveolar => {
                        Voiceless => ["ɬ"],
                        Voiced => ["ɮ"],
                    },
                    Retroflex => {
                        Voiceless => ["ꞎ"],
                        Voiced => ["ɭ˔"],
                    },
                    Palatal => {
                        Voiceless => ["ʎ̝̊"],
                        Voiced => ["ʎ̝"],
                    },
                    Velar => {
                        Voiceless => ["ʟ̝̊"],
                        Voiced => ["ʟ̝"],
                    },
                },
                LateralAffricate => {
                    Alveolar => {
                        Voiceless => ["t͡ɬ"],
                        Voiced => ["d͡ɮ"],
                    },
                    Retroflex => {
                        Voiceless => ["ʈ͡ꞎ", "t͡ꞎ"],
                        Voiced => ["ɖ͡ɭ˔", "d͡ɭ˔"],
                    },
                    Palatal => {
                        Voiceless => ["c͡ʎ̥˔"],
                        Voiced => ["ɟ͡ʎ̝"],
                    },
                    Velar => {
                        Voiceless => ["k͡ʟ̝̊"],
                        Voiced => ["ɢ͡ʟ̝"],
                    },
                },
                LateralApproximant => {
                    Alveolar => {
                        Voiced => ["l"],
                    },
                    Retroflex => {
                        Voiced => ["ɭ"],
                    },
                    Palatal => {
                        Voiced => ["ʎ"],
                    },
                    Velar => {
                        Voiced => ["ʟ"],
                    },
                    Uvular => {
                        Voiced => ["ʟ̠"],
                    },
                },
                LateralFlap => {
                    Alveolar => {
                        Voiceless => ["ɺ̥"],
                        Voiced => ["ɺ"],
                    },
                    Retroflex => {
                        Voiceless => ["ɭ̥̆"],
                        Voiced => ["ɭ̆"],
                    },
                    Palatal => {
                        Voiced => ["ʎ̆"],
                    },
                },
            },

            Ejective => {
                Plosive => {
                    Bilabial => {
                        Voiceless => ["pʼ"],
                    },
                    Alveolar => {
                        Voiceless => ["tʼ"],
                    },
                    Retroflex => {
                        Voiceless => ["ʈʼ"],
                    },
                    Palatal => {
                        Voiceless => ["cʼ"],
                    },
                    Velar => {
                        Voiceless => ["kʼ"],
                    },
                    Uvular => {
                        Voiceless => ["qʼ"],
                    },
                    Pharyngeal => {
                        Voiceless => ["ʡʼ"],
                    },
                },
                SibilantFricative => {
                    Alveolar => {
                        Voiceless => ["sʼ"],
                    },
                    PostAlveolar => {
                        Voiceless => ["ʃʼ"],
                    },
                    Retroflex => {
                        Voiceless => ["ʂʼ"],
                    },
                    Palatal => {
                        Voiceless => ["ɕʼ"],
                    },
                },
                NonSibilantFricative => {
                    Bilabial => {
                        Voiceless => ["ɸʼ"],
                    },
                    Labiodental => {
                        Voiceless => ["fʼ"],
                    },
                    Dental => {
                        Voiceless => ["θʼ"],
                    },
                    Velar => {
                        Voiceless => ["xʼ"],
                    },
                    Uvular => {
                        Voiceless => ["ꭓʼ"],
                    },
                },
                SibilantAffricate => {
                    Alveolar => {
                        Voiceless => ["t͡sʼ"],
                    },
                    PostAlveolar => {
                        Voiceless => ["t͡ʃʼ"],
                    },
                    Retroflex => {
                        Voiceless => ["ʈ͡ʂʼ", "t͡ʂʼ"],
                    },
                },
                NonSibilantAffricate => {
                    Dental => {
                        Voiceless => ["t͡θʼ"],
                    },
                    Velar => {
                        Voiceless => ["k͡xʼ"],
                    },
                    Uvular => {
                        Voiceless => ["q͡ꭓʼ"],
                    },
                },
                LateralFricative => {
                    Alveolar => {
                        Voiceless => ["ɬʼ"],
                    },
                },
                LateralAffricate => {
                    Alveolar => {
                        Voiceless => ["t͡ɬʼ"],
                    },
                    Palatal => {
                        Voiceless => ["c͡ʎ̥ʼ"],
                    },
                    Velar => {
                        Voiceless => ["k͡ʟ̝̊ʼ"],
                    },
                },
            },
            Implosive => {
                Plosive => {
                    Bilabial => {
                        Voiceless => ["ɓ̥"],
                        Voiced => ["ɓ"],
                    },
                    Alveolar => {
                        Voiceless => ["ɗ̥"],
                        Voiced => ["ɗ"],
                    },
                    Retroflex => {
                        Voiceless => ["ᶑ̊"],
                        Voiced => ["ᶑ"],
                    },
                    Palatal => {
                        Voiceless => ["ʄ̊"],
                        Voiced => ["ʄ"],
                    },
                    Velar => {
                        Voiceless => ["ɠ̊"],
                        Voiced => ["ɠ"],
                    },
                    Uvular => {
                        Voiceless => ["ʛ̥"],
                        Voiced => ["ʛ"],
                    },
                },
            },

            Click => {
                Plosive => {
                    Bilabial => {
                        Voiceless => ["ʘ"],
                        Voiced => ["g͡ʘ"],
                    },
                    Dental => {
                        Voiceless => ["ǀ"],
                        Voiced => ["g͡ǀ"],
                    },
                    Alveolar => {
                        Voiceless => ["ǃ"],
                        Voiced => ["g͡ǃ"],
                    },
                    Retroflex => {
                        Voiceless => ["‼"],
                        Voiced => ["g͡‼"],
                    },
                    Palatal => {
                        Voiceless => ["ǂ"],
                        Voiced => ["g͡ǂ"],
                    },
                },
                Nasal => {
                    Bilabial => {
                        Voiced => ["ŋ͡ʘ"],
                    },
                    Dental => {
                        Voiced => ["ŋ͡ǀ"],
                    },
                    Alveolar => {
                        Voiced => ["ŋ͡ǃ"],
                    },
                    Retroflex => {
                        Voiced => ["ŋ͡‼"],
                    },
                    Palatal => {
                        Voiced => ["ŋ͡ǂ"],
                    },
                },
                Lateral => {
                    Alveolar => {
                        Voiceless => ["ǁ"],
                        Voiced => ["g͡ǁ"],
                    },
                },
                LateralNasal => {
                    Alveolar => {
                        Voiced => ["ŋ͡ǁ"],
                    },
                },
            }
        },

        Vowel => {
            Close => {
                Front => {
                    Unrounded => ["i"],
                    Rounded => ["y"],
                },
                Central => {
                    Unrounded => ["ɨ"],
                    Rounded => ["ʉ"],
                },
                Back => {
                    Unrounded => ["ɯ"],
                    Rounded => ["u"],
                },
            },
            NearClose => {
                Front => {
                    Unrounded => ["ɪ"],
                    Rounded => ["ʏ"],
                },
                Back => {
                    Rounded => ["ʊ"],
                },
            },
            CloseMid => {
                Front => {
                    Unrounded => ["e"],
                    Rounded => ["ø"],
                },
                Central => {
                    Unrounded => ["ɘ"],
                    Rounded => ["ɵ"],
                },
                Back => {
                    Unrounded => ["ɤ"],
                    Rounded => ["o"],
                },
            },
            Mid => {
                Central => {
                    Unrounded => ["ə"],
                },
            },
            OpenMid => {
                Front => {
                    Unrounded => ["ɛ"],
                    Rounded => ["œ"],
                },
                Central => {
                    Unrounded => ["ɜ"],
                    Rounded => ["ɞ"],
                },
                Back => {
                    Unrounded => ["ʌ"],
                    Rounded => ["ɔ"],
                },
            },
            NearOpen => {
                Front => {
                    Unrounded => ["æ"],
                },
                Central => {
                    Unrounded => ["ɐ"],
                },
            },
            Open => {
                Front => {
                    Unrounded => ["a"],
                    Rounded => ["ɶ"],
                },
                Back => {
                    Unrounded => ["ɑ"],
                    Rounded => ["ɒ"],
                },
            },
        },

        Tone => {
            Tone::ExtraHigh => ["˥"],
            Tone::High => ["˦"],
            Tone::Mid => ["˧"],
            Tone::Low => ["˨"],
            Tone::ExtraLow => ["˩"],
        },

        Diacritic => {
            Diacritic::Voiceless => {
                Top => ["\u{030a}"],
                Bottom => ["\u{0325}"],
            },
            Diacritic::Voiced => {
                Bottom => ["\u{032c}"],
            },
            Diacritic::Aspirated => {
                Inline => ["ʰ"],
            },
            Diacritic::MoreRounded => {
                Top => ["\u{0325}"],
                Bottom => ["\u{0339}"],
                Inline => ["˒"],
            },
            Diacritic::LessRounded => {
                Top => ["\u{0325}"],
                Bottom => ["\u{031c}"],
                Inline => ["˓"],
            },
            Diacritic::Advanced => {
                Bottom => ["\u{031f}"],
                Inline => ["˖"],
            },
            Diacritic::Retracted => {
                Bottom => ["\u{0320}"],
                Inline => ["˗"],
            },
            Diacritic::Centralized => {
                Top => ["\u{0308}"],
            },
            Diacritic::MidCentralized => {
                Top => ["\u{033d}"],
            },
            Diacritic::Syllabic => {
                Top => ["\u{030d}"],
                Bottom => ["\u{0329}"],
            },
            Diacritic::NonSyllabic => {
                Top => ["\u{0311}"],
                Bottom => ["\u{032f}"],
            },
            Diacritic::Rhoticity => {
                Inline => ["˞"],
            },
            Diacritic::BreathyVoiced => {
                Bottom => ["\u{0324}"],
            },
            Diacritic::CreakyVoiced => {
                Bottom => ["\u{0330}"],
            },
            Diacritic::Linguolabial => {
                Bottom => ["\u{033c}"],
            },
            Diacritic::Labialized => {
                Inline => ["ʷ"],
            },
            Diacritic::Palatalized => {
                Inline => ["ʲ"],
            },
            Diacritic::Velarized => {
                Top => ["\u{0334}"],
                Inline => ["ˠ"],
            },
            Diacritic::Pharyngealized => {
                Inline => ["ˤ"],
            },
            Diacritic::Raised => {
                Bottom => ["\u{031d}"],
                Inline => ["˔"],
            },
            Diacritic::Lowered => {
                Bottom => ["\u{031e}"],
                Inline => ["˕"],
            },
            Diacritic::RTR => {
                Bottom => ["\u{0319}"],
                Inline => ["꭫"],
            },
            Diacritic::ATR => {
                Bottom => ["\u{0318}"],
                Inline => ["꭪"],
            },
            Diacritic::Dental => {
                Top => ["\u{0346}"],
                Bottom => ["\u{032a}"],
            },
            Diacritic::Apical => {
                Bottom => ["\u{033a}"],
            },
            Diacritic::Laminal => {
                Bottom => ["\u{033b}"],
            },
            Diacritic::Nasalized => {
                Top => ["\u{0303}"],
            },
            Diacritic::NasalRelease => {
                Inline => ["ⁿ"],
            },
            Diacritic::LateralRelease => {
                Inline => ["ˡ"],
            },
            Diacritic::NoAudibleRelease => {
                Top => ["\u{031a}"],
            },
            Diacritic::MidCentralVowelRelease => {
                Inline => ["ᵊ"],
            },
            Diacritic::VoicelessDentalFricativeRelease => {
                Inline => ["ᶿ"],
            },
            Diacritic::VoicelessVelarFricativeRelease => {
                Inline => ["ˣ"],
            },
            Diacritic::Ejective => {
                Inline => ["ʼ"],
            },
            Diacritic::DoubleArticulation => {
                Top => ["\u{0361}"],
                Bottom => ["\u{035c}"],
            },
        },

        Suprasegmental => {
            PrimaryStress => ["ˈ"],
            SecondaryStress => ["ˌ"],
            Long => ["ː"],
            HalfLong => ["ˑ"],
            ExtraShort => ["\u{0306}"],
            SyllableBreak => ["."],
            MinorGroup => ["|"],
            MajorGroup => ["‖"],
            Linking => ["\u{203F}"],
            GlobalRise => ["↗"],
            GlobalFall => ["↘"],
        },

        Delimiter => {
            PhoneticOpen => ["["],
            PhoneticClose => ["]"],
            Phonemic => ["/"],
            SilentOpen => ["("],
            SilentClose => [")"],
            ObscuredOpen => ["⸨"],
            ObscuredClose => ["⸩"],
            ProsodicOpen => ["{"],
            ProsodicClose => ["}"],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_symbols() {
        assert_eq!(
            Ipa::from_symbol("t͡ʂ").unwrap(),
            Ipa::Consonant {
                mechanism: Mechanism::Pulmonic,
                manner: Manner::SibilantAffricate,
                place: Place::Retroflex,
                phonation: Phonation::Voiceless,
            },
        );

        assert_eq!(
            Ipa::from_symbol("ʈ͡ʂ").unwrap(),
            Ipa::Consonant {
                mechanism: Mechanism::Pulmonic,
                manner: Manner::SibilantAffricate,
                place: Place::Retroflex,
                phonation: Phonation::Voiceless,
            },
        );

        assert_eq!(
            Ipa::Consonant {
                mechanism: Mechanism::Pulmonic,
                manner: Manner::SibilantAffricate,
                place: Place::Retroflex,
                phonation: Phonation::Voiceless,
            }
            .symbol()
            .unwrap(),
            "ʈ͡ʂ",
        );
    }
}
