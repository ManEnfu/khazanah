mod char {
    use std::str::FromStr;

    use crate::{
        ipa::{
            FricativeVariant, MannerOfArticulation, Phonation, PlaceOfArticulation, VowelBackness,
            VowelHeight, VowelRounding,
        },
        Ipa,
    };

    #[test]
    fn from_char() {
        assert_eq!(
            Ipa::from_str("a"),
            Ok(Ipa::Vowel(
                VowelHeight::Open,
                VowelBackness::Front,
                VowelRounding::Unrounded
            )),
        );
        assert_eq!(
            Ipa::from_str("ə"),
            Ok(Ipa::Vowel(
                VowelHeight::Mid,
                VowelBackness::Central,
                VowelRounding::Unrounded
            )),
        );
        assert_eq!(
            Ipa::from_str("ʃ"),
            Ok(Ipa::Consonant(
                Phonation::Voiceless,
                PlaceOfArticulation::PostAlveolar,
                MannerOfArticulation::Fricative(FricativeVariant::Sibilant)
            )),
        );
        assert_eq!(
            Ipa::from_str("d͡ʑ"),
            Ok(Ipa::Consonant(
                Phonation::Voiced,
                PlaceOfArticulation::Palatal,
                MannerOfArticulation::Affricate(FricativeVariant::Sibilant)
            )),
        );
        assert_eq!(Ipa::from_str("+"), Err(()));
    }

    #[test]
    fn to_char() {
        assert_eq!(
            Ipa::Vowel(
                VowelHeight::Open,
                VowelBackness::Front,
                VowelRounding::Unrounded
            )
            .to_str(),
            Some("a"),
        );
        assert_eq!(
            Ipa::Vowel(
                VowelHeight::Mid,
                VowelBackness::Central,
                VowelRounding::Unrounded
            )
            .to_str(),
            Some("ə"),
        );
        assert_eq!(
            Ipa::Consonant(
                Phonation::Voiceless,
                PlaceOfArticulation::PostAlveolar,
                MannerOfArticulation::Fricative(FricativeVariant::Sibilant)
            )
            .to_str(),
            Some("ʃ"),
        );
        assert_eq!(
            Ipa::Consonant(
                Phonation::Voiced,
                PlaceOfArticulation::Palatal,
                MannerOfArticulation::Affricate(FricativeVariant::Sibilant)
            )
            .to_str(),
            Some("d͡ʑ"),
        );
        assert_eq!(
            Ipa::Consonant(
                Phonation::Voiceless,
                PlaceOfArticulation::Bilabial,
                MannerOfArticulation::LateralFricative,
            )
            .to_str(),
            None
        );
    }
}

mod string {
    use crate::{
        ipa::{
            self, FricativeVariant, MannerOfArticulation, Phonation, PlaceOfArticulation, Prosody,
            VowelBackness, VowelHeight, VowelRounding,
        },
        Ipa,
    };
    #[test]
    fn from_string() {
        let ipa1 = "pɹə.nʌn.si.ˈeɪ.ʃən";
        let ipa2 = vec![
            Ipa::Consonant(
                Phonation::Voiceless,
                PlaceOfArticulation::Bilabial,
                MannerOfArticulation::Plosive,
            ),
            Ipa::Consonant(
                Phonation::Voiced,
                PlaceOfArticulation::Alveolar,
                MannerOfArticulation::Approximant,
            ),
            Ipa::Vowel(
                VowelHeight::Mid,
                VowelBackness::Central,
                VowelRounding::Unrounded,
            ),
            Ipa::Prosody(Prosody::SyllableBreak),
            Ipa::Consonant(
                Phonation::Voiced,
                PlaceOfArticulation::Alveolar,
                MannerOfArticulation::Nasal,
            ),
            Ipa::Vowel(
                VowelHeight::OpenMid,
                VowelBackness::Back,
                VowelRounding::Unrounded,
            ),
            Ipa::Consonant(
                Phonation::Voiced,
                PlaceOfArticulation::Alveolar,
                MannerOfArticulation::Nasal,
            ),
            Ipa::Prosody(Prosody::SyllableBreak),
            Ipa::Consonant(
                Phonation::Voiceless,
                PlaceOfArticulation::Alveolar,
                MannerOfArticulation::Fricative(FricativeVariant::Sibilant),
            ),
            Ipa::Vowel(
                VowelHeight::Close,
                VowelBackness::Front,
                VowelRounding::Unrounded,
            ),
            Ipa::Prosody(Prosody::SyllableBreak),
            Ipa::Prosody(Prosody::PrimaryStress),
            Ipa::Vowel(
                VowelHeight::CloseMid,
                VowelBackness::Front,
                VowelRounding::Unrounded,
            ),
            Ipa::Vowel(
                VowelHeight::NearClose,
                VowelBackness::Front,
                VowelRounding::Unrounded,
            ),
            Ipa::Prosody(Prosody::SyllableBreak),
            Ipa::Consonant(
                Phonation::Voiceless,
                PlaceOfArticulation::PostAlveolar,
                MannerOfArticulation::Fricative(FricativeVariant::Sibilant),
            ),
            Ipa::Vowel(
                VowelHeight::Mid,
                VowelBackness::Central,
                VowelRounding::Unrounded,
            ),
            Ipa::Consonant(
                Phonation::Voiced,
                PlaceOfArticulation::Alveolar,
                MannerOfArticulation::Nasal,
            ),
        ];
        assert_eq!(&ipa::parse_str(ipa1), &ipa2);
        assert_eq!(&ipa::collect_to_str(&ipa2), ipa1);
    }

    #[test]
    fn from_xsampa() {
        let xsampa_str = "pr\\@.nVn.si.\"eI.S@n";
        let ipa_str = "pɹə.nʌn.si.ˈeɪ.ʃən";
        assert_eq!(&ipa::transliterate_xsampa(xsampa_str), ipa_str);

        let xsampa_str = "ar_?\\.b_?\\@.<\\a";
        let ipa_str = "arˤ.bˤə.ʢa";
        assert_eq!(&ipa::transliterate_xsampa(xsampa_str), ipa_str);
    }
}
