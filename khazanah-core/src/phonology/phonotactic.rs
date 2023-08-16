use rand::{seq::SliceRandom, Rng};
use regex::{Regex, RegexSet};

use super::{Categories, Error, Inventory, Pattern};

use crate::xml::{ReadXml, WriteXml, XmlError, XmlReader, XmlWriter};

pub enum StressIndexing {
    FromStart(u32),
    FromEnd(u32),
}

pub enum StressRule {
    Syllable(StressIndexing),
    Mora(StressIndexing),
}

/// Rules that governs the formation of syllables.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Phonotactic {
    onset_patterns: Vec<Pattern>,
    nucleus_patterns: Vec<Pattern>,
    coda_patterns: Vec<Pattern>,
    // stress_rule: Option<StressRule>
}

impl Phonotactic {
    /// Creates a new phonotactic.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets a reference to the list of onset patterns.
    pub fn onset_patterns(&self) -> &Vec<Pattern> {
        &self.onset_patterns
    }

    /// Gets a mutable reference to the list of onset patterns.
    pub fn onset_patterns_mut(&mut self) -> &mut Vec<Pattern> {
        &mut self.onset_patterns
    }

    /// Gets a reference to the list of nucleus patterns.
    pub fn nucleus_patterns(&self) -> &Vec<Pattern> {
        &self.nucleus_patterns
    }

    /// Gets a mutable reference to the list of nucleus patterns.
    pub fn nucleus_patterns_mut(&mut self) -> &mut Vec<Pattern> {
        &mut self.nucleus_patterns
    }

    /// Gets a reference to the list of coda patterns.
    pub fn coda_patterns(&self) -> &Vec<Pattern> {
        &self.coda_patterns
    }

    /// Gets a mutable reference to the list of coda patterns.
    pub fn coda_patterns_mut(&mut self) -> &mut Vec<Pattern> {
        &mut self.coda_patterns
    }

    /// Gets a regex string for the pattern.
    pub fn regex_pattern(&self, categories: &Categories, inventory: &Inventory) -> String {
        let mut ret = String::new();

        ret += "(";

        let mut first = true;
        for op in self.onset_patterns.iter() {
            if first {
                first = false;
            } else {
                ret += "|"
            }
            ret += &op.regex_pattern(categories, inventory);
        }

        ret += ")(";

        first = true;
        for np in self.nucleus_patterns.iter() {
            if first {
                first = false;
            } else {
                ret += "|"
            }
            ret += &np.regex_pattern(categories, inventory);
        }

        ret += ")(";

        first = true;
        for cp in self.coda_patterns.iter() {
            if first {
                first = false;
            } else {
                ret += "|"
            }
            ret += &cp.regex_pattern(categories, inventory);
        }

        ret += ")";

        ret
    }

    /// Gets a regular expression for the pattern, using data in `categories` and `inventory`.
    pub fn regex(&self, categories: &Categories, inventory: &Inventory) -> Result<Regex, Error> {
        let r = format!("^{}$", self.regex_pattern(categories, inventory));
        Regex::new(&r).map_err(Error::from)
    }

    /// Gets a regular expression set for the pattern, using data in `categories` and `inventory`.
    pub fn regex_set(
        &self,
        categories: &Categories,
        inventory: &Inventory,
    ) -> Result<RegexSet, Error> {
        let mut ret = Vec::<String>::new();

        for op in self.onset_patterns.iter() {
            let o_re = op.regex_pattern(categories, inventory);
            for np in self.nucleus_patterns.iter() {
                let n_re = np.regex_pattern(categories, inventory);
                for cp in self.coda_patterns.iter() {
                    let c_re = cp.regex_pattern(categories, inventory);

                    ret.push(format!("^({o_re})({n_re})({c_re})$"));
                }
            }
        }

        RegexSet::new(ret).map_err(Error::from)
    }

    /// Generates a random syllable following the rule.
    pub fn generate<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        categories: &Categories,
        inventory: &Inventory,
    ) -> String {
        let onset = self
            .onset_patterns
            .choose(rng)
            .map(|p| p.generate(rng, categories, inventory))
            .unwrap_or_default();

        let nucleus = self
            .nucleus_patterns
            .choose(rng)
            .map(|p| p.generate(rng, categories, inventory))
            .unwrap_or_default();

        let coda = self
            .coda_patterns
            .choose(rng)
            .map(|p| p.generate(rng, categories, inventory))
            .unwrap_or_default();

        format!("{onset}{nucleus}{coda}")
    }
}

impl ReadXml for Phonotactic {
    type Error = Error;

    type ReaderState = Option<String>;

    const TAG: &'static str = "phonotactic";

    fn process_tag_start<R: std::io::BufRead>(
        &mut self,
        reader: &mut XmlReader<R>,
        state: &mut Self::ReaderState,
        name: String,
        attrs: Vec<(String, String)>,
    ) -> Result<(), XmlError<Self::Error>> {
        match reader.last_tag_pair() {
            (_, Some(Self::TAG)) => {}
            (Some(Self::TAG), Some("patterns")) => {
                *state = attrs
                    .iter()
                    .find(|&x| x.0 == "type")
                    .map(|x| x.1.to_owned());
            }
            (Some("patterns"), Some(Pattern::TAG)) => {
                let pat = Pattern::deserialize_xml(reader, Some((name, attrs)))?;
                match state.as_deref() {
                    Some("onset") => {
                        self.onset_patterns.push(pat);
                    }
                    Some("nucleus") => {
                        self.nucleus_patterns.push(pat);
                    }
                    Some("coda") => {
                        self.coda_patterns.push(pat);
                    }
                    _ => {}
                }
            }
            _ => return Err(XmlError::InvalidTag(name)),
        }

        Ok(())
    }

