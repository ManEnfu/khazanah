use std::str::Chars;

use rand::Rng;
use regex::Regex;

use super::{Categories, Category, Error, Inventory, Phoneme};

use crate::xml::{ReadXml, WriteXml, XmlError, XmlReader, XmlWriter};

/// An element of a pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternElement<'a> {
    Str(&'a str),
    Category(&'a Category),
}

#[derive(Debug, Clone)]
pub struct PatternElements<'a> {
    pattern: &'a str,
    p_iter: Chars<'a>,
    categories: &'a Categories,
    index: usize,
    next_char: Option<char>,
}

impl<'a> PatternElements<'a> {
    pub fn new(pattern: &'a Pattern, categories: &'a Categories) -> Self {
        Self {
            pattern: pattern.pattern_str.as_str(),
            p_iter: pattern.pattern_str.chars(),
            categories,
            index: 0,
            next_char: None,
        }
    }
}

impl<'a> Iterator for PatternElements<'a> {
    type Item = PatternElement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let p_len = self.pattern.len();
        if self.index >= p_len {
            None
        } else {
            let init_char = self.next_char.or_else(|| self.p_iter.next())?;
            let mut end_index = self.index + init_char.len_utf8();

            let is_category = init_char.is_ascii_uppercase();

            loop {
                self.next_char = self.p_iter.next();

                if let Some(nc) = self.next_char {
                    if (is_category && nc.is_ascii_digit())
                        || (!is_category && !nc.is_ascii_uppercase())
                    {
                        end_index += nc.len_utf8();
                        continue;
                    }
                }
                break;
            }

            let slice = self.pattern.get(self.index..end_index)?;
            self.index = end_index;

            let elem = if is_category {
                if let Some(cat) = self.categories.category_by_name(slice) {
                    PatternElement::Category(cat)
                } else {
                    PatternElement::Str(slice)
                }
            } else {
                PatternElement::Str(slice)
            };

            Some(elem)
        }
    }
}

/// A syllable pattern. The pattern string can contains IPA symbols and category names.
///
/// For example: `"CrV"` means that the pattern consists of a consonant, followed by /r/,
/// followed by a vowel, forming something like /kra/ or /tri/.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    /// The pattern string.
    pub pattern_str: String,
    /// The mora or weight of the pattern.
    pub mora: u32,
    /// The likeliness of this pattern to occur.
    pub likeliness: usize,
}

impl Pattern {
    /// Creates a new pattern.
    pub fn new(s: String) -> Self {
        Self {
            pattern_str: s,
            mora: 1,
            likeliness: 1,
        }
    }

    /// Parse and iterates pattern string into elements.
    pub fn parse_elements<'a>(&'a self, categories: &'a Categories) -> PatternElements<'a> {
        PatternElements::new(self, categories)
    }

    /// Gets a regex string for the pattern.
    pub fn regex_pattern(&self, categories: &Categories, inventory: &Inventory) -> String {
        let mut ret = "".to_string();

        for elem in self.parse_elements(categories) {
            match elem {
                PatternElement::Str(s) => {
                    ret += s;
                }
                PatternElement::Category(c) => {
                    ret += "(";
                    let mut first = true;
                    for p in c.iter_phonemes(inventory) {
                        if first {
                            first = false;
                        } else {
                            ret += "|"
                        }
                        ret += &regex::escape(p.sound());
                    }
                    ret += ")";
                }
            }
        }

        ret
    }

    /// Gets a regular expression for the pattern, using data in `categories` and `inventory`.
    pub fn regex(&self, categories: &Categories, inventory: &Inventory) -> Result<Regex, Error> {
        let r = format!("^{}$", self.regex_pattern(categories, inventory));
        Regex::new(&r).map_err(Error::from)
    }

    /// Generates a random syllable following the pattern.
    pub fn generate<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        categories: &Categories,
        inventory: &Inventory,
    ) -> String {
        let mut ret = String::new();

        for elem in self.parse_elements(categories) {
            match elem {
                PatternElement::Str(s) => {
                    ret += s;
                }
                PatternElement::Category(c) => {
                    ret += c
                        .choose_phoneme(rng, inventory)
                        .map(Phoneme::sound)
                        .unwrap_or_default();
                }
            }
        }

        ret
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            pattern_str: Default::default(),
            mora: 1,
            likeliness: 1,
        }
    }
}

