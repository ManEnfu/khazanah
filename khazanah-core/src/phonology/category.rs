use crate::prelude::*;
use rand::{seq::SliceRandom, Rng};
use uuid::Uuid;

use crate::xml::{ReadXml, WriteXml, XmlError, XmlReader, XmlWriter};

use super::Error;

/// A category of phonemes. Used in phonotactics and word generator.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Category {
    /// The id of the category.
    id: Option<Uuid>,
    /// The name of the category.
    name: String,
    /// The id of the phonemes in the category.
    phonemes_id: Vec<Uuid>,
}

impl IdAble for Category {
    /// Gets the id of the category.
    fn id(&self) -> Option<Uuid> {
        self.id
    }

    /// Generates new id for the category, and then returns it.
    fn generate_id(&mut self) -> Uuid {
        let id = Uuid::new_v4();
        self.id = Some(id);
        id
    }
}

impl Category {
    /// Creates a new category.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new category with specified id.
    pub fn new_with_id(id: Uuid) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    /// Gets the name of the category.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of the category.
    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }

    /// Adds a phoneme id into the category.
    pub fn add_phoneme_id(&mut self, id: Uuid) {
        if !self.phonemes_id.contains(&id) {
            self.phonemes_id.push(id);
        }
    }

    /// Removes a phoneme id at index `index` from the category.
    pub fn remove_phoneme_id_by_index(&mut self, index: usize) {
        self.phonemes_id.remove(index);
    }

    /// Removes a phoneme id from the category.
    pub fn remove_phoneme_id(&mut self, id: Uuid) {
        if let Some(index) = self.phonemes_id.iter().position(|v| v == &id) {
            self.remove_phoneme_id_by_index(index);
        }
    }

    /// Gets a reference to phoneme id by index.
    pub fn phoneme_id_by_index(&self, index: usize) -> Option<&Uuid> {
        self.phonemes_id.get(index)
    }

    /// Returns `true` if a phoneme id is in the category.
    pub fn contains_phoneme_id(&self, id: &Uuid) -> bool {
        self.phonemes_id.contains(id)
    }

    /// Iterates over phoneme ids.
    pub fn iter_phoneme_ids(&mut self) -> impl Iterator<Item = &Uuid> {
        self.phonemes_id.iter()
    }

    /// Randomly chooses phoneme ids from the category.
    pub fn choose_phoneme_id<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<&Uuid> {
        self.phonemes_id.choose(rng)
    }
}

impl ReadXml for Category {
    type Error = Error;

    type ReaderState = Option<Uuid>;

    const TAG: &'static str = "category";

    fn process_tag_start<R: std::io::BufRead>(
        &mut self,
        reader: &mut XmlReader<R>,
        state: &mut Self::ReaderState,
        name: String,
        attrs: Vec<(String, String)>,
    ) -> Result<(), XmlError<Self::Error>> {
        match reader.last_tag_pair() {
            (_, Some(Self::TAG)) => {
                let id = attrs
                    .iter()
                    .find(|&x| x.0 == "id")
                    .map(|x| Uuid::parse_str(&x.1))
                    .unwrap_or_else(|| Ok(Uuid::new_v4()))
                    .map_err(|e| XmlError::Other(Error::Id(e)))?;
                self.id = Some(id);
            }
            (Some(Self::TAG), Some("name")) => {
                self.name.clear();
            }
            (Some(Self::TAG), Some("phonemes")) => {
                self.phonemes_id.clear();
            }
            (Some("phonemes"), Some("id")) => {
                *state = None;
            }
            _ => return Err(XmlError::InvalidTag(name)),
        }

        Ok(())
    }

    fn process_text<R: std::io::BufRead>(
        &mut self,
        reader: &mut XmlReader<R>,
        state: &mut Self::ReaderState,
        text: String,
    ) -> Result<(), XmlError<Self::Error>> {
        match reader.last_tag() {
            Some("name") => {
                self.name += &text;
            }
            Some("id") => {
                let id = Uuid::parse_str(&text).map_err(|e| XmlError::Other(Error::Id(e)))?;
                *state = Some(id);
            }
            _ => {}
        }

        Ok(())
    }

    fn process_tag_end<R: std::io::BufRead>(
        &mut self,
        _reader: &mut XmlReader<R>,
        state: &mut Self::ReaderState,
        name: String,
    ) -> Result<(), XmlError<Self::Error>> {
        if name == "id" {
            if let Some(id) = state {
                self.phonemes_id.push(*id);
                *state = None;
            }
        }

        Ok(())
    }
}

impl WriteXml for Category {
    type Error = Error;

    fn serialize_xml<W: std::io::Write>(
        &self,
        writer: &mut XmlWriter<W>,
    ) -> Result<(), XmlError<Self::Error>> {
        if let Some(id) = self.id {
            writer
                .write_tag_start_with_attributes("category", [("id", id.to_string().as_str())])?;
        } else {
            writer.write_tag_start("category")?;
        };

        writer.write_tag_start("name")?;
        writer.write_text(self.name())?;
        writer.write_tag_end("name")?;

        writer.write_tag_start("phonemes")?;
        for pid in self.phonemes_id.iter() {
            writer.write_tag_start("id")?;
            writer.write_text(pid.to_string().as_str())?;
            writer.write_tag_end("id")?;
        }
        writer.write_tag_end("phonemes")?;

        writer.write_tag_end("category")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const XML1: &str = r#"
    <category id="74a61b73-2830-4d23-80d7-fe3222741e80">
        <name>C</name>
        <phonemes>
            <id>fdd685d9-9a96-42b0-856c-fd3b7de584e7</id>
            <id>266bd118-7c61-4822-ad82-b73a3125f9b5</id>
            <id>ae835d0b-b4ce-4686-b16f-d7fbbec55d96</id>
        </phonemes>
    </category>
    "#;

    #[test]
    fn read_xml() {
        let cat = Category::load_xml_str(XML1).unwrap();
        dbg!(&cat);

        assert_eq!(
            cat.id(),
            Some(Uuid::parse_str("74a61b73-2830-4d23-80d7-fe3222741e80").unwrap())
        );
        assert_eq!(cat.name(), "C");

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
        let cat = Category::load_xml_str(XML1).unwrap();
        let xml2 = cat.save_xml_string().unwrap();
        let cat2 = Category::load_xml_str(&xml2).unwrap();
        assert_eq!(&cat, &cat2);
    }
}
