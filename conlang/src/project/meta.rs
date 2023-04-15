use std::io::{BufRead, Write};

use crate::xml::{ReadXml, WriteXml, XmlError, XmlReader, XmlWriter};

/// Error type that can be emitted by reading a `Meta` file.
#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("<word> tag doesn't have attribute `id`")]
    NoId,
}

/// Metadata for a project.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Meta {
    /// Name of the project, as well as the language family.
    pub name: String,
    /// Language to which the conlangs are translated.
    pub local_lang: String,
    /// The author of the project.
    pub author: String,
    /// The description of the project.
    pub description: String,
}

impl Meta {
    /// Creates a new meta.
    pub fn new() -> Self {
        Self::default()
    }
}

impl ReadXml for Meta {
    type Error = Error;

    type ReaderState = ();

    const TAG: &'static str = "meta";

    fn process_tag_start<R: BufRead>(
        &mut self,
        reader: &mut XmlReader<R>,
        _state: &mut Self::ReaderState,
        _name: String,
        _attrs: Vec<(String, String)>,
    ) -> Result<(), XmlError<Self::Error>> {
        let l = reader.context.len();
        let tag = reader.context.last().map(|s| s.as_str());
        let ptag = match l {
            2.. => reader.context.get(l - 2).map(|s| s.as_str()),
            _ => None,
        };

        match (ptag, tag) {
            // Root tag;
            (None, Some("project")) => {}
            // Clear meta properties
            (Some("project"), Some("name")) => {
                self.name.clear();
            }
            (Some("project"), Some("local-lang")) => {
                self.local_lang.clear();
            }
            (Some("project"), Some("author")) => {
                self.author.clear();
            }
            (Some("project"), Some("description")) => {
                self.description.clear();
            }
            // Invalid tag
            _ => return Err(XmlError::InvalidTag(tag.unwrap_or_default().to_string())),
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
            Some("name") => self.name += &text,
            Some("local-lang") => self.local_lang += &text,
            Some("author") => self.author += &text,
            Some("description") => self.description += &text,
            _ => {}
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

impl WriteXml for Meta {
    type Error = Error;

    fn serialize_xml<W: Write>(&self, w: &mut XmlWriter<W>) -> Result<(), XmlError<Self::Error>> {
        w.write_tag_start("project")?;

        w.write_tag_start("name")?;
        w.write_text(&self.name)?;
        w.write_tag_end("name")?;

        w.write_tag_start("local-lang")?;
        w.write_text(&self.local_lang)?;
        w.write_tag_end("local-lang")?;

        w.write_tag_start("author")?;
        w.write_text(&self.author)?;
        w.write_tag_end("author")?;

        w.write_tag_start("description")?;
        w.write_text(&self.description)?;
        w.write_tag_end("description")?;

        w.write_tag_end("project")?;

        Ok(())
    }
}
