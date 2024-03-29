use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

#[doc(hidden)]
mod imp {
    use std::cell::RefCell;

    use crate::models;

    use khazanah_core::{Ipa, Phoneme};

    use super::*;

    #[derive(Debug, Default)]
    pub struct AddPhonemeListModel {
        pub list: RefCell<Vec<models::AddPhonemeObject>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AddPhonemeListModel {
        const NAME: &'static str = "KhzAddPhonemeListModel";
        type Type = super::AddPhonemeListModel;
        type Interfaces = (gio::ListModel,);
    }

    impl ObjectImpl for AddPhonemeListModel {
        fn constructed(&self) {
            self.parent_constructed();
            self.list.replace(
                Ipa::iter_valids()
                    .map(Phoneme::with_ipa)
                    .map(models::AddPhonemeObject::new)
                    .collect(),
            );
        }
    }

    impl ListModelImpl for AddPhonemeListModel {
        fn item_type(&self) -> glib::Type {
            models::AddPhonemeObject::static_type()
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            self.list
                .borrow()
                .get(position as usize)
                .map(|it| it.clone().upcast())
        }

        fn n_items(&self) -> u32 {
            self.list.borrow().len() as u32
        }
    }
}

glib::wrapper! {
    /// List of base phonemes available.
    pub struct  AddPhonemeListModel(ObjectSubclass<imp::AddPhonemeListModel>)
        @implements gio::ListModel;
}

impl AddPhonemeListModel {
    /// Creates new list.
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

impl Default for AddPhonemeListModel {
    fn default() -> Self {
        Self::new()
    }
}
