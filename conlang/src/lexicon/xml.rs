use std::{borrow::Cow, io::BufRead};

use quick_xml::{
    events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event},
    Reader, Writer,
};

use super::{Lexicon, Word};

/// Valid XML tags for `Lexicon`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XmlTag {
    Unknown,
    Lexicon,
    Word,
    Romanization,
    Pronunciation,
    Translation,
    PartOfSpeech,
}

impl From<&[u8]> for XmlTag {
    fn from(value: &[u8]) -> Self {
        match value {
            b"lexicon" => Self::Lexicon,
            b"word" => Self::Word,
            b"romanization" => Self::Romanization,
            b"pronunciation" => Self::Pronunciation,
            b"translation" => Self::Translation,
            b"part-of-speech" => Self::PartOfSpeech,
            _ => Self::Unknown,
        }
    }
}

impl<'a> From<XmlTag> for Cow<'a, str> {
    fn from(value: XmlTag) -> Self {
        let s = match value {
            XmlTag::Lexicon => "lexicon",
            XmlTag::Word => "word",
            XmlTag::Romanization => "romanization",
            XmlTag::Pronunciation => "pronunciation",
            XmlTag::Translation => "translation",
            XmlTag::PartOfSpeech => "part-of-speech",
            _ => "unknown",
        };
        Cow::Owned(s.to_owned())
    }
}

/// Error type for `Lexicon` XML parsing.
#[derive(Clone, Debug)]
pub enum XmlError {
    /// Error produced by `Reader`
    Reader(quick_xml::Error),
    /// Error produced by `Writer`
    Writer(quick_xml::Error),
    /// Trying to set value of a nonexistent word. This should not happen.
    WriteInvalidWord,
    /// A valid tag in a wrong context.
    WrongContext { ptag: XmlTag, tag: XmlTag },
}

/// XML reader for `Lexicon`. Reads XML data and writes to `Lexicon`
pub struct XmlReader<R> {
    reader: Reader<R>,
    buf: Vec<u8>,
    context: Vec<XmlTag>,
}

impl<R: BufRead> XmlReader<R> {
    pub fn new(reader: R) -> Self {
        let mut reader = Reader::from_reader(reader);
        reader.trim_text(true);

        Self {
            reader,
            buf: Vec::new(),
            context: Vec::new(),
        }
    }

    pub fn read(&mut self) -> Result<Lexicon, XmlError> {
        let mut lex = Lexicon::new();

        loop {
            let ptag = self
                .context
                .last()
                .map(|x| x.to_owned())
                .unwrap_or(XmlTag::Unknown);

            let word = lex
                .words
                .last_mut()
                .ok_or_else(|| XmlError::WriteInvalidWord);

            match self.reader.read_event_into(&mut self.buf) {
                Err(e) => {
                    return Err(XmlError::Reader(e));
                }

                Ok(Event::Eof) => break,

                Ok(Event::Start(e)) => {
                    let tag = XmlTag::from(e.name().as_ref());
                    self.context.push(tag);

                    match (ptag, tag) {
                        // Root tag
                        (XmlTag::Unknown, XmlTag::Lexicon) => {}
                        // Insert new word
                        (XmlTag::Lexicon, XmlTag::Word) => {
                            // words.push(self.read_word());
                            lex.words.push(Word::new());
                        }
                        // Clear word properties
                        (XmlTag::Word, XmlTag::Romanization) => {
                            word?.romanization.clear();
                        }
                        (XmlTag::Word, XmlTag::Pronunciation) => {
                            word?.pronunciation.clear();
                        }
                        (XmlTag::Word, XmlTag::Translation) => {
                            word?.translation.clear();
                        }
                        (XmlTag::Word, XmlTag::PartOfSpeech) => {}
                        // Invalid tag
                        _ => return Err(XmlError::WrongContext { ptag, tag }),
                    }
                }

                Ok(Event::Text(e)) => {
                    let v = e.unescape().map_err(XmlError::Reader)?;
                    match ptag {
                        XmlTag::Romanization => word?.romanization += &v,
                        XmlTag::Pronunciation => word?.pronunciation += &v,
                        XmlTag::Translation => word?.translation += &v,
                        XmlTag::PartOfSpeech => {
                            word?.part_of_speech = Some(v.as_ref().into());
                        }
                        _ => {}
                    }
                }

                Ok(Event::End(e)) => {
                    let _tag = XmlTag::from(e.name().as_ref());
                    self.context.pop();
                }
                _ => {}
            }
            self.buf.clear();
        }

        Ok(lex)
    }
}

/// XML writer for `Lexicon`. Writes `Lexicon` contents into XML.
pub struct XmlWriter<'a, W: std::io::Write> {
    pub writer: Writer<W>,
    pub lex: &'a Lexicon,
}

impl<'a, W: std::io::Write> XmlWriter<'a, W> {
    pub fn new(lex: &'a Lexicon, writer: W) -> Self {
        Self {
            lex,
            // writer: Writer::new_with_indent(writer, ' ' as u8, 2),
            writer: Writer::new(writer),
        }
    }

    pub fn write(&mut self) -> Result<(), XmlError> {
        self.writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
            .map_err(XmlError::Writer)?;

        self.writer
            .write_event(Event::Start(BytesStart::new(XmlTag::Lexicon)))
            .map_err(XmlError::Writer)?;

        for word in self.lex.words.iter() {
            self.writer
                .write_event(Event::Start(BytesStart::new(XmlTag::Word)))
                .map_err(XmlError::Writer)?;

            self.writer
                .write_event(Event::Start(BytesStart::new(XmlTag::Romanization)))
                .map_err(XmlError::Writer)?;
            self.writer
                .write_event(Event::Text(BytesText::new(&word.romanization)))
                .map_err(XmlError::Writer)?;
            self.writer
                .write_event(Event::End(BytesEnd::new(XmlTag::Romanization)))
                .map_err(XmlError::Writer)?;

            self.writer
                .write_event(Event::Start(BytesStart::new(XmlTag::Pronunciation)))
                .map_err(XmlError::Writer)?;
            self.writer
                .write_event(Event::Text(BytesText::new(&word.pronunciation)))
                .map_err(XmlError::Writer)?;
            self.writer
                .write_event(Event::End(BytesEnd::new(XmlTag::Pronunciation)))
                .map_err(XmlError::Writer)?;

            self.writer
                .write_event(Event::Start(BytesStart::new(XmlTag::Translation)))
                .map_err(XmlError::Writer)?;
            self.writer
                .write_event(Event::Text(BytesText::new(&word.translation)))
                .map_err(XmlError::Writer)?;
            self.writer
                .write_event(Event::End(BytesEnd::new(XmlTag::Translation)))
                .map_err(XmlError::Writer)?;

            if let Some(pos) = &word.part_of_speech {
                self.writer
                    .write_event(Event::Start(BytesStart::new(XmlTag::PartOfSpeech)))
                    .map_err(XmlError::Writer)?;
                self.writer
                    .write_event(Event::Text(BytesText::new(pos.name())))
                    .map_err(XmlError::Writer)?;
                self.writer
                    .write_event(Event::End(BytesEnd::new(XmlTag::PartOfSpeech)))
                    .map_err(XmlError::Writer)?;
            }

            self.writer
                .write_event(Event::End(BytesEnd::new(XmlTag::Word)))
                .map_err(XmlError::Writer)?;
        }

        self.writer
            .write_event(Event::End(BytesEnd::new(XmlTag::Lexicon)))
            .map_err(XmlError::Writer)?;
        Ok(())
    }

    pub fn into_inner(self) -> W {
        self.writer.into_inner()
    }
}
