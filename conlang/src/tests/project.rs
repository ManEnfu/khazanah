use std::io::{BufReader, Cursor};

use crate::{
    lexicon::{Lexicon, PartOfSpeech, Word},
    project::Project,
    Meta,
};

fn test_proj() -> Project {
    let meta = Meta {
        name: "Test Language".to_owned(),
        local_lang: "English".to_owned(),
        author: "ManEnfu".to_owned(),
        description: "This is a language.".to_owned(),
    };

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

    Project {
        file_path: None,
        meta,
        lexicon: lex,
    }
}

#[test]
fn write_and_read() {
    let proj = test_proj();
    let buf = proj
        .save(Cursor::new(Vec::<u8>::new()))
        .unwrap()
        .into_inner();

    let proj2 = Project::load(Cursor::new(buf)).unwrap();

    assert_eq!(proj, proj2);
}

mod meta {
    use crate::Meta;

    fn test_meta() -> Meta {
        Meta {
            name: "Test Language".to_owned(),
            local_lang: "English".to_owned(),
            author: "ManEnfu".to_owned(),
            description: "This is a language.".to_owned(),
        }
    }

    #[test]
    fn read_xml() {
        let xml = r#"
            <?xml version="1.0" encoding="UTF8"?>
            <project>
                <name>Test Language</name>
                <local-lang>English</local-lang>
                <author>ManEnfu</author>
                <description>This is a language.</description>
            </project>
        "#;

        assert_eq!(test_meta(), Meta::load_xml_str(xml).unwrap());
    }

    #[test]
    fn write_xml() {
        let meta = test_meta();
        let xml = meta.save_xml_string().unwrap();
        let meta2 = Meta::load_xml_str(&xml).unwrap();
        assert_eq!(meta, meta2);
    }
}
