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

pub use error::Error;
pub use meta::Meta;

use std::{
    fs::File,
    io::{BufReader, Read, Seek, Write},
    path::Path,
};
use zip::{result::ZipError, write::FileOptions, ZipArchive, ZipWriter};

use crate::{
    xml::{ReadXml, WriteXml},
    Lexicon,
};

mod error;
pub mod meta;

// The MIME type of a project file.
pub const PROJECT_MIME_TYPE: &str = "application/khz";

// The file extension of a project file.
pub const PROJECT_FILE_EXT: &str = "khz";

/// A project.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Project {
    /// The metadata of the project.
    pub(crate) meta: Meta,
    /// The lexicon of the project.
    pub(crate) lexicon: Lexicon,
}

impl Project {
    /// Creates a new project.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a reference to the metadata.
    pub fn meta(&self) -> &Meta {
        &self.meta
    }

    /// Get a mutable reference to the metadata.
    pub fn meta_mut(&mut self) -> &mut Meta {
        &mut self.meta
    }

    pub fn lexicon(&self) -> &Lexicon {
        &self.lexicon
    }

    pub fn lexicon_mut(&mut self) -> &mut Lexicon {
        &mut self.lexicon
    }

    /// Loads project from ZIP archive.
    pub fn load<R: Read + Seek>(reader: R) -> Result<Self, Error> {
        // Initialize ZIP Archive
        let mut archive = ZipArchive::new(reader)?;

        // Check MIME type
        {
            let mut mimetype_file = archive.by_name("mimetype")?;
            let mut mimetype = String::new();
            mimetype_file.read_to_string(&mut mimetype)?;
            if mimetype.trim() != PROJECT_MIME_TYPE {
                return Err(Error::WrongMimeType);
            }
        }

        // Load metadata
        let meta = match archive.by_name("meta.xml") {
            Ok(f) => Meta::read_xml(BufReader::new(f))?.0,
            Err(ZipError::FileNotFound) => Meta::new(),
            Err(e) => return Err(Error::Zip(e)),
        };

        // Load lexicon
        let lexicon = match archive.by_name("lexicon.xml") {
            Ok(f) => Lexicon::read_xml(BufReader::new(f))?.0,
            Err(ZipError::FileNotFound) => Lexicon::new(),
            Err(e) => return Err(Error::Zip(e)),
        };

        Ok(Self { meta, lexicon })
    }

    /// Loads project from ZIP file in filesystem.
    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(&path)?;
        let proj = Self::load(file)?;
        Ok(proj)
    }

    /// Saves project to ZIP archive.
    pub fn save<W: Write + Seek>(&self, writer: W) -> Result<W, Error> {
        let mut archive = ZipWriter::new(writer);

        let options = FileOptions::default();

        // Save MIME type
        archive.start_file("mimetype", options)?;
        archive.write_all(PROJECT_MIME_TYPE.as_bytes())?;

        // Save metadata
        archive.start_file("meta.xml", options)?;
        archive = self.meta.write_xml(archive)?;

        // Save lexicon
        archive.start_file("lexicon.xml", options)?;
        archive = self.lexicon.write_xml(archive)?;

        Ok(archive.finish()?)
    }

    /// Saves project to ZIP archive in filesystem.
    pub fn save_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        let file = File::create(&path)?;
        self.save(file)?;
        Ok(())
    }
}
