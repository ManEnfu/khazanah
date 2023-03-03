use std::{cell::Cell, rc::Rc};

use conlang::{lexicon::Lexicon, project::Project};

fn main() {
    // let lex = Lexicon::load_xml_file("./lexicon.xml");
    // println!("{:?}", &lex);

    // let lex = lex.unwrap();
    // lex.save_xml_file("./lx2.xml").unwrap();

    // let a = Rc::new(Cell::new(100));
    // let b = a.clone();
    // let v = b.get();
    // b.set(v + 103);
    // println!("{:?}", a.get());

    println!("{:?}", Project::load_file("ep.zip"));
}
