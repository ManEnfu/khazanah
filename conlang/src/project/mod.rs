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

use std::{
    fs::File,
    future::Future,
    io::{BufReader, Read, Seek, Write},
    path::{Path, PathBuf},
    pin::Pin,
};
use zip::{write::FileOptions, ZipArchive, ZipWriter};

use crate::Lexicon;

pub mod error;

// The MIME type of a project file.
pub const PROJECT_MIME_TYPE: &str = "application/cnpr";

/// A project.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Project {
    /// The file path of the project.
    file_path: Option<PathBuf>,
    /// The lexicon of the project.
    lexicon: Lexicon,
}

impl Project {
    /// Creates a new project.
    pub fn new() -> Self {
        Self::default()
    }

    /// Getter for file path of the project.
    pub fn file_path(&self) -> Option<&Path> {
        self.file_path
            .as_ref()
            .map(<PathBuf as AsRef<Path>>::as_ref)
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

        Ok(Self {
            file_path: None,
            lexicon,
        })
    }

    /// Loads project from ZIP file in filesystem.
    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(&path)?;
        let mut proj = Self::load(file)?;
        proj.file_path = Some(path.as_ref().into());
        Ok(proj)
    }

    pub async fn load_file_async<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        todo!()
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
    pub fn save_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        let file = File::create(&path)?;
        self.save(file)?;
        self.file_path = Some(path.as_ref().into());
        Ok(())
    }

    pub async fn save_file_async<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        todo!()
    }
}
