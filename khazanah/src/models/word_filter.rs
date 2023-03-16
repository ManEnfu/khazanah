use conlang::PartOfSpeech;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default, Debug, Clone)]
pub enum WordFilterBy {
    #[default]
    None,
    Romanization(String),
    Translation(String),
    Pronunciation(String),
    PartOfSpeech(PartOfSpeech),
}

#[doc(hidden)]
mod imp {
    use std::cell::RefCell;

    use crate::models::WordObject;

    use super::*;

    #[derive(Debug, Default)]
    pub struct WordFilter {
        pub filter_by: RefCell<WordFilterBy>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WordFilter {
        const NAME: &'static str = "KhzWordFilter";
        type Type = super::WordFilter;
        type ParentType = gtk::Filter;
    }

    impl ObjectImpl for WordFilter {}

    impl FilterImpl for WordFilter {
        fn match_(&self, item: &glib::Object) -> bool {
            let word = item
                .downcast_ref::<WordObject>()
                .expect("`KhzWordFilter` expected `KhzWordObject` to match.");
            match &*self.filter_by.borrow() {
                WordFilterBy::Romanization(s) => word
                    .romanization()
                    .to_lowercase()
                    .contains(&s.to_lowercase()),
                WordFilterBy::Translation(s) => word
                    .translation()
                    .to_lowercase()
                    .contains(&s.to_lowercase()),
                WordFilterBy::Pronunciation(s) => word.pronunciation().contains(s),
                WordFilterBy::None => true,
                _ => false,
            }
        }

        fn strictness(&self) -> gtk::FilterMatch {
            gtk::FilterMatch::Some
        }
    }
}

glib::wrapper! {
    /// Filter for `WordObject`.
    pub struct WordFilter(ObjectSubclass<imp::WordFilter>)
        @extends gtk::Filter;
}

impl WordFilter {
    /// Creates a new filter.
    pub fn new(filter_by: WordFilterBy) -> Self {
        let obj = glib::Object::builder::<Self>().build();
        obj.imp().filter_by.replace(filter_by);
        obj
    }

    /// Sets the criteria of the filter.
    pub fn set_filter_by(&self, filter_by: WordFilterBy) {
        self.imp().filter_by.replace(filter_by);
        self.changed(gtk::FilterChange::Different);
    }
}