impl ReadXml for Pattern {
    type Error = Error;

    type ReaderState = ();

    const TAG: &'static str = "pattern";

    fn process_tag_start<R: std::io::BufRead>(
        &mut self,
        reader: &mut XmlReader<R>,
        _state: &mut Self::ReaderState,
        name: String,
        _attrs: Vec<(String, String)>,
    ) -> Result<(), XmlError<Self::Error>> {
        match reader.last_tag_pair() {
            (_, Some(Self::TAG)) => {}
            (Some(Self::TAG), Some("pattern-str")) => {
                self.pattern_str.clear();
            }
            (Some(Self::TAG), Some("mora")) => {
                self.mora = 1;
            }
            (Some(Self::TAG), Some("likeliness")) => {
                self.likeliness = 1;
            }
            _ => return Err(XmlError::InvalidTag(name)),
        }
        Ok(())
    }

    fn process_text<R: std::io::BufRead>(
        &mut self,
        reader: &mut XmlReader<R>,
        _state: &mut Self::ReaderState,
        text: String,
    ) -> Result<(), XmlError<Self::Error>> {
        match reader.last_tag() {
            Some("pattern-str") => {
                self.pattern_str += &text;
            }
            Some("mora") => {
                self.mora = text.parse().unwrap_or(1);
            }
            Some("likeliness") => {
                self.likeliness = text.parse().unwrap_or(1);
            }
            _ => {}
        }

        Ok(())
    }

    fn process_tag_end<R: std::io::BufRead>(
        &mut self,
        _reader: &mut XmlReader<R>,
        _state: &mut Self::ReaderState,
        _name: String,
    ) -> Result<(), XmlError<Self::Error>> {
        Ok(())
    }
}

impl WriteXml for Pattern {
    type Error = Error;

