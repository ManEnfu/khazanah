use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

use crate::models::PhonemeObject;

use uuid::Uuid;

#[doc(hidden)]
mod imp {
    use std::cell::{Cell, RefCell};

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::PhonemeListRow)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/view/inventory/phoneme_list_row.ui")]
    pub struct PhonemeListRow {
        #[template_child]
        pub sound_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub romanization_label: TemplateChild<gtk::Label>,

        #[property(get, set)]
        pub reveal_action_buttons: Cell<bool>,

        pub id: Cell<Uuid>,

        pub bindings: RefCell<Vec<glib::Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PhonemeListRow {
        const NAME: &'static str = "KhzInventoryViewPhonemeListRow";
        type Type = super::PhonemeListRow;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PhonemeListRow {
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

    impl WidgetImpl for PhonemeListRow {}

    impl BinImpl for PhonemeListRow {}
}

glib::wrapper! {
    pub struct PhonemeListRow(ObjectSubclass<imp::PhonemeListRow>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl PhonemeListRow {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn bind(&self, phoneme_object: &PhonemeObject) {
        let imp = self.imp();
        let sound_label = imp.sound_label.get();
        let romanization_label = imp.romanization_label.get();

        let mut bindings = imp.bindings.borrow_mut();

        bindings.push(
            phoneme_object
                .bind_property("sound", &sound_label, "label")
                .sync_create()
                .transform_to(|_, s: Option<String>| {
                    Some(s.map_or("(New phoneme)".to_string(), |s| format!("/{}/", s)))
                })
                .build(),
        );

        bindings.push(
            phoneme_object
                .bind_property("romanization", &romanization_label, "label")
                .sync_create()
                .transform_to(|_, s: Option<String>| {
                    Some(s.map_or(String::default(), |s| format!("<{}>", s)))
                })
                .build(),
        );

        imp.id.set(phoneme_object.id());
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }

    #[template_callback]
    pub fn handle_delete_button(&self, _button: &gtk::Button) {
        let id = self.imp().id.get();
        self.activate_action(
            "inventory.delete-phoneme",
            Some(&glib::Variant::from(id.to_string())),
        )
        .unwrap_or_default();
    }
}

impl Default for PhonemeListRow {
    fn default() -> Self {
        Self::new()
    }
}
