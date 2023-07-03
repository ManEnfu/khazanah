use uuid::Uuid;

use crate::{
    ipa,
    xml::{ReadXml, WriteXml, XmlError, XmlWriter},
};

use super::{Error, PartOfSpeech};
use std::{fmt::Debug, io::Write};

/// Word entry in the lexicon.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Word {
    /// The id of the word.
    id: Option<Uuid>,
    /// The romanization of the word.
    romanization: String,
    /// The translation of the word.
    translation: String,
    /// The pronunciation of word in IPA.
    pronunciation: String,
    /// Which part of speech this word belongs to.
    part_of_speech: Option<PartOfSpeech>,
    /// The X-SAMPA pronunciation of the word, if exists.
    xsampa_pronunciation: Option<String>,
}

impl Word {
    /// Creates a new word.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new word with a specific id.
    pub fn new_with_id(id: Uuid) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    /// Gets the id of the word.
    pub fn id(&self) -> Option<Uuid> {
        self.id
    }

    /// Generates new id for the word, and then returns it.
    pub fn generate_id(&mut self) -> Uuid {
        let id = Uuid::new_v4();
        self.id = Some(id);
        id
    }

    /// Gets the romanization of the word.
    pub fn romanization(&self) -> &str {
        self.romanization.as_str()
    }

    /// Sets the romanization of the word.
    pub fn set_romanization(&mut self, value: String) {
        self.romanization = value;
    }

    /// Gets the translation of the word.
    pub fn translation(&self) -> &str {
        self.translation.as_str()
    }

    /// Sets the translation of the word.
    pub fn set_translation(&mut self, value: String) {
        self.translation = value;
    }

    /// Gets the IPA pronunciation of the word.
    pub fn pronunciation(&self) -> &str {
        self.pronunciation.as_str()
    }

    /// Sets the IPA pronunciation of the word.
    pub fn set_pronunciation(&mut self, value: String) {
        self.pronunciation = value;
    }

    /// Gets the X-SAMPA pronunciation of the word.
    pub fn xsampa_pronunciation(&self) -> Option<&str> {
        self.xsampa_pronunciation.as_deref()
    }

    /// Sets the X-SAMPA pronunciation of the word.
    /// The value will be converted to IPA pronunciation and used
    /// to set the pronunciation of the word.
    pub fn set_xsampa_pronunciation(&mut self, s: Option<String>) {
        if let Some(s) = &s {
            self.pronunciation = ipa::transliterate_xsampa(s);
        }
        self.xsampa_pronunciation = s;
    }

    /// Sets the part of speech of the word.
    pub fn part_of_speech(&self) -> Option<PartOfSpeech> {
        self.part_of_speech
    }

    /// Sets the part of speech of the word.
    pub fn set_part_of_speech(&mut self, value: Option<PartOfSpeech>) {
        self.part_of_speech = value;
    }
}

impl ReadXml for Word {
    type Error = Error;

    type ReaderState = ();

    const TAG: &'static str = "word";

    fn process_tag_start<R: std::io::BufRead>(
        &mut self,
        reader: &mut crate::xml::XmlReader<R>,
        _state: &mut Self::ReaderState,
        _name: String,
        attrs: Vec<(String, String)>,
    ) -> Result<(), XmlError<Self::Error>> {
        let tag = reader.context.last().map(|s| s.as_str());

        match tag {
            Some("word") => {
                let id = attrs
                    .iter()
                    .find(|&x| x.0 == "id")
                    .map(|x| Uuid::parse_str(&x.1))
                    .unwrap_or_else(|| Ok(Uuid::new_v4()))
                    .map_err(|e| XmlError::Other(Error::Id(e)))?;
                self.id = Some(id);
            }
            Some("romanization") => {
                self.romanization.clear();
            }
            Some("pronunciation") => {
                self.xsampa_pronunciation = attrs
                    .iter()
                    .find(|&x| x.0 == "xsampa")
                    .map(|x| x.1.to_owned());
                self.pronunciation.clear();
            }
            Some("translation") => {
                self.translation.clear();
            }
            Some("part-of-speech") => {
                self.part_of_speech = None;
            }
            _ => return Err(XmlError::InvalidTag(tag.unwrap_or_default().to_string())),
        }
        Ok(())
    }

    fn process_text<R: std::io::BufRead>(
        &mut self,
        reader: &mut crate::xml::XmlReader<R>,
        _state: &mut Self::ReaderState,
        text: String,
    ) -> Result<(), XmlError<Self::Error>> {
        let tag = reader.context.last().map(|s| s.as_str());

        match tag {
            // Set word properties
            Some("romanization") => self.romanization += &text,
            Some("pronunciation") => self.pronunciation += &text,
            Some("translation") => self.translation += &text,
            Some("part-of-speech") => {
                self.part_of_speech = Some(text.as_str().into());
            }
            _ => return Err(XmlError::InvalidTag(tag.unwrap_or_default().to_string())),
        }
        Ok(())
    }

    fn process_tag_end<R: std::io::BufRead>(
        &mut self,
        _reader: &mut crate::xml::XmlReader<R>,
        _state: &mut Self::ReaderState,
        _name: String,
    ) -> Result<(), XmlError<Self::Error>> {
        Ok(())
    }
}

