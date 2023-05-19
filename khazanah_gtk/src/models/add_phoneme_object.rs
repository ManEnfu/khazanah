use khazanah_core::Phoneme;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[doc(hidden)]
mod imp {
    use std::cell::RefCell;

    use khazanah_core::Phoneme;

    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_types = super::AddPhonemeObject)]
    pub struct AddPhonemeObject {
        #[property(name = "name", type = String,
            get = Self::get_name)]
        #[property(name = "symbol", type = String,
            get = Self::get_symbol)]
        pub phoneme: RefCell<Option<Phoneme>>,
    }

    impl AddPhonemeObject {
        fn get_name(&self) -> String {
            self.phoneme
                .borrow()
                .as_ref()
                .map(|x| x.base.to_string())
                .unwrap_or_default()
        }

        fn get_symbol(&self) -> String {
            self.phoneme
                .borrow()
                .as_ref()
                .and_then(|x| x.base.symbol())
                .unwrap_or_default()
                .to_string()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AddPhonemeObject {
        const NAME: &'static str = "KhzAddPhonemeObject";
        type Type = super::AddPhonemeObject;
    }

    impl ObjectImpl for AddPhonemeObject {
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
    pub struct AddPhonemeObject(ObjectSubclass<imp::AddPhonemeObject>);
}

impl AddPhonemeObject {
    pub fn new(phoneme: Phoneme) -> Self {
        let obj = glib::Object::builder::<Self>().build();
        obj.imp().phoneme.replace(Some(phoneme));
        obj
    }
}
