use std::{
    borrow::Cow,
    fs::File,
    io::{BufRead, BufReader, Cursor, Write},
    path::Path,
    string::FromUtf8Error,
};

use crate::xml::{XmlError, XmlReader, XmlWriter};

/// Error type for `Meta`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Filesystem error
    #[error("Filesystem error: {0}")]
    Fs(#[from] std::io::Error),
    /// XML error
    #[error("XML: {0}")]
    Xml(#[from] XmlError<ReadError>),
    /// Converting from Utf8
    #[error("Error in converting to string from UTF-8: {0}")]
    FromUtf8(#[from] FromUtf8Error),
}

/// Error type that can be emitted by reading a `Meta` file.
#[derive(Clone, Debug, thiserror::Error)]
pub enum ReadError {
    /// A valid tag in a wrong context.
    #[error("tag <{}> should not be inside <{}>", .tag, .ptag)]
    WrongContext { ptag: String, tag: String },
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

    /// Reads from XML.
    pub fn read_xml<R: BufRead>(reader: R) -> Result<Self, Error> {
        XmlReader::<R, Meta, ReadError>::new(reader)
            .process_tag_start(|mut meta, ctx, _name, _attrs| {
                let l = ctx.len();
                let tag = ctx.last().map(|s| s.as_str());
                let ptag = match l {
                    2.. => ctx.get(l - 2).map(|s| s.as_str()),
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
                        return Err(ReadError::WrongContext {
                            ptag: ptag.unwrap_or_default().to_string(),
                            tag: tag.unwrap_or_default().to_string(),
                        })
                    }
                }
                Ok(meta)
            })
            .process_text(|mut meta, ctx, text| {
                let tag = ctx.last().map(|s| s.as_str());
                match tag {
                    Some("name") => meta.name += &text,
                    Some("local-lang") => meta.local_lang += &text,
                    Some("author") => meta.author += &text,
                    Some("description") => meta.description += &text,
                    _ => {}
                }
                Ok(meta)
            })
            .read()
            .map_err(Error::Xml)
    }

    /// Load `Meta` from XML file.
    pub fn load_xml_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let f = File::open(path)?;
        Self::read_xml(BufReader::new(f))
    }

    /// Load `Meta` from XML string.
    pub fn load_xml_str(s: &str) -> Result<Self, Error> {
        Self::read_xml(s.as_bytes())
    }

    /// Writes to XML.
    pub fn write_xml<W: Write>(&self, writer: W) -> Result<W, Error> {
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

    /// Saves `Meta` to XML file.
    pub fn save_xml_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let f = File::create(path)?;
        self.write_xml(f)?;
        Ok(())
    }

    /// Saves `Meta` to XML string.
    pub fn save_xml_string(&self) -> Result<String, Error> {
        let w = self.write_xml(Cursor::new(Vec::<u8>::new())).unwrap();
        let ar = w.into_inner();
        String::from_utf8(ar).map_err(Error::from)
    }
}
