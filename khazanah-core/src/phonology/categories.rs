use crate::prelude::*;
use crate::Store;

use uuid::Uuid;

use crate::xml::{ReadXml, WriteXml, XmlError, XmlReader, XmlWriter};

use super::{Category, Error};

/// Collections of categories
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Categories {
    inner: Store<Category>,
}

impl Categories {
    /// Creates a new collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a category.
    pub fn add_category(&mut self, category: Category) -> Uuid {
        self.inner.add(category)
    }

    /// Removes a category by id.
    pub fn remove_category_by_id(&mut self, id: Uuid) -> Option<Category> {
        self.inner.remove(id)
    }

    /// Removes a category by name.
    pub fn remove_category_by_name(&mut self, name: &str) -> Option<Category> {
        let id = self
            .inner
            .iter()
            .find_map(|cat| if cat.name() == name { cat.id() } else { None });

        if let Some(id) = id {
            self.inner.remove(id)
        } else {
            None
        }
    }

    /// Gets a reference to category by id.
    pub fn category_by_id(&self, id: Uuid) -> Option<&Category> {
        self.inner.get(id)
    }

    /// Gets a reference to category by name.
    pub fn category_by_name(&self, name: &str) -> Option<&Category> {
        let id = self
            .inner
            .iter()
            .find_map(|cat| if cat.name() == name { cat.id() } else { None });

        if let Some(id) = id {
            self.inner.get(id)
        } else {
            None
        }
    }

    /// Iterates over categories.
    pub fn iter_categories(&self) -> impl Iterator<Item = &Category> {
        self.inner.iter()
    }

    /// Iterates over category ids.
    pub fn ids(&self) -> impl Iterator<Item = &Uuid> {
        self.inner.ids()
    }
}

impl ReadXml for Categories {
    type Error = Error;

    type ReaderState = ();

    const TAG: &'static str = "categories";

    fn process_tag_start<R: std::io::BufRead>(
        &mut self,
        reader: &mut XmlReader<R>,
        state: &mut Self::ReaderState,
        name: String,
        attrs: Vec<(String, String)>,
    ) -> Result<(), XmlError<Self::Error>> {
        self.inner
            ._process_tag_start(Self::TAG, reader, state, name, attrs)
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

impl WriteXml for Categories {
    type Error = Error;

    fn serialize_xml<W: std::io::Write>(
        &self,
        writer: &mut XmlWriter<W>,
    ) -> Result<(), XmlError<Self::Error>> {
        self.inner._serialize_xml("categories", writer)
    }
}
