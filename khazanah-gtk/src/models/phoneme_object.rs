use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use khazanah_core::Ipa;

use crate::models;
use khazanah_core::Phoneme;
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
    #[properties(wrapper_types = super::PhonemeObject)]
    pub struct PhonemeObject {
        #[property(name = "name", type = String,
            get = Self::get_name)]
        #[property(name = "sound", type = String,
            get = Self::get_sound, set = Self::set_sound)]
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
                Some(Inner::QueryFromProject {
                    project_model: _,
                    id: _,
                }) => todo!(),
                None => T::default(),
            }
        }

        fn update<F>(&self, f: F)
        where
            F: Fn(&mut Phoneme),
        {
            match self.inner.borrow_mut().as_mut() {
                Some(Inner::Owned(phoneme)) => f(phoneme),
                Some(Inner::QueryFromProject {
                    project_model: _,
                    id: _,
                }) => todo!(),
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
        }

        pub fn get_base(&self) -> Option<Ipa> {
            self.query(|phoneme| phoneme.base())
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
}
