use crate::phonology::{Categories, Category};
use crate::xml::{ReadXml, WriteXml, XmlError, XmlReader, XmlWriter};
use crate::{phonology::Inventory, Dictionary};
use crate::{Phoneme, Word};

pub use error::Error;
pub use meta::Meta;
use uuid::Uuid;

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

pub struct LanguageStores<'a> {
    pub phonemic_inventory: &'a Inventory,
    pub phoneme_categories: &'a Categories,
    pub dictionary: &'a Dictionary,
}

pub struct LanguageStoresMut<'a> {
    pub phonemic_inventory: &'a mut Inventory,
    pub phoneme_categories: &'a mut Categories,
    pub dictionary: &'a mut Dictionary,
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

    /// PHONEMIC INVENTORY

    /// Gets a reference to phonemic inventory store.
    pub fn phonemic_inventory(&self) -> &Inventory {
        &self.phonemic_inventory
    }

    /// Gets a reference to phonemic inventory store.
    pub fn phonemic_inventory_mut(&mut self) -> &mut Inventory {
        self.phonemic_inventory.is_inner = true;
        &mut self.phonemic_inventory
    }

    /// Removes a phoneme of id `id` from the inventory.
    /// If `cascade` is `true`, any reference to the phoneme is also removed.
    /// If `cascade` is `false`, the operation fails if any reference to the phoneme exists.
    pub fn phonemic_inventory_remove_phoneme_by_id(
        &mut self,
        id: Uuid,
        cascade: bool,
    ) -> Option<Phoneme> {
        self.phonemic_inventory.remove_phoneme_by_id_joined(
            id,
            cascade,
            &mut self.phoneme_categories,
        )
    }

    // PHONEME CATEGORIES

    /// Gets a reference to phoneme categories store.
    pub fn phoneme_categories(&self) -> &Categories {
        &self.phoneme_categories
    }

    /// Gets a mutable reference to phoneme categories store.
    pub fn phoneme_categories_mut(&mut self) -> &mut Categories {
        &mut self.phoneme_categories
    }

    /// Removes a category by id.
    pub fn phoneme_categories_remove_category_by_id(&mut self, id: Uuid) -> Option<Category> {
        self.phoneme_categories.remove_category_by_id(id)
    }

    /// Removes a category by name.
    pub fn phoneme_categories_remove_category_by_name(&mut self, name: &str) -> Option<Category> {
        self.phoneme_categories.remove_category_by_name(name)
    }

    // DICTIONARY

    /// Gets a reference to dictionary store.
    pub fn dictionary(&self) -> &Dictionary {
        &self.dictionary
    }

    /// Gets a reference to dictionary store.
    pub fn dictionary_mut(&mut self) -> &mut Dictionary {
        &mut self.dictionary
    }

    /// Removes a word of id `id` from lexicon. Returns `true` if removal is successful.
    /// If `cascade` is `true`, any reference to the word is also removed.
    /// If `cascade` is `false`, the operation fails if any reference to the word exists.
    pub fn dictionary_remove_word_by_id(&mut self, id: Uuid, _cascade: bool) -> Option<Word> {
        self.dictionary.remove_word_by_id(id)
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
