use std::collections::{hash_map::Keys, HashMap};

use bimap::BiHashMap;
use uuid::Uuid;

use crate::xml::{ReadXml, WriteXml, XmlError, XmlReader, XmlWriter};
use crate::{utils, Phoneme};

use super::Error;

/// An inventory of phonemes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Inventory {
    phonemes: HashMap<Uuid, Phoneme>,

    romanization_pronunciation_map: Option<BiHashMap<String, String>>,
}

impl Inventory {
    /// Creates a new inventory.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a phoneme into the inventory and returns its id.
    pub fn add_phoneme(&mut self, mut phoneme: Phoneme) -> Uuid {
        self.romanization_pronunciation_map = None;
        let id = if let Some(id) = phoneme.id() {
            id
        } else {
            phoneme.generate_id()
        };
        self.phonemes.insert(id, phoneme);
        id
    }

    /// Removes a phoneme of id `id` from the inventory.
    pub fn delete_phoneme_by_id(&mut self, id: Uuid) -> Option<Phoneme> {
        self.romanization_pronunciation_map = None;
        self.phonemes.remove(&id)
    }

    /// Gets the number of phonemes.
    pub fn n_phonemes(&self) -> usize {
        self.phonemes.len()
    }

    /// Gets a reference to phoneme by id.
    pub fn phoneme_by_id(&self, id: Uuid) -> Option<&Phoneme> {
        self.phonemes.get(&id)
    }

    /// Gets a mutable reference to phoneme by id.
    pub fn phoneme_by_id_mut(&mut self, id: Uuid) -> Option<&mut Phoneme> {
        // Assume that the romanization map is outdated.
        self.romanization_pronunciation_map = None;
        self.phonemes.get_mut(&id)
    }

    /// Iterates over phonemes.
    pub fn iter_phonemes(&self) -> impl Iterator<Item = &Phoneme> {
        self.phonemes.values()
    }

    /// Iterates over mutable reference of phonemes.
    pub fn iter_phonemes_mut(&mut self) -> impl Iterator<Item = &mut Phoneme> {
        self.phonemes.values_mut()
    }

    /// Iterates over phoneme ids.
    pub fn ids(&self) -> Keys<Uuid, Phoneme> {
        self.phonemes.keys()
    }

    /// Converts a romanization to IPA pronunciation using rules specified by the inventory.
    pub fn pronunce_romanization(&mut self, romanization: &str) -> String {
        if self.romanization_pronunciation_map.is_none() {
            self.populate_romanization_pronunciation_map();
        }
        if let Some(map) = &self.romanization_pronunciation_map {
            String::from_iter(utils::transliterate(romanization, 5, |s| {
                map.get_by_left(s).cloned()
            }))
        } else {
            String::default()
        }
    }

    /// Converts an IPA pronunciation to romanization using rules specified by the inventory.
    pub fn get_romanization(&mut self, pronunciation: &str) -> String {
        if self.romanization_pronunciation_map.is_none() {
            self.populate_romanization_pronunciation_map();
        }
        if let Some(map) = &self.romanization_pronunciation_map {
            String::from_iter(utils::transliterate(pronunciation, 5, |s| {
                map.get_by_right(s).cloned()
            }))
        } else {
            String::default()
        }
    }

    fn populate_romanization_pronunciation_map(&mut self) {
        let mut map = BiHashMap::new();
        for phoneme in self.iter_phonemes() {
            map.insert(
                phoneme
                    .romanization()
                    .unwrap_or(phoneme.sound())
                    .to_string(),
                phoneme.sound().to_string(),
            );
        }
        self.romanization_pronunciation_map = Some(map);
    }
}

impl ReadXml for Inventory {
    type Error = Error;

    type ReaderState = ();

