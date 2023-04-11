use std::{
    borrow::Cow,
    io::{BufRead, Write},
};

use crate::xml::{ReadXml, WriteXml, XmlError, XmlReader, XmlReaderProcess, XmlWriter};

/// Error type that can be emitted by reading a `Meta` file.
#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    /// A valid tag in a wrong context.
    #[error("tag <{}> should not be inside <{}>", .tag, .ptag)]
    WrongContext { ptag: String, tag: String },

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

    fn read_xml<R: BufRead>(reader: R) -> Result<(Self, R), XmlError<Self::Error>> {
        let mut xml_reader = XmlReader::new(reader, XmlReaderProcessor::new());
        let ret = xml_reader.read()?;
        Ok((ret, xml_reader.finish()))
    }
}

struct XmlReaderProcessor();

impl XmlReaderProcessor {
    pub fn new() -> Self {
        Self()
    }
}

impl XmlReaderProcess for XmlReaderProcessor {
    type Output = Meta;
    type Error = Error;

    fn process_tag_start(
        &mut self,
        mut meta: Self::Output,
        context: &[String],
        _name: &str,
        _attrs: Vec<(&str, String)>,
    ) -> Result<Self::Output, Self::Error> {
        let l = context.len();
        let tag = context.last().map(|s| s.as_str());
        let ptag = match l {
            2.. => context.get(l - 2).map(|s| s.as_str()),
            _ => None,
        };

        match (ptag, tag) {
            // Root tag;
            (None, Some("project")) => {}
            // Clear meta properties
            (Some("project"), Some("name")) => {
                meta.name.clear();
            }
            (Some("project"), Some("local-lang")) => {
                meta.local_lang.clear();
            }
            (Some("project"), Some("author")) => {
                meta.author.clear();
            }
            (Some("project"), Some("description")) => {
                meta.description.clear();
            }
            // Invalid tag
            _ => {
                return Err(Error::WrongContext {
                    ptag: ptag.unwrap_or_default().to_string(),
                    tag: tag.unwrap_or_default().to_string(),
                })
            }
        }
        Ok(meta)
    }

    fn process_text(
        &mut self,
        mut meta: Self::Output,
        context: &[String],
        text: Cow<str>,
    ) -> Result<Self::Output, Self::Error> {
        let tag = context.last().map(|s| s.as_str());

        match tag {
            Some("name") => meta.name += &text,
            Some("local-lang") => meta.local_lang += &text,
            Some("author") => meta.author += &text,
            Some("description") => meta.description += &text,
            _ => {}
        }
        Ok(meta)
    }
}

impl WriteXml for Meta {
    type Error = Error;

    fn write_xml<W: Write>(&self, writer: W) -> Result<W, XmlError<Self::Error>> {
        let mut w = XmlWriter::new(writer);

        w.write_init()?;
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

        Ok(w.finish())
    }
}
