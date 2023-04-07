use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

#[doc(hidden)]
mod imp {
    use std::cell::RefCell;

    use crate::models;

    use conlang::{ipa::IPA_BASE_PHONEMES, Phoneme};

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
                IPA_BASE_PHONEMES
                    .iter()
                    .map(|ip| Phoneme::new(*ip))
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
    pub struct  AddPhonemeListModel(ObjectSubclass<imp::AddPhonemeListModel>)
        @implements gio::ListModel;
}

impl AddPhonemeListModel {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

impl Default for AddPhonemeListModel {
    fn default() -> Self {
        Self::new()
    }
}