    fn serialize_xml<W: std::io::Write>(
        &self,
        writer: &mut XmlWriter<W>,
    ) -> Result<(), XmlError<Self::Error>> {
        writer.write_tag_start("pattern")?;

        writer.write_tag_start("pattern-str")?;
        writer.write_text(self.pattern_str.as_str())?;
        writer.write_tag_end("pattern-str")?;

        writer.write_tag_start("mora")?;
        writer.write_text(self.mora.to_string().as_str())?;
        writer.write_tag_end("mora")?;

        writer.write_tag_start("likeliness")?;
        writer.write_text(self.likeliness.to_string().as_str())?;
        writer.write_tag_end("likeliness")?;

        writer.write_tag_end("pattern")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Phoneme;

    use super::*;

    const XML1: &str = r#"
    <pattern>
        <pattern-str>CVN</pattern-str>
        <mora>2</mora>
        <likeliness>3</likeliness>
    </pattern>
    "#;

    #[test]
    fn read_xml() {
        let pat = Pattern::load_xml_str(XML1).unwrap();

        assert_eq!(&pat.pattern_str, "CVN");
        assert_eq!(pat.mora, 2);
        assert_eq!(pat.likeliness, 3);
    }

    #[test]
    fn write_xml() {
        let pat = Pattern::load_xml_str(XML1).unwrap();
        let xml2 = pat.save_xml_string().unwrap();
        let pat2 = Pattern::load_xml_str(&xml2).unwrap();
        assert_eq!(&pat, &pat2);
    }

    #[test]
    fn parse_patterns() {
        let mut cats = Categories::new();

        assert_eq!(
            Pattern::new("C".to_string())
                .parse_elements(&cats)
                .collect::<Vec<_>>(),
            vec![PatternElement::Str("C"),]
        );
        assert_eq!(
            Pattern::new("CV".to_string())
                .parse_elements(&cats)
                .collect::<Vec<_>>(),
            vec![PatternElement::Str("C"), PatternElement::Str("V"),]
        );
        assert_eq!(
            Pattern::new("CVC".to_string())
                .parse_elements(&cats)
                .collect::<Vec<_>>(),
            vec![
                PatternElement::Str("C"),
                PatternElement::Str("V"),
                PatternElement::Str("C"),
            ]
        );
        assert_eq!(
            Pattern::new("CrV".to_string())
                .parse_elements(&cats)
                .collect::<Vec<_>>(),
            vec![
                PatternElement::Str("C"),
                PatternElement::Str("r"),
                PatternElement::Str("V"),
            ]
        );
        assert_eq!(
            Pattern::new("C1C2rVN".to_string())
                .parse_elements(&cats)
                .collect::<Vec<_>>(),
            vec![
                PatternElement::Str("C1"),
                PatternElement::Str("C2"),
                PatternElement::Str("r"),
                PatternElement::Str("V"),
                PatternElement::Str("N"),
            ]
        );
        assert_eq!(
            Pattern::new("Cro".to_string())
                .parse_elements(&cats)
                .collect::<Vec<_>>(),
            vec![PatternElement::Str("C"), PatternElement::Str("ro"),]
        );

        let mut cat = Category::new();
        cat.set_name("C".to_string());
        let _ = cats.add_category(cat);

        let mut cat = Category::new();
        cat.set_name("V".to_string());
        let _ = cats.add_category(cat);

        let c_ref = cats.category_by_name("C").unwrap();
        let v_ref = cats.category_by_name("V").unwrap();

        assert_eq!(
            Pattern::new("CrVC".to_string())
                .parse_elements(&cats)
                .collect::<Vec<_>>(),
            vec![
                PatternElement::Category(c_ref),
                PatternElement::Str("r"),
                PatternElement::Category(v_ref),
                PatternElement::Category(c_ref),
            ]
        );
    }

    fn test_data() -> (Categories, Inventory) {
        let mut cats = Categories::new();
        let mut inv = Inventory::new();

        let mut cat = Category::new();
        cat.set_name("C".to_string());
        cat.add_phoneme_id(inv.add_phoneme(Phoneme::with_sound("m".to_string())));
        cat.add_phoneme_id(inv.add_phoneme(Phoneme::with_sound("n".to_string())));
        cat.add_phoneme_id(inv.add_phoneme(Phoneme::with_sound("p".to_string())));
        cat.add_phoneme_id(inv.add_phoneme(Phoneme::with_sound("t".to_string())));
        cat.add_phoneme_id(inv.add_phoneme(Phoneme::with_sound("k".to_string())));
        let _ = cats.add_category(cat);

        let mut cat = Category::new();
        cat.set_name("V".to_string());
        cat.add_phoneme_id(inv.add_phoneme(Phoneme::with_sound("a".to_string())));
        cat.add_phoneme_id(inv.add_phoneme(Phoneme::with_sound("i".to_string())));
        cat.add_phoneme_id(inv.add_phoneme(Phoneme::with_sound("u".to_string())));
        let _ = cats.add_category(cat);

        (cats, inv)
    }

    #[test]
    fn regex() {
        let (cats, inv) = test_data();

        let pat = Pattern::new("CrVC".to_string());
        dbg!(pat.regex_pattern(&cats, &inv));

        let re = pat.regex(&cats, &inv).unwrap();
        assert!(re.is_match("krak"));
        assert!(re.is_match("mrin"));
        assert!(re.is_match("prun"));
        assert!(!re.is_match("kun"));
        assert!(!re.is_match("atrun"));
        assert!(!re.is_match("truna"));
    }

    #[test]
    fn generate() {
        let (cats, inv) = test_data();
        let mut rng = rand::thread_rng();

        for p in ["CV", "CrV", "CVC", "CrVC"] {
            dbg!(p);
            let pat = Pattern::new(p.to_string());
            let re = pat.regex(&cats, &inv).unwrap();
            for _i in 0..10 {
                let s = pat.generate(&mut rng, &cats, &inv);
                dbg!(&s);
                assert!(re.is_match(&s));
            }
        }
    }
}
