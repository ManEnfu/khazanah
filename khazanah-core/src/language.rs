use crate::phonology::Categories;
use crate::xml::{ReadXml, WriteXml, XmlError, XmlReader, XmlWriter};
use crate::{phonology::Inventory, Dictionary};

pub use error::Error;
pub use meta::Meta;

mod error;
mod meta;

/// A language.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Language {
    meta: Meta,
    phonemic_inventory: Inventory,
    phoneme_categories: Categories,
    dictionary: Dictionary,
}

impl Language {
    /// Creates a new language.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn meta(&self) -> &Meta {
        &self.meta
    }

    pub fn meta_mut(&mut self) -> &mut Meta {
        &mut self.meta
    }

    pub fn phonemic_inventory(&self) -> &Inventory {
        &self.phonemic_inventory
    }

    pub fn phonemic_inventory_mut(&mut self) -> &mut Inventory {
        &mut self.phonemic_inventory
    }

    pub fn phoneme_categories(&self) -> &Categories {
        &self.phoneme_categories
    }

    pub fn phoneme_categories_mut(&mut self) -> &mut Categories {
        &mut self.phoneme_categories
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
        match reader.last_tag_pair() {
            (_, Some(Self::TAG)) => {}
            (Some(Self::TAG), Some(Meta::TAG)) => {
                self.meta = Meta::deserialize_xml(reader, Some((name, attrs)))?;
            }
            (Some(Self::TAG), Some("phonology")) => {}
            (Some("phonology"), Some(Inventory::TAG)) => {
                self.phonemic_inventory = Inventory::deserialize_xml(reader, Some((name, attrs)))
                    .map_err(|xe| xe.map_into())?;
            }
            (Some("phonology"), Some(Categories::TAG)) => {
                self.phoneme_categories = Categories::deserialize_xml(reader, Some((name, attrs)))
                    .map_err(|xe| xe.map_into())?;
            }
            (Some(Self::TAG), Some("lexicon")) => {}
            (Some("lexicon"), Some(Dictionary::TAG)) => {
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

        self.meta.serialize_xml(writer)?;

        writer.write_tag_start("phonology")?;
        self.phonemic_inventory
            .serialize_xml(writer)
            .map_err(|xe| xe.map_into())?;
        self.phoneme_categories
            .serialize_xml(writer)
            .map_err(|xe| xe.map_into())?;
        writer.write_tag_end("phonology")?;

        writer.write_tag_start("lexicon")?;
        self.dictionary
            .serialize_xml(writer)
            .map_err(|xe| xe.map_into())?;
        writer.write_tag_end("lexicon")?;

        writer.write_tag_end("language")?;

        Ok(())
    }
}
