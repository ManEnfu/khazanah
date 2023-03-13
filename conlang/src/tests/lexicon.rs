use crate::{Lexicon, PartOfSpeech, Word};

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
    let lex = test_lex();
    let mut xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <lexicon>
        "#
    .to_string();

    for (id, word) in lex.words_iter() {
        xml += format!(
            r#"
            <word id="{}">
                <romanization>{}</romanization>
                <pronunciation>{}</pronunciation>
                <translation>{}</translation>
                <part-of-speech>{}</part-of-speech>
            </word>
            "#,
            id.to_string(),
            &word.romanization,
            &word.pronunciation,
            &word.translation,
            word.part_of_speech.as_ref().unwrap().name()
        )
        .as_str();
    }

    xml += r#"
        </lexicon>
    "#;

    assert_eq!(lex, Lexicon::load_xml_str(&xml).unwrap());
}

#[test]
fn write_xml() {
    let lex = test_lex();
    let xml = lex.save_xml_string().unwrap();
    let lex2 = Lexicon::load_xml_str(&xml).unwrap();
    assert_eq!(lex, lex2);
}
