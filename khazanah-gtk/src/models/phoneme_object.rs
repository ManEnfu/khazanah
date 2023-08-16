use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use khazanah_core::prelude::*;

use crate::models;
use khazanah_core::{Ipa, Phoneme};
use uuid::Uuid;

#[doc(hidden)]
mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug)]
    pub enum Inner {
        Owned(Phoneme),
        QueryFromProject {
            project_model: models::ProjectModel,
            id: Uuid,
        },
    }

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::PhonemeObject)]
    pub struct PhonemeObject {
        #[property(name = "name", type = String,
            get = Self::get_name)]
        #[property(name = "sound", type = String,
            get = Self::get_sound, set = Self::set_sound)]
        #[property(name = "use-xsampa", type = bool,
            get = Self::get_use_xsampa, set = Self::set_use_xsampa)]
        #[property(name = "xsampa-sound", type = String,
            get = Self::get_xsampa_sound, set = Self::set_xsampa_sound)]
        #[property(name = "romanization", type = String,
            get = Self::get_romanization, set = Self::set_romanization)]
        #[property(name = "mora", type = u32,
            get = Self::get_mora, set = Self::set_mora)]
        #[property(name = "display-romanization", type = String,
            get = Self::get_display_romanization)]
        #[property(name = "base-symbol", type = String,
            get = Self::get_base_symbol)]
        pub inner: RefCell<Option<Inner>>,
    }

    impl PhonemeObject {
        fn query<T, F>(&self, f: F) -> T
        where
            T: Default,
            F: Fn(&Phoneme) -> T,
        {
            match self.inner.borrow().as_ref() {
                Some(Inner::Owned(phoneme)) => f(phoneme),
                Some(Inner::QueryFromProject { project_model, id }) => project_model
                    .query(|project| {
                        project
                            .language()
                            .phonemic_inventory()
                            .phoneme_by_id(*id)
                            .map(&f)
                    })
                    .flatten()
                    .unwrap_or_default(),
                None => T::default(),
            }
        }

        fn update<F>(&self, f: F)
        where
            F: Fn(&mut Phoneme),
        {
            match self.inner.borrow_mut().as_mut() {
                Some(Inner::Owned(phoneme)) => f(phoneme),
                Some(Inner::QueryFromProject { project_model, id }) => {
                    project_model.update(|project| {
                        project
                            .language_mut()
                            .phonemic_inventory_mut()
                            .phoneme_by_id_mut(*id)
                            .map(&f)
                    });
                }
                None => {}
            }
        }

        fn get_name(&self) -> String {
            self.query(|phoneme| phoneme.base().map(|x| x.name()).unwrap_or_default())
        }

        fn get_sound(&self) -> String {
            self.query(|phoneme| phoneme.sound().to_string())
        }

        fn set_sound(&self, value: String) {
            self.update(|phoneme| phoneme.set_sound(value.clone()));
            self.obj().notify_display_romanization();
            self.obj().notify_name();
            self.obj().notify_base_symbol();
        }

        fn get_romanization(&self) -> String {
            self.query(|phoneme| phoneme.romanization().unwrap_or_default().to_string())
        }

        fn set_romanization(&self, value: String) {
            if value.is_empty() {
                self.update(|phoneme| phoneme.set_romanization(None));
            } else {
                self.update(|phoneme| phoneme.set_romanization(Some(value.clone())));
            }
            self.obj().notify_display_romanization();
        }

        fn get_display_romanization(&self) -> String {
            self.query(|phoneme| phoneme.display_romanization().to_string())
        }

        pub fn get_mora(&self) -> u32 {
            self.query(|phoneme| phoneme.mora())
        }

        pub fn set_mora(&self, value: u32) {
            self.update(|phoneme| phoneme.set_mora(value))
        }

        pub fn get_base(&self) -> Option<Ipa> {
            self.query(|phoneme| phoneme.base())
        }

        fn get_base_symbol(&self) -> String {
            self.query(|phoneme| {
                phoneme
                    .base()
                    .map(|x| x.symbol_with_placeholder())
                    .unwrap_or_default()
            })
        }

        fn get_xsampa_sound(&self) -> String {
            self.query(|phoneme| phoneme.xsampa_sound().unwrap_or_default().to_string())
        }

        fn set_xsampa_sound(&self, value: String) {
            self.update(|phoneme| {
                if phoneme.xsampa_sound().is_some() {
                    phoneme.set_xsampa_sound(Some(value.clone()));
                }
            });
            self.obj().notify_sound();
            self.obj().notify_display_romanization();
            self.obj().notify_name();
            self.obj().notify_base_symbol();
        }

        fn get_use_xsampa(&self) -> bool {
            self.query(|phoneme| phoneme.xsampa_sound().is_some())
        }

        fn set_use_xsampa(&self, value: bool) {
            self.update(|phoneme| {
                if value {
                    if phoneme.xsampa_sound().is_none() {
                        phoneme.set_xsampa_sound(Some("".to_string()));
                    }
                } else {
                    phoneme.set_xsampa_sound(None);
                }
            });
            let obj = self.obj();
            obj.notify_xsampa_sound();
            obj.notify_sound();
        }

        pub fn get_id(&self) -> Uuid {
            if let Some(Inner::QueryFromProject { id, .. }) = self.inner.borrow().as_ref() {
                *id
            } else {
                self.query(|p| p.id().unwrap_or_default())
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PhonemeObject {
        const NAME: &'static str = "KhzPhonemeObject";
        type Type = super::PhonemeObject;
    }

    impl ObjectImpl for PhonemeObject {
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
    pub struct PhonemeObject(ObjectSubclass<imp::PhonemeObject>);
}

impl PhonemeObject {
    pub fn from_phoneme(phoneme: Phoneme) -> Self {
        let obj = glib::Object::builder::<Self>().build();
        obj.imp().inner.replace(Some(imp::Inner::Owned(phoneme)));
        obj
    }

    pub fn query_project(project_model: models::ProjectModel, id: Uuid) -> Self {
        let obj = glib::Object::builder::<Self>().build();
        obj.imp()
            .inner
            .replace(Some(imp::Inner::QueryFromProject { project_model, id }));
        obj
    }

    pub fn base(&self) -> Option<Ipa> {
        self.imp().get_base()
    }

    pub fn id(&self) -> Uuid {
        self.imp().get_id()
    }
}
