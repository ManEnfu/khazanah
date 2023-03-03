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

use zip::{ZipArchive, ZipWriter, write::FileOptions};
use std::{
    fs::File,
    io::{BufReader, Read, Seek, Write},
    path::Path,
};

use crate::Lexicon;

pub mod error;

// The MIME type of a project file.
pub const PROJECT_MIME_TYPE: &str = "application/cnpr";

/// A project.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Project {
    /// The lexicon of the project.
    pub lexicon: Lexicon,
}

impl Project {
    /// Creates a new project.
    pub fn new() -> Self {
        Self::default()
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

        // Load lexicon
        let lex_file = archive.by_name("lexicon.xml")?;
        let lexicon = Lexicon::read_xml(BufReader::new(lex_file))?;

        Ok(Self { lexicon })
    }

    /// Loads project from ZIP file in filesystem.
    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(path)?;
        Self::load(file)
    }

    /// Saves project to ZIP archive.
    pub fn save<W: Write + Seek>(&self, writer: W) -> Result<W, Error> {
        let mut archive = ZipWriter::new(writer);

        let options = FileOptions::default();

        archive.start_file("mimetype", options)?;
        archive.write_all(b"application/cnpr")?;

        archive.start_file("lexicon.xml", options)?;
        archive = self.lexicon.write_xml(archive)?;
        
        Ok(archive.finish()?)
    }

    /// Saves project to ZIP archive in filesystem.
    pub fn save_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let file = File::create(path)?;
        self.save(file)?;
        Ok(())
    }
}
