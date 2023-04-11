//! Module for lexicon and related data structures.

pub use error::Error;
pub use pos::{PartOfSpeech, ALL_PARTS_OF_SPEECH};
use uuid::Uuid;
pub use word::Word;

use std::{
    collections::{
        hash_map::{Iter, Keys},
        HashMap,
    },
    io::{BufRead, Write},
};

use crate::xml::{ReadXml, WriteXml, XmlError, XmlReader, XmlReaderProcess, XmlWriter};

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
    pub fn add_word(&mut self, word: Word) -> Uuid {
        let id = Uuid::new_v4();
        self.words.insert(id, word);
        id
    }

    /// Removes a word of id `id` from lexicon. Returns `true` if removal is successful.
    pub fn delete_word_by_id(&mut self, id: Uuid) -> bool {
        self.words.remove(&id).is_some()
    }

    /// Gets the number of words.
    pub fn num_words(&self) -> usize {
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
    pub fn word_by_id(&self, id: &Uuid) -> Option<&Word> {
        self.words.get(id)
    }

    /// Gets a mutable reference to word by id.
    pub fn word_by_id_mut(&mut self, id: &Uuid) -> Option<&mut Word> {
        self.words.get_mut(id)
    }

    pub fn words_iter(&self) -> Iter<Uuid, Word> {
        self.words.iter()
    }

    /// Iterates over words ids.
    pub fn ids(&self) -> Keys<Uuid, Word> {
        self.words.keys()
    }
}

impl ReadXml for Lexicon {
    type Error = Error;

    fn read_xml<R: BufRead>(reader: R) -> Result<(Self, R), XmlError<Self::Error>> {
        let mut xml_reader = XmlReader::new(reader, XmlReaderProcessor::new());
        let ret = xml_reader.read()?;
        Ok((ret, xml_reader.finish()))
    }
}

struct XmlReaderProcessor {
    current_id: Uuid,
}

impl XmlReaderProcessor {
    pub fn new() -> Self {
        Self {
            current_id: Uuid::default(),
        }
    }
}

impl XmlReaderProcess for XmlReaderProcessor {
    type Output = Lexicon;
    type Error = Error;

    fn process_tag_start(
        &mut self,
        mut data: Self::Output,
        context: &[String],
        _name: &str,
        attrs: Vec<(&str, String)>,
    ) -> Result<Self::Output, Self::Error> {
        let l = context.len();
        let tag = context.last().map(|s| s.as_str());
        let ptag = match l {
            2.. => context.get(l - 2).map(|s| s.as_str()),
            _ => None,
        };

        let word = data
            .words
            .get_mut(&self.current_id)
            .ok_or(Error::WriteInvalidWord);

        match (ptag, tag) {
            // Root tag;
            (None, Some("lexicon")) => {}
            // Insert new word
            (Some("lexicon"), Some("word")) => {
                // If there's no id attribute, generate a random one for this word.
                self.current_id = attrs
                    .iter()
                    .find(|&x| x.0 == "id")
                    .map(|x| Uuid::parse_str(&x.1))
                    .unwrap_or_else(|| Ok(Uuid::new_v4()))?;
                data.words.insert(self.current_id, Word::new());
            }
            // Clear word properties
            (Some("word"), Some("romanization")) => {
                word?.romanization.clear();
            }
            (Some("word"), Some("pronunciation")) => {
                let word = word?;
                word.xsampa_pronunciation = attrs
                    .iter()
                    .find(|&x| x.0 == "xsampa")
                    .map(|x| x.1.to_owned());
                word.pronunciation.clear();
            }
            (Some("word"), Some("translation")) => {
                word?.translation.clear();
            }
            (Some("word"), Some("part-of-speech")) => {
                word?.part_of_speech = None;
            }
            _ => {
                return Err(Error::WrongContext {
                    ptag: ptag.unwrap_or_default().to_string(),
                    tag: tag.unwrap_or_default().to_string(),
                })
            }
        }
        Ok(data)
    }

    fn process_text(
        &mut self,
        mut data: Self::Output,
        context: &[String],
        text: std::borrow::Cow<str>,
    ) -> Result<Self::Output, Self::Error> {
        let tag = context.last().map(|s| s.as_str());

        let word = data
            .words
            .get_mut(&self.current_id)
            .ok_or(Error::WriteInvalidWord);

        match tag {
            // Set word properties
            Some("romanization") => word?.romanization += &text,
            Some("pronunciation") => word?.pronunciation += &text,
            Some("translation") => word?.translation += &text,
            Some("part-of-speech") => {
                word?.part_of_speech = Some(text.as_ref().into());
            }
            _ => {}
        }
        Ok(data)
    }
}

impl WriteXml for Lexicon {
    type Error = Error;

    fn write_xml<W: Write>(&self, writer: W) -> Result<W, XmlError<Self::Error>> {
        let mut w = XmlWriter::new(writer);

        w.write_init()?;
        w.write_tag_start("lexicon")?;

        for (id, word) in self.words.iter() {
            w.write_tag_start_with_attributes("word", [("id", id.to_string().as_str())])?;

            w.write_tag_start("romanization")?;
            w.write_text(&word.romanization)?;
            w.write_tag_end("romanization")?;

            if let Some(xs) = &word.xsampa_pronunciation {
                w.write_tag_start_with_attributes("pronunciation", [("xsampa", xs.as_str())])?;
            } else {
                w.write_tag_start("pronunciation")?;
            }
            w.write_text(&word.pronunciation)?;
            w.write_tag_end("pronunciation")?;

            w.write_tag_start("translation")?;
            w.write_text(&word.translation)?;
            w.write_tag_end("translation")?;

            if let Some(pos) = &word.part_of_speech {
                w.write_tag_start("part-of-speech")?;
                w.write_text(pos.name())?;
                w.write_tag_end("part-of-speech")?;
            }

            w.write_tag_end("word")?;
        }

        w.write_tag_end("lexicon")?;

        Ok(w.finish())
    }
}
