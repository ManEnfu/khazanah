use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

// use adw::prelude::*;
use adw::subclass::prelude::*;

use crate::models::AddPhonemeObject;

#[doc(hidden)]
mod imp {
    use std::cell::{Cell, RefCell};

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::AddPhonemeRow)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/add_phoneme_row.ui")]
    pub struct AddPhonemeRow {
        #[template_child]
        pub name_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub symbol_label: TemplateChild<gtk::Label>,

        #[property(get, set)]
        pub reveal_action_buttons: Cell<bool>,

        pub bindings: RefCell<Vec<glib::Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AddPhonemeRow {
        const NAME: &'static str = "KhzAddPhonemeRow";
        type Type = super::AddPhonemeRow;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AddPhonemeRow {
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

    impl WidgetImpl for AddPhonemeRow {}

    impl BinImpl for AddPhonemeRow {}
}

glib::wrapper! {
    /// Row widget for `AddPhonemeView`.
    pub struct AddPhonemeRow(ObjectSubclass<imp::AddPhonemeRow>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl AddPhonemeRow {
    /// Creates a new list row.
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    /// Binds widget to a word object.
    pub fn bind(&self, phoneme_object: &AddPhonemeObject) {
        let mut bindings = self.imp().bindings.borrow_mut();
        let imp = self.imp();

        bindings.push(
            phoneme_object
                .bind_property("name", &*imp.name_label, "label")
                .sync_create()
                .build(),
        );

        bindings.push(
            phoneme_object
                .bind_property("symbol", &*imp.symbol_label, "label")
                .sync_create()
                .build(),
        );
    }

    /// Unbinds widget.
    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}

impl Default for AddPhonemeRow {
    fn default() -> Self {
        Self::new()
    }
}
