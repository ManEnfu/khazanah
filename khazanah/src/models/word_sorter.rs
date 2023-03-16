use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default, Debug, Clone, Copy, glib::Enum)]
#[enum_type(name = "KhzWordSortBy")]
#[repr(u32)]
pub enum WordSortBy {
    #[default]
    None,
    Romanization,
    Translation,
    Pronunciation,
    PartOfSpeech,
}

#[doc(hidden)]
mod imp {
    use std::{cell::Cell, cmp};

    use gtk::prelude::Cast;

    use crate::models::WordObject;

    use super::*;

    #[derive(Debug, Default)]
    pub struct WordSorter {
        pub sort_by: Cell<WordSortBy>,
        pub descending: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WordSorter {
        const NAME: &'static str = "KhzWordSorter";
        type Type = super::WordSorter;
        type ParentType = gtk::Sorter;
    }

    impl ObjectImpl for WordSorter {}

    impl SorterImpl for WordSorter {
        fn order(&self) -> gtk::SorterOrder {
            gtk::SorterOrder::Partial
        }

        fn compare(&self, item1: &glib::Object, item2: &glib::Object) -> gtk::Ordering {
            let word1 = item1
                .downcast_ref::<WordObject>()
                .expect("`KhzWordSorter` expected `KhzWordObject` to compare.");
            let word2 = item2
                .downcast_ref::<WordObject>()
                .expect("`KhzWordSorter` expected `KhzWordObject` to compare.");

            match self.sort_by.get() {
                WordSortBy::Romanization => self.compare_by_romanization(word1, word2),
                WordSortBy::Translation => self.compare_by_translation(word1, word2),
                WordSortBy::Pronunciation => self.compare_by_pronunciation(word1, word2),
                WordSortBy::PartOfSpeech => self.compare_by_part_of_speech(word1, word2),
                _ => gtk::Ordering::Equal,
            }
        }
    }

    impl WordSorter {
        fn compare_by_romanization(&self, word1: &WordObject, word2: &WordObject) -> gtk::Ordering {
            let v1 = word1.romanization();
            let v2 = word2.romanization();
            self.convert_ordering(v1.cmp(&v2))
        }

        fn compare_by_translation(&self, word1: &WordObject, word2: &WordObject) -> gtk::Ordering {
            let v1 = word1.translation();
            let v2 = word2.translation();
            self.convert_ordering(v1.cmp(&v2))
        }

        fn compare_by_pronunciation(
            &self,
            word1: &WordObject,
            word2: &WordObject,
        ) -> gtk::Ordering {
            let v1 = word1.pronunciation();
            let v2 = word2.pronunciation();
            self.convert_ordering(v1.cmp(&v2))
        }

        fn compare_by_part_of_speech(
            &self,
            word1: &WordObject,
            word2: &WordObject,
        ) -> gtk::Ordering {
            let v1 = word1.part_of_speech();
            let v2 = word2.part_of_speech();
            self.convert_ordering(v1.cmp(&v2))
        }

        fn convert_ordering(&self, ordering: cmp::Ordering) -> gtk::Ordering {
            if self.descending.get() {
                match ordering {
                    cmp::Ordering::Less => gtk::Ordering::Larger,
                    cmp::Ordering::Greater => gtk::Ordering::Smaller,
                    cmp::Ordering::Equal => gtk::Ordering::Equal,
                }
            } else {
                match ordering {
                    cmp::Ordering::Less => gtk::Ordering::Smaller,
                    cmp::Ordering::Greater => gtk::Ordering::Larger,
                    cmp::Ordering::Equal => gtk::Ordering::Equal,
                }
            }
        }
    }
}

glib::wrapper! {
    /// Sorter for `WordObject`.
    pub struct WordSorter(ObjectSubclass<imp::WordSorter>)
        @extends gtk::Sorter;
}

impl WordSorter {
    /// Creates a new sorter.
    pub fn new(sort_by: WordSortBy) -> Self {
        let obj = glib::Object::builder::<Self>().build();
        obj.imp().sort_by.set(sort_by);
        obj
    }

    /// Sets the criteria of the sort.
    pub fn set_sort_by(&self, sort_by: WordSortBy) {
        self.imp().sort_by.set(sort_by);
        self.changed(gtk::SorterChange::Different);
    }

    /// Gets the criteria of the sort.
    pub fn sort_by(&self) -> WordSortBy {
        self.imp().sort_by.get()
    }

    pub fn set_descending(&self, descending: bool) {
        if self.imp().descending.get() != descending {
            self.imp().descending.set(descending);
            self.changed(gtk::SorterChange::Inverted);
        }
    }
}
