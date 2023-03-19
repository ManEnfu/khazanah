//! Generic reader and writer for XML files in UTF-8 encoding.

use std::{borrow::Cow, io::BufRead, str::Utf8Error};

use quick_xml::{
    events::{
        attributes::{AttrError, Attribute},
        BytesDecl, BytesEnd, BytesStart, BytesText, Event,
    },
    Reader, Writer,
};

/// Error type for `XMLReader` and `XMLWriter`..
#[derive(Clone, Debug, thiserror::Error)]
pub enum XmlError<E> {
    /// Error produced by `quick_xml`.
    #[error(transparent)]
    Qxml(#[from] quick_xml::Error),
    #[error(transparent)]
    Qattr(#[from] AttrError),
    /// Encoding error.
    #[error(transparent)]
    Utf8(#[from] Utf8Error),
    /// Other, domain specific error.
    #[error(transparent)]
    Other(E),
}

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
pub struct XmlReader<R, P> {
    reader: Reader<R>,
    buf: Vec<u8>,
    context: Vec<String>,

    processor: P,
    // process_tag_start: Option<Box<dyn Fn(O, &[String], &str, Vec<(&str, String)>) -> Result<O, E>>>,
    // process_text: Option<Box<dyn Fn(O, &[String], Cow<str>) -> Result<O, E>>>,
    // process_tag_end: Option<Box<dyn Fn(O, &[String], &str) -> Result<O, E>>>,
}

impl<R, P> XmlReader<R, P>
where
    R: BufRead,
    P: XmlReaderProcess,
{
    /// Creates a new reader.
    pub fn new(reader: R, processor: P) -> Self {
        let mut reader = Reader::from_reader(reader);
        reader.trim_text(true);

        Self {
            reader,
            buf: Vec::new(),
            context: Vec::new(),

            processor,
        }
    }

    // /// Assigns a callback when encountering an opening tag.
    // pub fn process_tag_start<F>(&mut self, f: F) -> &mut Self
    // where
    //     F: Fn(O, &[String], &str, Vec<(&str, String)>) -> Result<O, E> + 'static,
    // {
    //     self.process_tag_start = Some(Box::new(f));
    //     self
    // }

    // /// Assigns a callback when encountering a text.
    // pub fn process_text<F>(&mut self, f: F) -> &mut Self
    // where
    //     F: Fn(O, &[String], Cow<str>) -> Result<O, E> + 'static,
    // {
    //     self.process_text = Some(Box::new(f));
    //     self
    // }

    // /// Assigns a callback when encountering a closing tag.
    // pub fn process_tag_end<F>(&mut self, f: F) -> &mut Self
    // where
    //     F: Fn(O, &[String], &str) -> Result<O, E> + 'static,
    // {
    //     self.process_tag_end = Some(Box::new(f));
    //     self
    // }

    /// Reads the content.
    pub fn read(&mut self) -> Result<P::Output, XmlError<P::Error>> {
        let mut data: P::Output = Default::default();

        loop {
            match self.reader.read_event_into(&mut self.buf) {
                Err(e) => {
                    return Err(XmlError::Qxml(e));
                }

                Ok(Event::Eof) => break,

                Ok(Event::Start(e)) => {
                    let name = std::str::from_utf8(e.name().into_inner())?;
                    let mut attrs = Vec::new();
                    for a in e.attributes() {
                        let attr = a?;
                        attrs.push((
                            std::str::from_utf8(attr.key.into_inner())?,
                            attr.unescape_value()?.to_string(),
                        ));
                    }
                    self.context.push(name.to_owned());
                    data = self
                        .processor
                        .process_tag_start(data, &self.context, name, attrs)
                        .map_err(XmlError::Other)?;
                }

                Ok(Event::Text(e)) => {
                    let text = e.unescape()?;
                    data = self
                        .processor
                        .process_text(data, &self.context, text)
                        .map_err(XmlError::Other)?;
                }

                Ok(Event::End(e)) => {
                    let name = std::str::from_utf8(e.name().into_inner())?;
                    self.context.pop();
                    data = self
                        .processor
                        .process_tag_end(data, &self.context, name)
                        .map_err(XmlError::Other)?;
                }

                _ => {}
            }
        }

        Ok(data)
    }
}

/// Generic XML writer.
pub struct XmlWriter<W: std::io::Write> {
    pub writer: Writer<W>,
}

impl<W: std::io::Write> XmlWriter<W> {
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

    /// Writes an opening tag
    pub fn write_tag_start<E>(&mut self, name: &str) -> Result<(), XmlError<E>> {
        self.writer
            .write_event(Event::Start(BytesStart::new(name)))
            .map_err(XmlError::Qxml)
    }

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