impl WriteXml for Word {
    type Error = Error;

    fn serialize_xml<W: Write>(&self, w: &mut XmlWriter<W>) -> Result<(), XmlError<Self::Error>> {
        w.write_tag_start_with_attributes(
            "word",
            [("id", self.id.unwrap_or_default().to_string().as_str())],
        )?;

        w.write_tag_start("romanization")?;
        w.write_text(&self.romanization)?;
        w.write_tag_end("romanization")?;

        if let Some(xs) = &self.xsampa_pronunciation {
            w.write_tag_start_with_attributes("pronunciation", [("xsampa", xs.as_str())])?;
        } else {
            w.write_tag_start("pronunciation")?;
        }
        w.write_text(&self.pronunciation)?;
        w.write_tag_end("pronunciation")?;

        w.write_tag_start("translation")?;
        w.write_text(&self.translation)?;
        w.write_tag_end("translation")?;

        if let Some(pos) = &self.part_of_speech {
            w.write_tag_start("part-of-speech")?;
            w.write_text(pos.name())?;
            w.write_tag_end("part-of-speech")?;
        }

        w.write_tag_end("word")?;

        Ok(())
    }
}

/// A builder struct of a word.
#[derive(Debug, Default)]
pub struct WordBuilder {
    inner: Word,
}

impl WordBuilder {
    pub fn new() -> Self {
        Self { inner: Word::new() }
    }

    pub fn romanization(mut self, value: String) -> Self {
        self.inner.set_romanization(value);
        self
    }

    pub fn translation(mut self, value: String) -> Self {
        self.inner.set_translation(value);
        self
    }

    pub fn pronunciation(mut self, value: String) -> Self {
        self.inner.set_pronunciation(value);
        self
    }

    pub fn xsampa_pronunciation(mut self, value: String) -> Self {
        self.inner.set_xsampa_pronunciation(Some(value));
        self
    }

    pub fn part_of_speech(mut self, value: PartOfSpeech) -> Self {
        self.inner.set_part_of_speech(Some(value));
        self
    }

    pub fn build(self) -> Word {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const UUID: Uuid = Uuid::from_fields(
        0x49a1429b,
        0xc82a,
        0x1103,
        &[0x54, 0x82, 0x11, 0xd9, 0x2e, 0x11, 0x32, 0x00],
    );

    const ROMANIZATION: &str = "nishr";
    const TRANSLATION: &str = "sun";
    const IPA: &str = "ˈni.ʃɹ";
    const XSAMPA: &str = "\"ni.Sr\\";
    const XSAMPA_ESC: &str = "&quot;ni.Sr\\";
    const POS: PartOfSpeech = PartOfSpeech::Noun;

    fn test_word() -> Word {
        Word {
            id: Some(UUID),
            romanization: ROMANIZATION.to_string(),
            translation: TRANSLATION.to_string(),
            pronunciation: IPA.to_string(),
            xsampa_pronunciation: Some(XSAMPA.to_string()),
            part_of_speech: Some(POS),
        }
    }

    fn test_xml() -> String {
        format!(
            r#"
            <word id="{}">
                <romanization>{}</romanization>
                <pronunciation xsampa="{}">{}</pronunciation>
                <translation>{}</translation>
                <part-of-speech>{}</part-of-speech>
            </word>
            "#,
            UUID.to_string(),
            ROMANIZATION,
            XSAMPA_ESC,
            IPA,
            TRANSLATION,
            POS.name(),
        )
    }

    #[test]
    fn constructors() {
        let word1 = Word::new();
        assert_eq!(word1.id(), None);

        let id2 = Uuid::new_v4();
        let word2 = Word::new_with_id(id2);
        assert_eq!(word2.id(), Some(id2));
    }

    #[test]
    fn getters_and_setters() {
        let mut word = Word::new();

        let rmz = "nishr";
        word.set_romanization(rmz.to_string());
        assert_eq!(word.romanization(), rmz);

        let tln = "sun";
        word.set_translation(tln.to_string());
        assert_eq!(word.translation(), tln);

        let prn = "ni.shr";
        word.set_pronunciation(prn.to_string());
        assert_eq!(word.pronunciation(), prn);

        let xsp = "\"ni.Sr\\";
        let ipa = "ˈni.ʃɹ";
        word.set_xsampa_pronunciation(Some(xsp.to_string()));
        assert_eq!(word.xsampa_pronunciation(), Some(xsp));
        assert_eq!(word.pronunciation(), ipa);

        word.set_xsampa_pronunciation(None);
        assert_eq!(word.xsampa_pronunciation(), None);
        assert_eq!(word.pronunciation(), ipa);

        let pos = PartOfSpeech::Noun;
        word.set_part_of_speech(Some(pos));
        assert_eq!(word.part_of_speech(), Some(pos));
    }

    #[test]
    fn read_xml() {
        let word = test_word();
        let xml = test_xml();
        assert_eq!(Word::load_xml_str(&xml).unwrap(), word);
    }

    #[test]
    fn write_xml() {
        let word = test_word();
        let xml = word.save_xml_string().unwrap();
        assert_eq!(Word::load_xml_str(&xml).unwrap(), word);
    }
}
