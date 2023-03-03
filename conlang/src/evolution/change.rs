use crate::lexicon::word::Word;

pub trait WordChange {
    fn will_apply(&self, word: &Word) -> bool;
    fn apply(&self, word: &Word) -> Word;
}

pub trait WordChangeBuilder {
    type Type;

    fn build(&self) -> Self::Type;
}
