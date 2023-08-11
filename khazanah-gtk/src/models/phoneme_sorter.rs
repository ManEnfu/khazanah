use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default, Debug, Clone, Copy, glib::Enum)]
#[enum_type(name = "KhzPhonemeSortBy")]
#[repr(u32)]
pub enum PhonemeSortBy {
    #[default]
    None,
    Name,
    Base,
}

#[doc(hidden)]
mod imp {
    use std::{cell::Cell, cmp};

    use gtk::prelude::Cast;

    use crate::models::PhonemeObject;

    use super::*;

    #[derive(Debug, Default)]
    pub struct PhonemeSorter {
        pub sort_by: Cell<PhonemeSortBy>,
        pub descending: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PhonemeSorter {
        const NAME: &'static str = "KhzPhonemeSorter";
        type Type = super::PhonemeSorter;
        type ParentType = gtk::Sorter;
    }

    impl ObjectImpl for PhonemeSorter {}

    impl SorterImpl for PhonemeSorter {
        fn order(&self) -> gtk::SorterOrder {
            gtk::SorterOrder::Partial
        }

        fn compare(&self, item1: &glib::Object, item2: &glib::Object) -> gtk::Ordering {
            let phoneme1 = item1
                .downcast_ref::<PhonemeObject>()
                .expect("`KhzPhonemeSorter` expected `KhzPhonemeObject` to compare.");
            let phoneme2 = item2
                .downcast_ref::<PhonemeObject>()
                .expect("`KhzPhonemeSorter` expected `KhzPhonemeObject` to compare.");

            match self.sort_by.get() {
                PhonemeSortBy::Name => self.compare_by_name(phoneme1, phoneme2),
                PhonemeSortBy::Base => {
                    let c = self.compare_by_base(phoneme1, phoneme2);
                    if c == gtk::Ordering::Equal {
                        self.compare_by_name(phoneme1, phoneme2)
                    } else {
                        c
                    }
                }
                _ => gtk::Ordering::Equal,
            }
        }
    }

    impl PhonemeSorter {
        fn compare_by_name(
            &self,
            phoneme1: &PhonemeObject,
            phoneme2: &PhonemeObject,
        ) -> gtk::Ordering {
            let v1 = phoneme1.name();
            let v2 = phoneme2.name();
            self.convert_ordering(v1.cmp(&v2))
        }

        fn compare_by_base(
            &self,
            phoneme1: &PhonemeObject,
            phoneme2: &PhonemeObject,
        ) -> gtk::Ordering {
            let v1 = phoneme1.base();
            let v2 = phoneme2.base();
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
    /// Sorter for `PhonemeObject`.
    pub struct PhonemeSorter(ObjectSubclass<imp::PhonemeSorter>)
        @extends gtk::Sorter;
}

impl PhonemeSorter {
    /// Creates a new sorter.
    pub fn new(sort_by: PhonemeSortBy) -> Self {
        let obj = glib::Object::builder::<Self>().build();
        obj.imp().sort_by.set(sort_by);
        obj
    }

    /// Sets the criteria of the sort.
    pub fn set_sort_by(&self, sort_by: PhonemeSortBy) {
        self.imp().sort_by.set(sort_by);
        self.changed(gtk::SorterChange::Different);
    }

    /// Gets the criteria of the sort.
    pub fn sort_by(&self) -> PhonemeSortBy {
        self.imp().sort_by.get()
    }

    pub fn set_descending(&self, descending: bool) {
        if self.imp().descending.get() != descending {
            self.imp().descending.set(descending);
            self.changed(gtk::SorterChange::Inverted);
        }
    }
}
