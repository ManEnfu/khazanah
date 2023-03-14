use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default, Debug, Clone, Copy, glib::Enum)]
#[enum_type(name = "KhzWordSortBy")]
#[repr(u32)]
pub enum WordSortBy {
    #[default]
    None = 0,
    Romanization = 1,
    Translation = 2,
    PartOfSpeech = 3,
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

        fn convert_ordering(&self, ordering: cmp::Ordering) -> gtk::Ordering {
            match ordering {
                cmp::Ordering::Less => gtk::Ordering::Smaller,
                cmp::Ordering::Greater => gtk::Ordering::Larger,
                cmp::Ordering::Equal => gtk::Ordering::Equal,
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
}
