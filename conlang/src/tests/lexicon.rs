use crate::lexicon::Lexicon;
mod xml {
    use crate::lexicon::{PartOfSpeech, Word};

    use super::*;

    fn test_lex() -> Lexicon {
        let mut lex = Lexicon::new();
        lex.add_word(Word {
            romanization: "nifutu".to_string(),
            pronunciation: "ˈni.ɸu.tu".to_string(),
            translation: "sun".to_string(),
            part_of_speech: Some(PartOfSpeech::Noun),
        });
        lex.add_word(Word {
            romanization: "xahlauraqi".to_string(),
            pronunciation: "ˈxa.ɬa.u.ɹa.qi".to_string(),
            translation: "story".to_string(),
            part_of_speech: Some(PartOfSpeech::Noun),
        });
        lex.add_word(Word {
            romanization: "pfunutsaaxi".to_string(),
            pronunciation: "ˈpɸu.nu.tsaː.xi".to_string(),
            translation: "flow".to_string(),
            part_of_speech: Some(PartOfSpeech::Verb),
        });
        lex
    }

    #[test]
    fn read_xml() {
        let xml = r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <lexicon>
                <word>
                    <romanization>nifutu</romanization>
                    <pronunciation>ˈni.ɸu.tu</pronunciation>
                    <translation>sun</translation>
                    <part-of-speech>Noun</part-of-speech>
                </word>
                <word>
                    <romanization>xahlauraqi</romanization>
                    <pronunciation>ˈxa.ɬa.u.ɹa.qi</pronunciation>
                    <translation>story</translation>
                    <part-of-speech>Noun</part-of-speech>
                </word>
                <word>
                    <romanization>pfunutsaaxi</romanization>
                    <pronunciation>ˈpɸu.nu.tsaː.xi</pronunciation>
                    <translation>flow</translation>
                    <part-of-speech>Verb</part-of-speech>
                </word>
            </lexicon>
            "#;

        assert_eq!(test_lex(), Lexicon::load_xml_str(xml).unwrap());
    }

    #[test]
    fn write_xml() {
        let lex = test_lex();
        let xml = lex.save_xml_string().unwrap();
        let lex2 = Lexicon::load_xml_str(&xml).unwrap();
        assert_eq!(lex, lex2);
    }
}
