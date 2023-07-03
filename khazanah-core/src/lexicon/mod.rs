//! Module for lexicon and related data structures.

pub use error::Error;
pub use pos::{PartOfSpeech, ALL_PARTS_OF_SPEECH};
use uuid::Uuid;
pub use word::{Word, WordBuilder};

use std::{
    collections::{
        hash_map::{Iter, Keys},
        HashMap,
    },
    io::{BufRead, Write},
};

use crate::xml::{ReadXml, WriteXml, XmlError, XmlReader, XmlWriter};

mod error;
mod pos;
mod word;

/// A lexicon. Stores dictionary of words.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Lexicon {
    words: HashMap<Uuid, Word>,
}

impl Lexicon {
    /// Creates a new `Lexicon`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a `Word` to `Lexicon` and returns its id.
    pub fn add_word(&mut self, mut word: Word) -> Uuid {
        let id = if let Some(id) = word.id() {
            id
        } else {
            word.generate_id()
        };
        self.words.insert(id, word);
        id
    }

    /// Removes a word of id `id` from lexicon. Returns `true` if removal is successful.
    pub fn delete_word_by_id(&mut self, id: Uuid) -> bool {
        self.words.remove(&id).is_some()
    }

    /// Gets the number of words.
    pub fn n_words(&self) -> usize {
        self.words.len()
    }

    // /// Gets a reference to word list.
    // pub fn words(&self) -> &[Word] {
    //     &self.words.va
    // }

    // /// Gets a mutable reference to word list.
    // pub fn words_mut(&mut self) -> &mut [Word] {
    //     &mut self.words
    // }

    // /// Gets a reference to word by index.
    // pub fn word_by_index(&self, index: usize) -> Option<&Word> {
    //     self.words.get(index)
    // }

    // /// Gets a mutable reference to word by index.
    // pub fn word_by_index_mut(&mut self, index: usize) -> Option<&mut Word> {
    //     self.words.get_mut(index)
    // }

    /// Gets a reference to word by id.
    pub fn word_by_id(&self, id: Uuid) -> Option<&Word> {
        self.words.get(&id)
    }

    /// Gets a mutable reference to word by id.
    pub fn word_by_id_mut(&mut self, id: Uuid) -> Option<&mut Word> {
        self.words.get_mut(&id)
    }

    /// Iterates over words.
    pub fn iter_words(&self) -> Iter<Uuid, Word> {
        self.words.iter()
    }

    /// Iterates over word ids.
    pub fn ids(&self) -> Keys<Uuid, Word> {
        self.words.keys()
    }
}

impl ReadXml for Lexicon {
    type Error = Error;

    type ReaderState = ();

    const TAG: &'static str = "lexicon";

    fn process_tag_start<R: BufRead>(
        &mut self,
        reader: &mut XmlReader<R>,
        _state: &mut Self::ReaderState,
        name: String,
        attrs: Vec<(String, String)>,
    ) -> Result<(), XmlError<Self::Error>> {
        let l = reader.context.len();
        let tag = reader.context.last().map(|s| s.as_str());
        let ptag = match l {
            2.. => reader.context.get(l - 2).map(|s| s.as_str()),
            _ => None,
        };

        match (ptag, tag) {
            // Root tag;
            (None, Some("lexicon")) => {}
            // Insert new word
            (Some("lexicon"), Some("word")) => {
                let word = Word::deserialize_xml(reader, Some((name, attrs)))?;
                self.add_word(word);
            }
            _ => return Err(XmlError::InvalidTag(tag.unwrap_or_default().to_string())),
        }
        Ok(())
    }

    fn process_text<R: BufRead>(
        &mut self,
        _reader: &mut XmlReader<R>,
        _state: &mut Self::ReaderState,
        _text: String,
    ) -> Result<(), XmlError<Self::Error>> {
        Ok(())
    }

    fn process_tag_end<R: BufRead>(
        &mut self,
        _reader: &mut XmlReader<R>,
        _state: &mut Self::ReaderState,
        _name: String,
    ) -> Result<(), XmlError<Self::Error>> {
        Ok(())
    }
}

impl WriteXml for Lexicon {
    type Error = Error;

    fn serialize_xml<W: Write>(
        &self,
        writer: &mut XmlWriter<W>,
    ) -> Result<(), XmlError<Self::Error>> {
        writer.write_tag_start("lexicon")?;

        for (_, word) in self.words.iter() {
            word.serialize_xml(writer)?;
        }

        writer.write_tag_end("lexicon")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lex() -> Lexicon {
        let mut lex = Lexicon::new();
        lex.add_word(
            WordBuilder::new()
                .romanization("nifutu".to_string())
                .pronunciation("ˈni.ɸu.tu".to_string())
                .translation("sun".to_string())
                .part_of_speech(PartOfSpeech::Noun)
                .build(),
        );
        lex.add_word(
            WordBuilder::new()
                .romanization("xahlauraqi".to_string())
                .pronunciation("ˈxa.ɬa.u.ɹa.qi".to_string())
                .translation("story".to_string())
                .part_of_speech(PartOfSpeech::Noun)
                .build(),
        );
        lex.add_word(
            WordBuilder::new()
                .romanization("pfunutsaaxi".to_string())
                .pronunciation("ˈpɸu.nu.tsaː.xi".to_string())
                .translation("flow".to_string())
                .part_of_speech(PartOfSpeech::Verb)
                .build(),
        );
        lex
    }

    fn test_xml(lex: &Lexicon) -> String {
        let mut xml = r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <lexicon>
            "#
        .to_string();

        for (id, word) in lex.iter_words() {
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
                &word.romanization(),
                &word.pronunciation(),
                &word.translation(),
                word.part_of_speech().as_ref().unwrap().name()
            )
            .as_str();
        }

        xml += r#"
            </lexicon>
        "#;

        xml
    }

    #[test]
    fn read_xml() {
        let lex = test_lex();
        let xml = test_xml(&lex);
        assert_eq!(Lexicon::load_xml_str(&xml).unwrap(), lex);
    }

    #[test]
    fn write_xml() {
        let lex = test_lex();
        let xml = lex.save_xml_string().unwrap();
        assert_eq!(Lexicon::load_xml_str(&xml).unwrap(), lex);
    }
}
