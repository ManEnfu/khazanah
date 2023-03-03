//! Module for lexicon and related data structures.

pub use error::Error;
pub use pos::PartOfSpeech;
pub use word::Word;
pub use xml::{XmlError, XmlReader, XmlTag, XmlWriter};

use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor, Write},
    path::Path,
};

pub mod error;
pub mod pos;
pub mod word;
pub mod xml;

/// A lexicon. Stores dictionary of words.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Lexicon {
    words: Vec<Word>,
}

impl Lexicon {
    /// Creates a new `Lexicon`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reads from XML.
    pub fn read_xml<R: BufRead>(reader: R) -> Result<Self, Error> {
        XmlReader::new(reader).read().map_err(|e| e.into())
    }

    /// Load `Lexicon` from XML file.
    pub fn load_xml_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let f = File::open(path)?;
        Self::read_xml(BufReader::new(f))
    }

    /// Load `Lexicon` from XML string.
    pub fn load_xml_str(s: &str) -> Result<Self, Error> {
        Self::read_xml(s.as_bytes())
    }

    /// Writes to XML.
    pub fn write_xml<W: Write>(&self, writer: W) -> Result<W, Error> {
        let mut w = XmlWriter::new(self, writer);
        w.write()?;
        Ok(w.into_inner())
    }

    /// Saves `Lexicon` to XML file.
    pub fn save_xml_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let f = File::create(path)?;
        self.write_xml(f)?;
        Ok(())
    }

    /// Saves `Lexicon` to XML string.
    pub fn save_xml_string(&self) -> Result<String, Error> {
        let mut w = XmlWriter::new(self, Cursor::new(Vec::<u8>::new()));
        w.write()?;
        let ar = w.into_inner().into_inner();
        String::from_utf8(ar).map_err(Error::from)
    }

    /// Adds a `Word` to `Lexicon`.
    pub fn add_word(&mut self, word: Word) {
        self.words.push(word);
    }

    /// Gets a reference to word list.
    pub fn words(&self) -> &[Word] {
        &self.words
    }

    /// Gets a mutable reference to word list.
    pub fn words_mut(&mut self) -> &mut [Word] {
        &mut self.words
    }
}
