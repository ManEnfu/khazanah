use crate::xml::{ReadXml, WriteXml, XmlError, XmlReader, XmlWriter};

use super::{Category, Error};

/// Collections of categories
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Categories {
    inner: Vec<Category>,
}

impl Categories {
    /// Creates a new collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a category.
    pub fn add_category(&mut self, category: Category) {
        self.inner.push(category);
    }

    /// Removes a category at index `index`
    pub fn remove_category_by_index(&mut self, index: usize) -> Category {
        self.inner.remove(index)
    }

    /// Removes a category by name.
    pub fn remove_category_by_name(&mut self, name: &str) -> Option<Category> {
        if let Some(index) = self.inner.iter().position(|v| v.name() == name) {
            Some(self.inner.remove(index))
        } else {
            None
        }
    }

    /// Gets a reference to category by index.
    pub fn category_by_index(&self, index: usize) -> Option<&Category> {
        self.inner.get(index)
    }

    /// Gets a reference to category by name.
    pub fn category_by_name(&self, name: &str) -> Option<&Category> {
        if let Some(index) = self.inner.iter().position(|v| v.name() == name) {
            self.inner.get(index)
        } else {
            None
        }
    }

    /// Iterates over categories.
    pub fn iter_categories(&self) -> impl Iterator<Item = &Category> {
        self.inner.iter()
    }
}

impl ReadXml for Categories {
    type Error = Error;

    type ReaderState = ();

    const TAG: &'static str = "categories";

    fn process_tag_start<R: std::io::BufRead>(
        &mut self,
        reader: &mut XmlReader<R>,
        _state: &mut Self::ReaderState,
        name: String,
        attrs: Vec<(String, String)>,
    ) -> Result<(), XmlError<Self::Error>> {
        match reader.last_tag_pair() {
            (_, Some(Self::TAG)) => {}
            (Some(Self::TAG), Some(Category::TAG)) => {
                let cat = Category::deserialize_xml(reader, Some((name, attrs)))?;
                self.inner.push(cat);
            }
            _ => {
                return Err(XmlError::InvalidTag(name));
            }
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

impl WriteXml for Categories {
    type Error = Error;

    fn serialize_xml<W: std::io::Write>(
        &self,
        writer: &mut XmlWriter<W>,
    ) -> Result<(), XmlError<Self::Error>> {
        writer.write_tag_start("categories")?;

        for cat in self.inner.iter() {
            cat.serialize_xml(writer)?;
        }

        writer.write_tag_end("categories")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    const XML1: &str = r#"
    <categories>
        <category>
            <name>C</name>
            <phonemes>
                <id>fdd685d9-9a96-42b0-856c-fd3b7de584e7</id>
                <id>266bd118-7c61-4822-ad82-b73a3125f9b5</id>
                <id>ae835d0b-b4ce-4686-b16f-d7fbbec55d96</id>
            </phonemes>
        </category>
    </categories>
    "#;

    #[test]
    fn read_xml() {
        let cats = Categories::load_xml_str(XML1).unwrap();
        dbg!(&cats);

        let cat = cats.category_by_name("C").unwrap();

        assert!(cat.contains_phoneme_id(
            &Uuid::parse_str("ae835d0b-b4ce-4686-b16f-d7fbbec55d96").unwrap()
        ));
        assert_eq!(
            cat.phoneme_id_by_index(1),
            Some(&Uuid::parse_str("266bd118-7c61-4822-ad82-b73a3125f9b5").unwrap())
        );
        assert!(!cat.contains_phoneme_id(
            &Uuid::parse_str("70583203-66ab-4b94-ae52-786d83374406").unwrap()
        ));
    }

    #[test]
    fn write_xml() {
        let cats = Categories::load_xml_str(XML1).unwrap();
        let xml2 = cats.save_xml_string().unwrap();
        let cats2 = Categories::load_xml_str(&xml2).unwrap();
        assert_eq!(&cats, &cats2);
    }
}
