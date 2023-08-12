use super::{Error, Word};
use crate::prelude::*;
use crate::Store;
use uuid::Uuid;

use std::io::{BufRead, Write};

use crate::xml::{XmlError, XmlReader, XmlWriter};

/// A lexicon. Stores dictionary of words.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Dictionary {
    words: Store<Word>,
}

impl Dictionary {
    /// Creates a new `Lexicon`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a `Word` to `Lexicon` and returns its id.
    pub fn add_word(&mut self, word: Word) -> Uuid {
        self.words.add(word)
    }

    /// Removes a word of id `id` from lexicon.
    pub fn remove_word_by_id(&mut self, id: Uuid) -> Option<Word> {
        self.words.remove(id)
    }

    /// Gets the number of words.
    pub fn n_words(&self) -> usize {
        self.words.len()
    }

    /// Gets a reference to word by id.
    pub fn word_by_id(&self, id: Uuid) -> Option<&Word> {
        self.words.get(id)
    }

    /// Gets a mutable reference to word by id.
    pub fn word_by_id_mut(&mut self, id: Uuid) -> Option<&mut Word> {
        self.words.get_mut(id)
    }

    /// Iterates over words.
    pub fn iter_words(&self) -> impl Iterator<Item = &Word> {
        self.words.iter()
    }

    /// Iterates over words.
    pub fn iter_words_mut(&mut self) -> impl Iterator<Item = &mut Word> {
        self.words.iter_mut()
    }

    /// Iterates over word ids.
    pub fn ids(&self) -> impl Iterator<Item = &Uuid> {
        self.words.ids()
    }

    /// Gets a reference to the inner store.
    pub fn words(&self) -> &Store<Word> {
        &self.words
    }
}

impl ReadXml for Dictionary {
    type Error = Error;

    type ReaderState = ();

    const TAG: &'static str = "dictionary";

    fn process_tag_start<R: BufRead>(
        &mut self,
        reader: &mut XmlReader<R>,
        state: &mut Self::ReaderState,
        name: String,
        attrs: Vec<(String, String)>,
    ) -> Result<(), XmlError<Self::Error>> {
        self.words
            ._process_tag_start(Self::TAG, reader, state, name, attrs)
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

impl WriteXml for Dictionary {
    type Error = Error;

    fn serialize_xml<W: Write>(
        &self,
        writer: &mut XmlWriter<W>,
    ) -> Result<(), XmlError<Self::Error>> {
        self.words._serialize_xml("dictionary", writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexicon::{PartOfSpeech, WordBuilder};

    fn test_lex() -> Dictionary {
        let mut lex = Dictionary::new();
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

    fn test_xml(lex: &Dictionary) -> String {
        let mut xml = r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <dictionary>
            "#
        .to_string();

        for word in lex.iter_words() {
            xml += format!(
                r#"
                <word id="{}">
                    <romanization>{}</romanization>
                    <pronunciation>{}</pronunciation>
                    <translation>{}</translation>
                    <part-of-speech>{}</part-of-speech>
                </word>
                "#,
                word.id().unwrap().to_string(),
                &word.romanization(),
                &word.pronunciation(),
                &word.translation(),
                word.part_of_speech().as_ref().unwrap().name()
            )
            .as_str();
        }

        xml += r#"
            </dictionary>
        "#;

        xml
    }

    #[test]
    fn read_xml() {
        let lex = test_lex();
        let xml = test_xml(&lex);
        assert_eq!(Dictionary::load_xml_str(&xml).unwrap(), lex);
    }

    #[test]
    fn write_xml() {
        let lex = test_lex();
        let xml = lex.save_xml_string().unwrap();
        assert_eq!(Dictionary::load_xml_str(&xml).unwrap(), lex);
    }
}
