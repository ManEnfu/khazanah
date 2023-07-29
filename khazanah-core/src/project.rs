//! Project handling.
//!
//! ---
//!
//! Project file is a zip archive with the following structure.
//!
//! ```txt
//! *.zip
//! |-mimetype
//! |-lexicon.xml
//! ```

pub use error::{ArchiveError, Error};

use std::{
    fs::File,
    io::{BufReader, Read, Seek, Write},
    path::Path,
};
use zip::{write::FileOptions, ZipArchive, ZipWriter};

use crate::{
    xml::{self, ReadXml, WriteXml},
    Language,
};

mod error;

// The MIME type of a project file.
pub const PROJECT_MIME_TYPE: &str = "application/khz";

// The file extension of a project file.
pub const PROJECT_FILE_EXT: &str = "khz";

/// A project.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Project {
    language: Language,
}

impl Project {
    /// Creates a new project.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn language(&self) -> &Language {
        &self.language
    }

    pub fn language_mut(&mut self) -> &mut Language {
        &mut self.language
    }

    /// Loads project from ZIP archive.
    pub fn load<R: Read + Seek>(reader: R) -> Result<Self, ArchiveError> {
        // Initialize ZIP Archive
        let mut archive = ZipArchive::new(reader)?;

        // Check MIME type
        {
            let mut mimetype_file = archive.by_name("mimetype")?;
            let mut mimetype = String::new();
            mimetype_file.read_to_string(&mut mimetype)?;
            if mimetype.trim() != PROJECT_MIME_TYPE {
                return Err(ArchiveError::WrongMimeType);
            }
        }

        // Loas XML file
        let proj = Self::read_xml(BufReader::new(archive.by_name("khazanah.xml")?))?.0;

        Ok(proj)
    }

    /// Loads project from ZIP file in filesystem.
    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self, ArchiveError> {
        let file = File::open(&path)?;
        let proj = Self::load(file)?;
        Ok(proj)
    }

    /// Saves project to ZIP archive.
    pub fn save<W: Write + Seek>(&self, writer: W) -> Result<W, ArchiveError> {
        let mut archive = ZipWriter::new(writer);

        let options = FileOptions::default();

        // Save MIME type
        archive.start_file("mimetype", options)?;
        archive.write_all(PROJECT_MIME_TYPE.as_bytes())?;

        // Save XML file
        archive.start_file("khazanah.xml", options)?;
        archive = self.write_xml(archive)?;

        Ok(archive.finish()?)
    }

    /// Saves project to ZIP archive in filesystem.
    pub fn save_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), ArchiveError> {
        let file = File::create(&path)?;
        self.save(file)?;
        Ok(())
    }
}

impl ReadXml for Project {
    type Error = Error;

    type ReaderState = ();

    const TAG: &'static str = "khazanah";

    fn process_tag_start<R: std::io::BufRead>(
        &mut self,
        reader: &mut xml::XmlReader<R>,
        _state: &mut Self::ReaderState,
        name: String,
        attrs: Vec<(String, String)>,
    ) -> Result<(), xml::XmlError<Self::Error>> {
        let l = reader.context.len();
        let ptag = match l {
            2.. => reader.context.get(l - 2).map(|s| s.as_str()),
            _ => None,
        };

        match (ptag, name.as_str()) {
            (_, "khazanah") => {}
            (Some("khazanah"), "language") => {
                self.language = Language::deserialize_xml(reader, Some((name, attrs)))
                    .map_err(|xe| xe.map_into())?;
            }
            _ => return Err(xml::XmlError::InvalidTag(name)),
        }

        Ok(())
    }

    fn process_text<R: std::io::BufRead>(
        &mut self,
        _reader: &mut xml::XmlReader<R>,
        _state: &mut Self::ReaderState,
        _text: String,
    ) -> Result<(), xml::XmlError<Self::Error>> {
        Ok(())
    }

    fn process_tag_end<R: std::io::BufRead>(
        &mut self,
        _reader: &mut xml::XmlReader<R>,
        _state: &mut Self::ReaderState,
        _name: String,
    ) -> Result<(), xml::XmlError<Self::Error>> {
        Ok(())
    }
}

impl WriteXml for Project {
    type Error = Error;

    fn serialize_xml<W: Write>(
        &self,
        writer: &mut xml::XmlWriter<W>,
    ) -> Result<(), xml::XmlError<Self::Error>> {
        writer.write_tag_start_with_attributes("khazanah", [("version", "1")])?;
        self.language
            .serialize_xml(writer)
            .map_err(|xe| xe.map_into())?;
        writer.write_tag_end("khazanah")?;

        Ok(())
    }
}
