use std::{cell::Cell, rc::Rc};

use conlang::{lexicon::Lexicon, project::Project, Word, PartOfSpeech};

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

fn main() {
    // let lex = Lexicon::load_xml_file("./lexicon.xml");
    let lex = test_lex();
    println!("{:?}", &lex);
    let s = lex.save_xml_string().unwrap();
    println!("{}", &s);

    let lex2 = Lexicon::load_xml_str(&s).unwrap();
    println!("{:?}", &lex2);

    // let lex = lex.unwrap();
    // lex.save_xml_file("./lx2.xml").unwrap();

    // let a = Rc::new(Cell::new(100));
    // let b = a.clone();
    // let v = b.get();
    // b.set(v + 103);
    // println!("{:?}", a.get());
}
