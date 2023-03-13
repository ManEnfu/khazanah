use conlang::{Word, ALL_PARTS_OF_SPEECH};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use uuid::Uuid;

use crate::models;

#[doc(hidden)]
mod imp {
    use std::cell::{Cell, RefCell};

    use uuid::Uuid;

    use crate::models;

    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_types = super::WordObject)]
    pub struct WordObject {
        #[property(name = "romanization", type = String,
            get = Self::get_romanization, set = Self::set_romanization)]
        #[property(name = "translation", type = String,
            get = Self::get_translation, set = Self::set_translation)]
        #[property(name = "pronunciation", type = String,
            get = Self::get_pronunciation, set = Self::set_pronunciation)]
        #[property(name = "part-of-speech", type = u32,
            get = Self::get_pos, set = Self::set_pos)]
        #[property(name = "part-of-speech-label", 
            get = Self::get_pos_label, type = String)]
        pub data: RefCell<Word>,

        #[property(get, set, construct_only)]
        pub project_model: RefCell<models::ProjectModel>,

        pub id: Cell<Uuid>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WordObject {
        const NAME: &'static str = "KhzWordObject";
        type Type = super::WordObject;
    }

    impl WordObject {
        fn get_word_property<T, F>(&self, f: F) -> T
        where
            T: Default,
            F: Fn(&Word) -> T,
        {
            if let Some(project) = self.project_model.borrow().project().as_ref() {
                if let Some(word) = project.lexicon().word_by_id(&self.id.get()) {
                    return f(word);
                }
            }
            Default::default()
        }

        fn set_word_property<F, T>(&self, value: T, f: F)
        where
            F: Fn(&mut Word, T),
        {
            if let Some(project) = self.project_model.borrow().project_mut().as_mut() {
                if let Some(word) = project.lexicon_mut().word_by_id_mut(&self.id.get()) {
                    f(word, value);
                }
            }
        }

        fn get_romanization(&self) -> String {
            self.get_word_property(|word| word.romanization.to_owned())
        }

        fn set_romanization(&self, value: String) {
            self.set_word_property(value, |word, value| word.romanization = value);
        }

        fn get_translation(&self) -> String {
            self.get_word_property(|word| word.translation.to_owned())
        }

        fn set_translation(&self, value: String) {
            self.set_word_property(value, |word, value| word.translation = value);
        }

        fn get_pronunciation(&self) -> String {
            self.get_word_property(|word| word.pronunciation.to_owned())
        }

        fn set_pronunciation(&self, value: String) {
            self.set_word_property(value, |word, value| word.pronunciation = value);
        }

        fn get_pos(&self) -> u32 {
            self.get_word_property(|word| {
                ALL_PARTS_OF_SPEECH
                    .iter()
                    .position(|&x| x == word.part_of_speech)
                    .unwrap_or_default() as u32
            })
        }

        fn set_pos(&self, value: u32) {
            self.set_word_property(value, |word, value| {
                word.part_of_speech = ALL_PARTS_OF_SPEECH
                    .get(value as usize)
                    .copied()
                    .unwrap_or_default();
            });
            self.obj().notify_part_of_speech_label();
        }

        fn get_pos_label(&self) -> String {
            self.get_word_property(|word| {
                word.part_of_speech
                    .map(|s| s.label().to_owned())
                    .unwrap_or_default()
            })
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
    /// Wrapper for `Word` data structure in a `Project`.
    pub struct WordObject(ObjectSubclass<imp::WordObject>);
}

impl WordObject {
    /// Creates a new word object.
    pub fn new(project_model: models::ProjectModel, id: Uuid) -> Self {
        let obj: Self = glib::Object::builder()
            .property("project-model", project_model)
            .build();
        obj.imp().id.set(id);
        obj
    }

    /// Returs the id of the object.
    pub fn id(&self) -> Uuid {
        self.imp().id.get()
    }
}
