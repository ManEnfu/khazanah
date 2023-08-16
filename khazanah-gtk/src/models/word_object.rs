use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use khazanah_core::prelude::*;
use khazanah_core::{Word, ALL_PARTS_OF_SPEECH};
use uuid::Uuid;

use crate::models;

#[doc(hidden)]
mod imp {
    use std::cell::RefCell;

    use uuid::Uuid;

    use super::*;

    #[derive(Debug)]
    pub enum Inner {
        Owned(Word),
        QueryFromProject {
            project_model: models::ProjectModel,
            id: Uuid,
        },
    }

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::WordObject)]
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
        #[property(name = "use-xsampa", type = bool,
            get = Self::get_use_xsampa, set = Self::set_use_xsampa)]
        #[property(name = "xsampa-pronunciation", type = String,
            get = Self::get_xsampa_pronunciation, set = Self::set_xsampa_pronunciation)]
        pub inner: RefCell<Option<Inner>>,
    }

    impl WordObject {
        fn query<T, F>(&self, f: F) -> T
        where
            T: Default,
            F: Fn(&Word) -> T,
        {
            match self.inner.borrow().as_ref() {
                Some(Inner::Owned(word)) => f(word),
                Some(Inner::QueryFromProject { project_model, id }) => project_model
                    .query(|project| project.language().dictionary().word_by_id(*id).map(&f))
                    .flatten()
                    .unwrap_or_default(),
                None => T::default(),
            }
        }

        fn update<F>(&self, f: F)
        where
            F: Fn(&mut Word),
        {
            match self.inner.borrow_mut().as_mut() {
                Some(Inner::Owned(word)) => f(word),
                Some(Inner::QueryFromProject { project_model, id }) => {
                    project_model.update(|project| {
                        project
                            .language_mut()
                            .dictionary_mut()
                            .word_by_id_mut(*id)
                            .map(&f)
                    });
                }
                None => {}
            }
        }

        fn get_romanization(&self) -> String {
            self.query(|word| word.romanization().to_string())
        }

        fn set_romanization(&self, value: String) {
            self.update(|word| word.set_romanization(value.clone()));
        }

        fn get_translation(&self) -> String {
            self.query(|word| word.translation().to_string())
        }

        fn set_translation(&self, value: String) {
            self.update(|word| word.set_translation(value.clone()));
        }

        fn get_pronunciation(&self) -> String {
            self.query(|word| word.pronunciation().to_string())
        }

        fn set_pronunciation(&self, value: String) {
            self.update(|word| word.set_pronunciation(value.clone()));
        }

        fn get_pos(&self) -> u32 {
            self.query(|word| {
                ALL_PARTS_OF_SPEECH
                    .iter()
                    .position(|&x| x == word.part_of_speech())
                    .unwrap_or_default() as u32
            })
        }

        fn set_pos(&self, value: u32) {
            self.update(|word| {
                word.set_part_of_speech(
                    ALL_PARTS_OF_SPEECH
                        .get(value as usize)
                        .copied()
                        .unwrap_or_default(),
                );
            });
            self.obj().notify_part_of_speech_label();
        }

        fn get_pos_label(&self) -> String {
            self.query(|word| {
                word.part_of_speech()
                    .map(|s| s.label().to_string())
                    .unwrap_or_default()
            })
        }

        fn get_xsampa_pronunciation(&self) -> String {
            self.query(|word| word.xsampa_pronunciation().unwrap_or_default().to_string())
        }

        fn set_xsampa_pronunciation(&self, value: String) {
            self.update(|word| {
                if word.xsampa_pronunciation().is_some() {
                    word.set_xsampa_pronunciation(Some(value.clone()));
                }
            });
            self.obj().notify_pronunciation();
        }

        fn get_use_xsampa(&self) -> bool {
            self.query(|word| word.xsampa_pronunciation().is_some())
        }

        fn set_use_xsampa(&self, value: bool) {
            self.update(|word| {
                if value {
                    if word.xsampa_pronunciation().is_none() {
                        word.set_xsampa_pronunciation(Some("".to_string()));
                    }
                } else {
                    word.set_xsampa_pronunciation(None);
                }
            });
            let obj = self.obj();
            obj.notify_xsampa_pronunciation();
            obj.notify_pronunciation();
        }

        pub fn get_id(&self) -> Uuid {
            if let Some(Inner::QueryFromProject { id, .. }) = self.inner.borrow().as_ref() {
                *id
            } else {
                self.query(|word| word.id().unwrap_or_default())
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WordObject {
        const NAME: &'static str = "KhzWordObject";
        type Type = super::WordObject;
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
    pub fn query_project(project_model: models::ProjectModel, id: Uuid) -> Self {
        let obj = glib::Object::builder::<Self>().build();
        obj.imp()
            .inner
            .replace(Some(imp::Inner::QueryFromProject { project_model, id }));
        obj
    }

    /// Returs the id of the object.
    pub fn id(&self) -> Uuid {
        self.imp().get_id()
    }
}