    fn process_text<R: std::io::BufRead>(
        &mut self,
        _reader: &mut XmlReader<R>,
        _state: &mut Self::ReaderState,
        _text: String,
    ) -> Result<(), XmlError<Self::Error>> {
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

impl WriteXml for Phonotactic {
    type Error = Error;

    fn serialize_xml<W: std::io::Write>(
        &self,
        writer: &mut XmlWriter<W>,
    ) -> Result<(), XmlError<Self::Error>> {
        writer.write_tag_start("phonotactic")?;

        writer.write_tag_start_with_attributes("patterns", [("type", "onset")])?;
        for p in self.onset_patterns.iter() {
            p.serialize_xml(writer)?;
        }
        writer.write_tag_end("patterns")?;

        writer.write_tag_start_with_attributes("patterns", [("type", "nucleus")])?;
        for p in self.nucleus_patterns.iter() {
            p.serialize_xml(writer)?;
        }
        writer.write_tag_end("patterns")?;

        writer.write_tag_start_with_attributes("patterns", [("type", "coda")])?;
        for p in self.coda_patterns.iter() {
            p.serialize_xml(writer)?;
        }
        writer.write_tag_end("patterns")?;

        writer.write_tag_end("phonotactic")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{phonology::Category, Phoneme};

    use super::*;

    const XML1: &str = r#"
    <phonotactic>
        <patterns type="onset">
            <pattern>
                <pattern-str></pattern-str>
            </pattern>
            <pattern>
                <pattern-str>r</pattern-str>
            </pattern>
            <pattern>
                <pattern-str>C</pattern-str>
            </pattern>
            <pattern>
                <pattern-str>Cr</pattern-str>
            </pattern>
        </patterns>
        <patterns type="nucleus">
            <pattern>
                <pattern-str>V</pattern-str>
            </pattern>
        </patterns>
        <patterns type="coda">
            <pattern>
                <pattern-str></pattern-str>
            </pattern>
            <pattern>
                <pattern-str>C</pattern-str>
            </pattern>
        </patterns>
    </phonotactic>
    "#;

    #[test]
    fn read_xml() {
        let pt = Phonotactic::load_xml_str(XML1).unwrap();

        assert_eq!(
            pt.onset_patterns()
                .iter()
                .map(|p| &p.pattern_str)
                .collect::<Vec<_>>(),
            vec!["", "r", "C", "Cr"]
        );

        assert_eq!(
            pt.nucleus_patterns()
                .iter()
                .map(|p| &p.pattern_str)
                .collect::<Vec<_>>(),
            vec!["V"]
        );

        assert_eq!(
            pt.coda_patterns()
                .iter()
                .map(|p| &p.pattern_str)
                .collect::<Vec<_>>(),
            vec!["", "C"]
        );
    }

    #[test]
    fn write_xml() {
        let pt = Phonotactic::load_xml_str(XML1).unwrap();
        let xml2 = pt.save_xml_string().unwrap();
        let pt2 = Phonotactic::load_xml_str(&xml2).unwrap();
        assert_eq!(&pt, &pt2);
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

        let pt = Phonotactic {
            onset_patterns: vec![
                Pattern::new("".to_string()),
                Pattern::new("r".to_string()),
                Pattern::new("C".to_string()),
                Pattern::new("Cr".to_string()),
            ],
            nucleus_patterns: vec![Pattern::new("V".to_string())],
            coda_patterns: vec![Pattern::new("".to_string()), Pattern::new("C".to_string())],
        };

        dbg!(pt.regex_pattern(&cats, &inv));

        let re = pt.regex(&cats, &inv).unwrap();
        assert!(re.is_match("tuk"));
        assert!(re.is_match("krik"));
        assert!(re.is_match("tran"));
        assert!(re.is_match("a"));
        assert!(re.is_match("at"));
        assert!(re.is_match("ri"));
        assert!(re.is_match("mi"));
        assert!(!re.is_match("mon"));
        assert!(!re.is_match("pen"));
        assert!(!re.is_match("trung"));
    }

    #[test]
    fn generate() {
        let (cats, inv) = test_data();
        let mut rng = rand::thread_rng();

        let pt = Phonotactic {
            onset_patterns: vec![
                Pattern::new("".to_string()),
                Pattern::new("r".to_string()),
                Pattern::new("C".to_string()),
                Pattern::new("Cr".to_string()),
            ],
            nucleus_patterns: vec![Pattern::new("V".to_string())],
            coda_patterns: vec![Pattern::new("".to_string()), Pattern::new("C".to_string())],
        };

        dbg!(pt.regex_pattern(&cats, &inv));
        let re = pt.regex(&cats, &inv).unwrap();

        for _i in 0..20 {
            let s = pt.generate(&mut rng, &cats, &inv);
            dbg!(&s);
            assert!(re.is_match(&s));
        }
    }
}
