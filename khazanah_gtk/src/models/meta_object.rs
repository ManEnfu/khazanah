use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::models;

#[doc(hidden)]
mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_types = super::MetaObject)]
    pub struct MetaObject {
        #[property(name = "name", type = String,
            get = Self::get_name, set = Self::set_name)]
        #[property(name = "local-language", type = String,
            get = Self::get_local_lang, set = Self::set_local_lang)]
        #[property(name = "author", type = String,
            get = Self::get_author, set = Self::set_author)]
        #[property(name = "description", type = String,
            get = Self::get_description, set = Self::set_description)]
        #[property(get, set)]
        pub project_model: RefCell<models::ProjectModel>,
    }

    impl MetaObject {
        fn get_name(&self) -> String {
            self.project_model
                .borrow()
                .query(|project| project.meta().name.to_owned())
                .unwrap_or_default()
        }

        fn set_name(&self, value: String) {
            self.project_model.borrow_mut().update(|project| {
                project.meta_mut().name = value.clone();
            });
        }

        fn get_local_lang(&self) -> String {
            self.project_model
                .borrow()
                .query(|project| project.meta().local_lang.to_owned())
                .unwrap_or_default()
        }

        fn set_local_lang(&self, value: String) {
            self.project_model.borrow_mut().update(|project| {
                project.meta_mut().local_lang = value.clone();
            });
        }

        fn get_author(&self) -> String {
            self.project_model
                .borrow()
                .query(|project| project.meta().author.to_owned())
                .unwrap_or_default()
        }

        fn set_author(&self, value: String) {
            self.project_model.borrow_mut().update(|project| {
                project.meta_mut().author = value.clone();
            });
        }

        fn get_description(&self) -> String {
            self.project_model
                .borrow()
                .query(|project| project.meta().description.to_owned())
                .unwrap_or_default()
        }

        fn set_description(&self, value: String) {
            self.project_model.borrow_mut().update(|project| {
                project.meta_mut().description = value.clone();
            });
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MetaObject {
        const NAME: &'static str = "KhzMetaObject";
        type Type = super::MetaObject;
    }

    impl ObjectImpl for MetaObject {
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
    /// Wrapper for `Meta` data structure in a `Project`.
    pub struct MetaObject(ObjectSubclass<imp::MetaObject>);
}

impl MetaObject {
    /// Creates a new meta object.
    pub fn new(project_model: models::ProjectModel) -> Self {
        let obj: Self = glib::Object::builder()
            .property("project-model", project_model)
            .build();
        obj
    }
}

impl Default for MetaObject {
    fn default() -> Self {
        Self::new(models::ProjectModel::new())
    }
}