    const TAG: &'static str = "inventory";

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
            (_, "inventory") => {}
            (Some("inventory"), "phoneme") => {
                let phoneme = Phoneme::deserialize_xml(reader, Some((name, attrs)))?;
                self.add_phoneme(phoneme);
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

impl WriteXml for Inventory {
    type Error = Error;

    fn serialize_xml<W: std::io::Write>(
        &self,
        writer: &mut XmlWriter<W>,
    ) -> Result<(), XmlError<Self::Error>> {
        writer.write_tag_start("inventory")?;

        for (_, phoneme) in self.phonemes.iter() {
            phoneme.serialize_xml(writer)?;
        }

        writer.write_tag_end("inventory")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const XML1: &str = r#"
    <inventory>
      <phoneme id="74a61b73-2830-4d23-80d7-fe3222741e80">
        <sound>t</sound>
      </phoneme>
      <phoneme id="bc629cc3-99be-44f0-a2c4-d51dce960f2c">
        <sound>h</sound>
      </phoneme>
      <phoneme id="07d1d0ce-3f14-4708-bcda-14b2413fbe8e">
        <sound>ɹ</sound>
        <romanization>r</romanization>
      </phoneme>
      <phoneme id="ae835d0b-b4ce-4686-b16f-d7fbbec55d96">
        <sound>s</sound>
      </phoneme>
      <phoneme id="fdd685d9-9a96-42b0-856c-fd3b7de584e7">
        <sound>m</sound>
      </phoneme>
      <phoneme id="5ce2f1b7-527f-4779-9c96-71939cb397af">
        <sound>t͡s</sound>
        <romanization>ts</romanization>
      </phoneme>
      <phoneme id="266bd118-7c61-4822-ad82-b73a3125f9b5">
        <sound>tʰ</sound>
        <romanization>th</romanization>
      </phoneme>

      <phoneme id="4529d630-8d85-4cfb-a81f-e53c4cb1e3dd">
        <sound>a</sound>
      </phoneme>
      <phoneme id="02cee97e-37ac-4f72-ba6d-b297027a8977">
        <sound>aː</sound>
        <romanization>aa</romanization>
      </phoneme>
      <phoneme id="7e89facb-6e4e-4319-8dba-447cef2bc665">
        <sound>ɪ</sound>
        <romanization>i</romanization>
      </phoneme>
      <phoneme id="57beed82-6a64-427d-b99b-8be53e548bd7">
        <sound>u</sound>
      </phoneme>
      <phoneme id="bb3d4b74-1260-4a5e-95f6-7d344c9516dc">
        <sound>e</sound>
      </phoneme>
      <phoneme id="46fab438-136d-4982-90a4-f8ea6ebb0c1f">
        <sound>ə</sound>
        <romanization>ë</romanization>
      </phoneme>
      <phoneme id="70583203-66ab-4b94-ae52-786d83374406">
        <sound>o</sound>
      </phoneme>
    </inventory>
    "#;

    #[test]
    fn read_xml() {
        let inv = Inventory::load_xml_str(XML1).unwrap();

        let p = inv
            .phoneme_by_id(Uuid::parse_str("5ce2f1b7-527f-4779-9c96-71939cb397af").unwrap())
            .unwrap();
        assert_eq!(p.romanization(), Some("ts"));
        assert_eq!(p.sound(), "t͡s");

        let p = inv
            .phoneme_by_id(Uuid::parse_str("7e89facb-6e4e-4319-8dba-447cef2bc665").unwrap())
            .unwrap();
        assert_eq!(p.romanization(), Some("i"));
        assert_eq!(p.sound(), "ɪ");

        let p = inv
            .phoneme_by_id(Uuid::parse_str("02cee97e-37ac-4f72-ba6d-b297027a8977").unwrap())
            .unwrap();
        assert_eq!(p.romanization(), Some("aa"));
        assert_eq!(p.sound(), "aː");
    }

    #[test]
    fn write_xml() {
        let inv = Inventory::load_xml_str(XML1).unwrap();
        let xml2 = inv.save_xml_string().unwrap();
        let inv2 = Inventory::load_xml_str(&xml2).unwrap();
        assert_eq!(&inv, &inv2)
    }

    #[test]
    fn romanization_and_pronunciation() {
        let mut inv = Inventory::load_xml_str(XML1).unwrap();
        let romanization = "thëmaartsim".to_string();
        let pronunciation = "ˈtʰə.maːɹ.t͡sɪm".to_string();
        let pronunciation_no_delimiter = "tʰəmaːɹt͡sɪm".to_string();
        let _a = inv.get_romanization(&pronunciation);
        dbg!(&inv);
        assert_eq!(inv.get_romanization(&pronunciation), romanization);
        assert_eq!(
            inv.pronunce_romanization(&romanization),
            pronunciation_no_delimiter
        );
    }
}
