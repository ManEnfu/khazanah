use crate::{Lexicon as Dictionary, phonology::Inventory};
use crate::xml::{ReadXml, WriteXml, XmlError, XmlReader, XmlWriter};
pub use error::Error;

mod error;

/// A language.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Language {
    phonemic_inventory: Inventory,
    dictionary: Dictionary,
}

impl Language {
    /// Creates a new language.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn phonemic_inventory(&self) -> &Inventory {
        &self.phonemic_inventory
    }

    pub fn phonemic_inventory_mut(&mut self) -> &mut Inventory {
        &mut self.phonemic_inventory
    }

    pub fn dictionary(&self) -> &Dictionary {
        &self.dictionary
    }

    pub fn dictionary_mut(&mut self) -> &mut Dictionary {
        &mut self.dictionary
    }
}

impl ReadXml for Language {
    type Error = Error;

    type ReaderState = ();

    const TAG: &'static str = "language";

    fn process_tag_start<R: std::io::BufRead>(
        &mut self,
        reader: &mut XmlReader<R>,
        _state: &mut Self::ReaderState,
        name: String,
        attrs: Vec<(String, String)>,
    ) -> Result<(), XmlError<Self::Error>> {
        let l = reader.context.len();
        let ptag = match l {
            2.. => reader.context.get(l - 2).map(|s| s.as_str()),
            _ => None,
        };

        match (ptag, name.as_str()) {
            (None, "phonology") => {}
            (Some("phonology"), "inventory") => {
                self.phonemic_inventory = Inventory::deserialize_xml(reader, Some((name, attrs)))
                    .map_err(|xe| xe.map_into())?;
            }
            (None, "lexicon") => {}
            (Some("lexicon"), "dictionary") => {
                self.dictionary = Dictionary::deserialize_xml(reader, Some((name, attrs)))
                    .map_err(|xe| xe.map_into())?;
            }
            _ => return Err(XmlError::InvalidTag(name)),
        }

        Ok(())
    }

    fn process_text<R: std::io::BufRead>(
        &mut self,
        _reader: &mut XmlReader<R>,
        _state: &mut Self::ReaderState,
        _text: String,
    ) -> Result<(), XmlError<Self::Error>> {
        Ok(())
    }

    fn process_tag_end<R: std::io::BufRead>(
        &mut self,
        _reader: &mut XmlReader<R>,
        _state: &mut Self::ReaderState,
        _name: String,
    ) -> Result<(), XmlError<Self::Error>> {
        Ok(())
    }
    
}

impl WriteXml for Language {
    type Error = Error;

    fn serialize_xml<W: std::io::Write>(
        &self,
        writer: &mut XmlWriter<W>,
    ) -> Result<(), XmlError<Self::Error>> {
        writer.write_tag_start("language")?;

        writer.write_tag_start("phonology")?;
        self.phonemic_inventory.serialize_xml(writer)
            .map_err(|xe| xe.map_into())?;
        writer.write_tag_end("phonology")?;

        writer.write_tag_start("lexicon")?;
        self.dictionary.serialize_xml(writer)
            .map_err(|xe| xe.map_into())?;
        writer.write_tag_end("lexicon")?;

        writer.write_tag_end("language")?;
        
        Ok(())
    }
}
