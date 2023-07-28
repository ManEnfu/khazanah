//! Generic reader and writer for XML files in UTF-8 encoding.

use std::{
    borrow::Cow,
    fs::File,
    io::{BufRead, BufReader, Cursor, Write},
    path::Path,
    str::Utf8Error,
    string::FromUtf8Error, todo,
};

use quick_xml::{
    events::{
        attributes::{AttrError, Attribute},
        BytesDecl, BytesEnd, BytesStart, BytesText, Event,
    },
    Reader, Writer,
};

/// Error type for `XMLReader` and `XMLWriter`..
#[derive(Debug, thiserror::Error)]
pub enum XmlError<E> {
    /// Filesystem error
    #[error("Filesystem error: {0}")]
    Fs(#[from] std::io::Error),
    /// Error produced by `quick_xml`.
    #[error(transparent)]
    Qxml(#[from] quick_xml::Error),
    /// Error produced by processing tag attributes.
    #[error(transparent)]
    Qattr(#[from] AttrError),
    /// Encoding error.
    #[error("Error in converting to str from UTF-8: {0}")]
    Utf8(#[from] Utf8Error),
    /// Encoding error.
    #[error("Error in converting to string from UTF-8: {0}")]
    FromUtf8(#[from] FromUtf8Error),
    /// Invalid tag.
    #[error("Invalid tag: <{0}>")]
    InvalidTag(String),
    /// Other, domain specific error.
    #[error(transparent)]
    Other(E),
}

impl<E> XmlError<E> {
    pub fn map_other<F, U>(self, f: F) -> XmlError<U> 
    where
        F: Fn(E) -> U
    {
        match self {
            XmlError::Fs(e) => XmlError::Fs(e),
            XmlError::Qxml(e) => XmlError::Qxml(e),
            XmlError::Qattr(e) => XmlError::Qattr(e),
            XmlError::Utf8(e) => XmlError::Utf8(e),
            XmlError::FromUtf8(e) => XmlError::FromUtf8(e),
            XmlError::InvalidTag(e) => XmlError::InvalidTag(e),
            XmlError::Other(e) => XmlError::Other(f(e)),
        }
    }

    pub fn map_into<U>(self) -> XmlError<U>
    where
        U: From<E>
    {
        match self {
            XmlError::Fs(e) => XmlError::Fs(e),
            XmlError::Qxml(e) => XmlError::Qxml(e),
            XmlError::Qattr(e) => XmlError::Qattr(e),
            XmlError::Utf8(e) => XmlError::Utf8(e),
            XmlError::FromUtf8(e) => XmlError::FromUtf8(e),
            XmlError::InvalidTag(e) => XmlError::InvalidTag(e),
            XmlError::Other(e) => XmlError::Other(e.into()),
        }
    }
}

/// Processor for `XmlReader`.
pub trait XmlReaderProcess {
    type Output: Default;

    type Error;

    fn process_tag_start(
        &mut self,
        data: Self::Output,
        _context: &[String],
        _name: &str,
        _attrs: Vec<(&str, String)>,
    ) -> Result<Self::Output, Self::Error> {
        Ok(data)
    }

    fn process_text(
        &mut self,
        data: Self::Output,
        _context: &[String],
        _text: Cow<str>,
    ) -> Result<Self::Output, Self::Error> {
        Ok(data)
    }

    fn process_tag_end(
        &mut self,
        data: Self::Output,
        _context: &[String],
        _name: &str,
    ) -> Result<Self::Output, Self::Error> {
        Ok(data)
    }
}

/// Generic XML reader.
#[allow(clippy::type_complexity)]
pub struct XmlReader<R> {
    reader: Reader<R>,
    buf: Vec<u8>,
    pub context: Vec<String>,
}

impl<R> XmlReader<R>
where
    R: BufRead,
{
    /// Creates a new reader.
    pub fn new(reader: R) -> Self {
        let mut reader = Reader::from_reader(reader);
        reader.trim_text(true);

        Self {
            reader,
            buf: Vec::new(),
            context: Vec::new(),
        }
    }

    /// Reads next event.
    pub fn read_event(&mut self) -> Result<Event, quick_xml::Error> {
        self.buf.clear();
        let ev = self.reader.read_event_into(&mut self.buf);
        match &ev {
            Ok(Event::Start(e)) => {
                let name = std::str::from_utf8(e.name().into_inner()).unwrap();
                self.context.push(name.to_owned());
            }

            Ok(Event::End(_)) => {
                self.context.pop();
            }

            _ => {}
        }
        ev
    }

    /// Finishes reading and returns the underlying reader.
    pub fn finish(self) -> R {
        self.reader.into_inner()
    }
}

/// Generic XML writer.
pub struct XmlWriter<W>
where
    W: std::io::Write,
{
    pub writer: Writer<W>,
}

impl<W> XmlWriter<W>
where
    W: std::io::Write,
{
    /// Creates a new writer.
    pub fn new(writer: W) -> Self {
        Self {
            writer: Writer::new(writer),
        }
    }

    /// Adds XML declaration.
    pub fn write_init<E>(&mut self) -> Result<(), XmlError<E>> {
        self.writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
        Ok(())
    }

    /// Writes an opening tag.
    pub fn write_tag_start<E>(&mut self, name: &str) -> Result<(), XmlError<E>> {
        self.writer
            .write_event(Event::Start(BytesStart::new(name)))
            .map_err(XmlError::Qxml)
    }

    /// Writes an opening tag with attributes.
    pub fn write_tag_start_with_attributes<'a, E, I>(
        &mut self,
        name: &str,
        attrs: I,
    ) -> Result<(), XmlError<E>>
    where
        I: IntoIterator,
        I::Item: Into<Attribute<'a>>,
    {
        self.writer
            .write_event(Event::Start(BytesStart::new(name).with_attributes(attrs)))
            .map_err(XmlError::Qxml)
    }

    /// Writes a text.
    pub fn write_text<E>(&mut self, text: &str) -> Result<(), XmlError<E>> {
        self.writer
            .write_event(Event::Text(BytesText::new(text)))
            .map_err(XmlError::Qxml)
    }

    /// Writes a closing tag.
    pub fn write_tag_end<E>(&mut self, name: &str) -> Result<(), XmlError<E>> {
        self.writer
            .write_event(Event::End(BytesEnd::new(name)))
            .map_err(XmlError::Qxml)
    }

    /// Finishes writing and returns the underlying writer.
    pub fn finish(self) -> W {
        self.writer.into_inner()
    }
}

/// A trait for object that can be read from XML.
pub trait ReadXml
where
    Self: Sized + Default,
{
    type Error;

    type ReaderState: Default;

    const TAG: &'static str;

    /// Processes an opening tag.
    fn process_tag_start<R: BufRead>(
        &mut self,
        reader: &mut XmlReader<R>,
        state: &mut Self::ReaderState,
        name: String,
        attrs: Vec<(String, String)>,
    ) -> Result<(), XmlError<Self::Error>>;

    /// Processes a text.
    fn process_text<R: BufRead>(
        &mut self,
        reader: &mut XmlReader<R>,
        state: &mut Self::ReaderState,
        text: String,
    ) -> Result<(), XmlError<Self::Error>>;

    /// Processes a closing tag.
    fn process_tag_end<R: BufRead>(
        &mut self,
        reader: &mut XmlReader<R>,
        state: &mut Self::ReaderState,
        name: String,
    ) -> Result<(), XmlError<Self::Error>>;

    /// Deserializes XML into object.
    fn deserialize_xml<R: BufRead>(
        reader: &mut XmlReader<R>,
        tag_start: Option<(String, Vec<(String, String)>)>,
    ) -> Result<Self, XmlError<Self::Error>> {
        let mut data = Self::default();
        let mut state = Self::ReaderState::default();

        if let Some((name, attrs)) = tag_start {
            data.process_tag_start(reader, &mut state, name, attrs)?;
        }

        loop {
            match reader.read_event() {
                Err(e) => {
                    return Err(XmlError::Qxml(e));
                }

                Ok(Event::Eof) => break,

                Ok(Event::Start(e)) => {
                    let name = std::str::from_utf8(e.name().into_inner())?.to_string();
                    let mut attrs = Vec::new();
                    for a in e.attributes() {
                        let attr = a?;
                        attrs.push((
                            std::str::from_utf8(attr.key.into_inner())?.to_string(),
                            attr.unescape_value()?.to_string(),
                        ));
                    }
                    data.process_tag_start(reader, &mut state, name, attrs)?;
                }

                Ok(Event::Text(e)) => {
                    let text = e.unescape()?.to_string();
                    data.process_text(reader, &mut state, text)?;
                }

                Ok(Event::End(e)) => {
                    let name = std::str::from_utf8(e.name().into_inner())?.to_string();
                    let tag_end = name == Self::TAG;
                    data.process_tag_end(reader, &mut state, name)?;
                    if tag_end {
                        break;
                    }
                }

                _ => {}
            }
        }
        Ok(data)
    }

    /// Reads fro XML.
    fn read_xml<R: BufRead>(reader: R) -> Result<(Self, R), XmlError<Self::Error>> {
        let mut r = XmlReader::new(reader);
        let ret = Self::deserialize_xml(&mut r, None)?;
        Ok((ret, r.finish()))
    }

    /// Loads from XML file.
    fn load_xml_file<P: AsRef<Path>>(path: P) -> Result<Self, XmlError<Self::Error>> {
        let f = File::open(path)?;
        Self::read_xml(BufReader::new(f)).map(|r| r.0)
    }

    /// Loads from XML string.
    fn load_xml_str(s: &str) -> Result<Self, XmlError<Self::Error>> {
        Self::read_xml(s.as_bytes()).map(|r| r.0)
    }
}

/// A trait for object that can be written into XML.
pub trait WriteXml
where
    Self: Sized,
{
    type Error;

    /// Serializes object into XML.
    fn serialize_xml<W: Write>(
        &self,
        writer: &mut XmlWriter<W>,
    ) -> Result<(), XmlError<Self::Error>>;

    /// Writes to XML.
    fn write_xml<W: Write>(&self, writer: W) -> Result<W, XmlError<Self::Error>> {
        let mut w = XmlWriter::new(writer);

        w.write_init()?;
        self.serialize_xml(&mut w)?;
        Ok(w.finish())
    }

    /// Saves to XML file.
    fn save_xml_file<P: AsRef<Path>>(&self, path: P) -> Result<(), XmlError<Self::Error>> {
        let f = File::create(path)?;
        self.write_xml(f)?;
        Ok(())
    }

    /// Saves to XML string.
    fn save_xml_string(&self) -> Result<String, XmlError<Self::Error>> {
        let w = self.write_xml(Cursor::new(Vec::<u8>::new()))?;
        let ar = w.into_inner();
        String::from_utf8(ar).map_err(XmlError::from)
    }
}
