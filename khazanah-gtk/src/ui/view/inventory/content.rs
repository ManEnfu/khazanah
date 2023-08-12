use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

use crate::models;
use crate::ui;

#[doc(hidden)]
#[allow(clippy::enum_variant_names)]
mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::glib::subclass::Signal;
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::Content)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/view/inventory/content.ui")]
    pub struct Content {
        #[template_child]
        pub sound_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub xsampa_toggle_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub xsampa_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub romanization_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub mora_entry: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub mora_adj: TemplateChild<gtk::Adjustment>,

        #[property(get, set)]
        pub project_model: RefCell<models::ProjectModel>,

        #[property(get, set)]
        pub fields_sensitive: Cell<bool>,

        pub form_bindings: RefCell<Vec<glib::Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Content {
        const NAME: &'static str = "KhzInventoryViewContent";
        type Type = super::Content;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Content {
        fn constructed(&self) {
            self.parent_constructed();
        }

        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(Vec::new);
            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for Content {}
    impl BinImpl for Content {}
}

glib::wrapper! {
    pub struct Content(ObjectSubclass<imp::Content>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl Content {
    pub fn select_phoneme(&self, phoneme: Option<&models::PhonemeObject>) {
        self.unbind();

        if let Some(phoneme) = phoneme {
            self.bind(phoneme);
            self.set_fields_sensitive(true);
        } else {
            self.clear_fields();
            self.set_fields_sensitive(false);
        }
    }

    /// Binds a phoneme to form.
    pub fn bind(&self, phoneme: &models::PhonemeObject) {
        let imp = self.imp();
        let mut bindings = imp.form_bindings.borrow_mut();

        bindings.push(
            phoneme
                .bind_property("sound", &imp.sound_entry.get(), "text")
                .sync_create()
                .bidirectional()
                .build(),
        );

        bindings.push(
            phoneme
                .bind_property("use-xsampa", &imp.xsampa_toggle_button.get(), "active")
                .sync_create()
                .bidirectional()
                .build(),
        );

        bindings.push(
            phoneme
                .bind_property("xsampa-sound", &imp.xsampa_entry.get(), "text")
                .sync_create()
                .bidirectional()
                .build(),
        );

        bindings.push(
            phoneme
                .bind_property("romanization", &imp.romanization_entry.get(), "text")
                .sync_create()
                .bidirectional()
                .build(),
        );

        bindings.push(
            phoneme
                .bind_property("mora", &imp.mora_adj.get(), "value")
                .sync_create()
                .bidirectional()
                .transform_to(|_, v: u32| Some(v as f64))
                .transform_from(|_, v: f64| Some(v as u32))
                .build(),
        );
    }

    /// Unbinds form.
    pub fn unbind(&self) {
        let mut bindings = self.imp().form_bindings.borrow_mut();

        for binding in bindings.drain(..) {
            binding.unbind();
        }
    }

    /// Clears form fields.
    pub fn clear_fields(&self) {
        let imp = self.imp();
        imp.sound_entry.set_text("");
        imp.romanization_entry.set_text("");
    }
}

impl ui::View for Content {
    fn load_state(&self) {
        log::debug!("Loading view state.");

        self.unbind();
        self.clear_fields();
    }

    fn unload_state(&self) {
        log::debug!("Unloading view state.");

        self.unbind();
    }
}
