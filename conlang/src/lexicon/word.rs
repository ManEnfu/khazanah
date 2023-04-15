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
    pub id: Option<Uuid>,
    /// Romanization of the word.
    pub romanization: String,
    /// Translation of the word.
    pub translation: String,
    /// Pronunciation of word in IPA.
    pub pronunciation: String,
    /// Which part of speech this word belongs to.
    pub part_of_speech: Option<PartOfSpeech>,
    /// X-SAMPA pronunciation of the word, if exists.
    pub xsampa_pronunciation: Option<String>,
}

impl Word {
    /// Creates a new word.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_id(id: Uuid) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    /// Sets X-SAMPA pronunciation of the word and converts it to IPA pronunciation.
    pub fn set_xsampa_pronunciation(&mut self, s: Option<String>) {
        if let Some(s) = &s {
            self.pronunciation = ipa::transliterate_xsampa(s);
        }
        self.xsampa_pronunciation = s;
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
