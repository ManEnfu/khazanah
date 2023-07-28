use std::io::{BufRead, Write};

use super::Error;
use crate::xml::{ReadXml, WriteXml, XmlError, XmlReader, XmlWriter};

/// Metadata for a language.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
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
            (_, Some(Self::TAG)) => {}
            // Clear meta properties
            (Some(Self::TAG), Some("name")) => {
                self.name.clear();
            }
            (Some(Self::TAG), Some("local-lang")) => {
                self.local_lang.clear();
            }
            (Some(Self::TAG), Some("author")) => {
                self.author.clear();
            }
            (Some(Self::TAG), Some("description")) => {
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
        w.write_tag_start("meta")?;

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

        w.write_tag_end("meta")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_meta() -> Meta {
        Meta {
            name: "Test Language".to_owned(),
            local_lang: "English".to_owned(),
            author: "ManEnfu".to_owned(),
            description: "This is a language.".to_owned(),
        }
    }

    fn test_xml() -> String {
        r#"
            <?xml version="1.0" encoding="UTF8"?>
            <meta>
                <name>Test Language</name>
                <local-lang>English</local-lang>
                <author>ManEnfu</author>
                <description>This is a language.</description>
            </meta>
        "#
        .to_string()
    }

    #[test]
    fn read_xml() {
        let meta = test_meta();
        let xml = test_xml();
        assert_eq!(Meta::load_xml_str(&xml).unwrap(), meta);
    }

    #[test]
    fn write_xml() {
        let meta = test_meta();
        let xml = meta.save_xml_string().unwrap();
        assert_eq!(Meta::load_xml_str(&xml).unwrap(), meta);
    }
}
