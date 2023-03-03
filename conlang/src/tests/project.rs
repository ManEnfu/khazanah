use std::io::{Cursor, BufReader};

use crate::{project::Project, lexicon::{Word, Lexicon, PartOfSpeech}};

fn test_proj() -> Project {
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
        lexicon: lex
    }
}

#[test]
fn write_and_read() {
    // let mut writer = Cur 

    let proj = test_proj();
    let buf = proj.save(Cursor::new(Vec::<u8>::new()))
        .unwrap()
        .into_inner();

    let proj2 = Project::load(Cursor::new(buf))
        .unwrap();

    assert_eq!(proj, proj2);
}
