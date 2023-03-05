use conlang::{PartOfSpeech, Word};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use glib::Properties;
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::WordObject)]
    pub struct WordObject {
        #[property(name = "romanization", get, set, type = String, member = romanization)]
        #[property(name = "pronunciation", get, set, type = String, member = pronunciation)]
        #[property(name = "translation", get, set, type = String, member = translation)]
        #[property(name = "part-of-speech", type = String,
            get = Self::part_of_speech, set = Self::set_part_of_speech)]
        #[property(name = "part-of-speech-label", type = String,
            get = Self::part_of_speech_label)]
        pub data: Rc<RefCell<Word>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WordObject {
        const NAME: &'static str = "KhzWordObject";
        type Type = super::WordObject;
    }

    impl WordObject {
        fn part_of_speech(&self) -> String {
            self.data
                .borrow()
                .part_of_speech
                .clone()
                .map(|s| s.into())
                .unwrap_or_default()
        }

        fn set_part_of_speech(&self, v: Option<String>) {
            self.data.borrow_mut().part_of_speech = v.map(|s| s.as_str().into());
        }

        fn part_of_speech_label(&self) -> String {
            self.data
                .borrow()
                .part_of_speech
                .clone()
                .map(|s| s.label().to_owned())
                .unwrap_or_default()
        }
    }

    impl ObjectImpl for WordObject {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }
    }
}

glib::wrapper! {
    /// A `GObject` that stores word data.
    pub struct WordObject(ObjectSubclass<imp::WordObject>);
}

impl WordObject {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn pos(&self) -> Option<PartOfSpeech> {
        self.imp().data.borrow().part_of_speech.clone()
    }

    pub fn set_pos(&self, pos: Option<PartOfSpeech>) {
        self.imp().data.borrow_mut().part_of_speech = pos;
    }
}

impl Default for WordObject {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Word> for WordObject {
    fn from(value: Word) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.imp().data.replace(value);
        obj
    }
}
