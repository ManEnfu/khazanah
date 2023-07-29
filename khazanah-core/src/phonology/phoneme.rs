use std::io::{BufRead, Write};

use uuid::Uuid;

use crate::{
    ipa,
    xml::{ReadXml, WriteXml, XmlError, XmlReader, XmlWriter},
};

use super::Error;

/// A Phoneme.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Phoneme {
    /// The id of the phoneme.
    id: Option<Uuid>,
    /// The IPA sound of the phoneme.
    sound: String,
    /// The romanization of the phoneme.
    romanization: Option<String>,
}

impl Phoneme {
    /// Creates a new phoneme.
    pub fn new() -> Self {
        Self::default()
    }

    // Creates a new phoneme with specified id and modifiers.
    pub fn new_with_id(id: Uuid) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    pub fn from_ipa(ipa: ipa::Ipa) -> Self {
        Self {
            sound: ipa.symbol().unwrap_or_default().to_string(),
            ..Default::default()
        }
    }

    pub fn builder() -> PhonemeBuilder {
        PhonemeBuilder::new()
    }

    /// Gets the id of the phoneme.
    pub fn id(&self) -> Option<Uuid> {
        self.id
    }

    /// Generates new id for the phoneme, and then returns it.
    pub fn generate_id(&mut self) -> Uuid {
        let id = Uuid::new_v4();
        self.id = Some(id);
        id
    }

    /// Gets the sound of the phoneme.
    pub fn sound(&self) -> &str {
        &self.sound
    }

    /// Sets the sound of the phoneme.
    pub fn set_sound(&mut self, value: String) {
        self.sound = value;
    }

    /// Gets the romanization of the phoneme.
    pub fn romanization(&self) -> Option<&str> {
        self.romanization.as_deref()
    }

    /// Sets the romanization of the phoneme.
    pub fn set_romanization(&mut self, value: Option<String>) {
        self.romanization = value;
    }

    /// Gets the base of the phoneme.
    pub fn base(&self) -> Option<ipa::Ipa> {
        let ipas = ipa::parse_str(&self.sound);
        ipas.get(0).copied()
    }
}

impl ReadXml for Phoneme {
    type Error = Error;

    type ReaderState = ();

    const TAG: &'static str = "phoneme";

    fn process_tag_start<R: BufRead>(
        &mut self,
        _reader: &mut XmlReader<R>,
        _state: &mut Self::ReaderState,
        name: String,
        attrs: Vec<(String, String)>,
    ) -> Result<(), XmlError<Self::Error>> {
        match name.as_str() {
            Self::TAG => {
                let id = attrs
                    .iter()
                    .find(|&x| x.0 == "id")
                    .map(|x| Uuid::parse_str(&x.1))
                    .unwrap_or_else(|| Ok(Uuid::new_v4()))
                    .map_err(|e| XmlError::Other(Error::Id(e)))?;
                self.id = Some(id);
            }
            "sound" => self.sound.clear(),
            "romanization" => {
                self.romanization = Some(String::default());
            }
            _ => return Err(XmlError::InvalidTag(name)),
        }
        Ok(())
    }

    fn process_text<R: BufRead>(
        &mut self,
        reader: &mut XmlReader<R>,
        _state: &mut Self::ReaderState,
        text: String,
    ) -> Result<(), XmlError<Self::Error>> {
        let tag = reader.context.last().map(|s| s.as_str());

        match tag {
            Some("sound") => {
                self.sound += &text;
            }
            Some("romanization") => {
                if let Some(rom) = self.romanization.as_mut() {
                    *rom += &text;
                }
            }
            _ => {
                return Err(XmlError::InvalidTag(tag.unwrap_or_default().to_string()));
            }
        }

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

impl WriteXml for Phoneme {
    type Error = Error;

    fn serialize_xml<W: Write>(&self, w: &mut XmlWriter<W>) -> Result<(), XmlError<Self::Error>> {
        w.write_tag_start_with_attributes(
            "phoneme",
            [("id", self.id.unwrap_or_default().to_string().as_str())],
        )?;

        w.write_tag_start("sound")?;
        w.write_text(&self.sound)?;
        w.write_tag_end("sound")?;

        if let Some(rom) = self.romanization.as_ref() {
            w.write_tag_start("romanization")?;
            w.write_text(rom)?;
            w.write_tag_end("romanization")?;
        }

        w.write_tag_end("phoneme")?;

        Ok(())
    }
}

/// A builder struct of a phoneme.
#[derive(Debug, Default)]
pub struct PhonemeBuilder {
    inner: Phoneme,
}

impl PhonemeBuilder {
    pub fn new() -> Self {
        Self {
            inner: Phoneme::default(),
        }
    }

    pub fn sound(mut self, value: String) -> Self {
        self.inner.set_sound(value);
        self
    }

    pub fn romanization(mut self, value: String) -> Self {
        self.inner.set_romanization(Some(value));
        self
    }

    pub fn build(self) -> Phoneme {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_phoneme() -> Phoneme {
        let mut ret = PhonemeBuilder::new()
            .sound("aː".to_string())
            .romanization("aa".to_string())
            .build();
        ret.generate_id();
        ret
    }

    fn test_xml(id: Uuid) -> String {
        format!(
            r#"
            <phoneme id="{id}">
                <sound>aː</sound>
                <romanization>aa</romanization>
            </phoneme>
            "#,
        )
    }

    #[test]
    fn read_xml() {
        let phoneme = test_phoneme();
        let xml = test_xml(phoneme.id().unwrap());
        assert_eq!(Phoneme::load_xml_str(&xml).unwrap(), phoneme);
    }

    #[test]
    fn write_xml() {
        let phoneme = test_phoneme();
        let xml = phoneme.save_xml_string().unwrap();
        assert_eq!(Phoneme::load_xml_str(&xml).unwrap(), phoneme);
    }
}
