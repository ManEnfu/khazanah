use super::change::WordChange;
use crate::lexicon::word::Word;

use regex::{Captures, Regex};

pub struct Substitution {
    re: Regex,
    repl: String,
}

impl Substitution {
    pub fn new(
        pattern: &str,
        repl: &str,
        env_left: &str,
        env_right: &str,
    ) -> Result<Self, regex::Error> {
        let re_pattern = format!("({})({})({})", env_left, pattern, env_right);
        let re = Regex::new(&re_pattern)?;
        Ok(Self {
            re,
            repl: repl.to_owned(),
        })
    }
}

impl WordChange for Substitution {
    fn will_apply(&self, word: &Word) -> bool {
        !word.pronunciation.override_value
    }

    fn apply(&self, word: &Word) -> Word {
        let new_pr = self.re.replace(&word.romanization.value, |c: &Captures| {
            format!("{}{}{}", &c[1], self.repl, &c[3])
        });

        let mut ret = word.clone();
        ret.romanization.value = new_pr.to_string();
        ret
    }
}
