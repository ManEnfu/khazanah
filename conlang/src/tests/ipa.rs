mod char {
    use crate::IPAChar;

    #[test]
    fn from_char() {
        assert_eq!(IPAChar::from('a'), IPAChar::OpenFrontUnroundedVowel);
        assert_eq!(IPAChar::from('ə'), IPAChar::MidCentralVowel);
        assert_eq!(IPAChar::from('ʃ'), IPAChar::VoicelessPostalveolarFricative);
        assert_eq!(IPAChar::from('+'), IPAChar::None);
    }

    #[test]
    fn to_char() {
        assert_eq!(char::from(IPAChar::OpenFrontUnroundedVowel), 'a');
        assert_eq!(char::from(IPAChar::MidCentralVowel), 'ə');
        assert_eq!(char::from(IPAChar::VoicelessPostalveolarFricative), 'ʃ');
        assert_eq!(char::from(IPAChar::None), '?');
    }

    #[test]
    fn to_xsampa() {
        assert_eq!(IPAChar::from_xsampa("a"), IPAChar::OpenFrontUnroundedVowel);
        assert_eq!(IPAChar::from_xsampa("@"), IPAChar::MidCentralVowel);
        assert_eq!(
            IPAChar::from_xsampa("S"),
            IPAChar::VoicelessPostalveolarFricative
        );
        assert_eq!(IPAChar::from_xsampa("H_"), IPAChar::None);
    }
}

mod string {
    use crate::{ipa::IPAStringError, IPAString};
    #[test]
    fn from_string() {
        let s = String::from("pɹə.nʌn.si.ˈeɪ.ʃən");
        let is = IPAString::from(&s);
        assert_eq!(String::from(is), s);
    }

    #[test]
    fn from_x_sampa() {
        let is = IPAString::from_xsampa("pr\\@.nVn.si.\"eI.S@n");
        assert!(is.is_ok());
        assert_eq!(String::from(is.unwrap()), "pɹə.nʌn.si.ˈeɪ.ʃən");

        let is = IPAString::from_xsampa("pr\\@.nan.si.\"eI.S@n");
        assert!(is.is_ok());
        assert_ne!(String::from(is.unwrap()), "pɹə.nʌn.si.ˈeɪ.ʃən");

        let is = IPAString::from_xsampa("pɹə.nʌn.si.ˈeɪ.ʃən");
        assert!(is.is_err());
        assert_eq!(is, Err(IPAStringError::IsNotAscii));
    }

    #[test]
    fn syllable_count() {
        let s = String::from("pɹə.nʌn.si.ˈeɪ.ʃən");
        let is = IPAString::from(&s);
        assert_eq!(is.syllable_count(), 5);
    }
}

mod category {
    use crate::{ipa, IPAChar};
    #[test]
    fn in_category() {
        assert!(ipa::VOWELS.contains(&IPAChar::OpenMidFrontUnroundedVowel));
        assert!(!ipa::ROUNDED_VOWELS.contains(&IPAChar::OpenMidFrontUnroundedVowel));
        assert!(ipa::UNROUNDED_VOWELS.contains(&IPAChar::OpenMidFrontUnroundedVowel));

        assert!(ipa::VOWELS.contains(&IPAChar::CloseMidBackRoundedVowel));
        assert!(ipa::ROUNDED_VOWELS.contains(&IPAChar::CloseMidBackRoundedVowel));
        assert!(!ipa::UNROUNDED_VOWELS.contains(&IPAChar::CloseMidBackRoundedVowel));

        assert!(!ipa::VOWELS.contains(&IPAChar::VoicedVelarNasal));
        assert!(!ipa::VOWELS.contains(&IPAChar::HalfLong));

        assert!(ipa::OBSTRUENTS.contains(&IPAChar::VoicedDentalFricative));
        assert!(!ipa::OBSTRUENTS.contains(&IPAChar::VoicedLabialPalatalApproximant));
    }
}
