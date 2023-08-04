use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use adw::prelude::*;
use adw::subclass::prelude::*;

use crate::models;
use crate::ui;

use khazanah_core::ALL_PARTS_OF_SPEECH;

#[doc(hidden)]
mod imp {
    use std::cell::{Cell, RefCell};

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::Content)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/view/dictionary/content.ui")]
    pub struct Content {
        #[template_child]
        pub romanization_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub translation_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub pronunciation_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub xsampa_toggle_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub xsampa_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub pos_dropdown: TemplateChild<adw::ComboRow>,

        #[property(get, set)]
        pub project_model: RefCell<models::ProjectModel>,

        #[property(get, set)]
        pub fields_sensitive: Cell<bool>,

        pub form_bindings: RefCell<Vec<glib::Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Content {
        const NAME: &'static str = "KhzDictionaryViewContent";
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

            let obj = self.obj();
            obj.setup_dropdown();
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
    }

    impl WidgetImpl for Content {}
    impl BinImpl for Content {}
}

glib::wrapper! {
    /// The view to edit lexicon word.
    pub struct Content(ObjectSubclass<imp::Content>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl Content {
    /// Setups dropdown widget.
    fn setup_dropdown(&self) {
        let imp = self.imp();

        // Populate dropdown.
        let pos_list: Vec<&str> = ALL_PARTS_OF_SPEECH
            .iter()
            .map(|pos| pos.map(|v| v.name()).unwrap_or("---"))
            .collect();
        let pos_model = gtk::StringList::new(&pos_list);
        imp.pos_dropdown.set_model(Some(&pos_model));
    }

    /// Binds a word to form.
    pub fn bind(&self, word: &models::WordObject) {
        let imp = self.imp();
        let mut bindings = imp.form_bindings.borrow_mut();

        bindings.push(
            word.bind_property("romanization", &imp.romanization_entry.get(), "text")
                .sync_create()
                .bidirectional()
                .build(),
        );

        bindings.push(
            word.bind_property("translation", &imp.translation_entry.get(), "text")
                .sync_create()
                .bidirectional()
                .build(),
        );

        bindings.push(
            word.bind_property("pronunciation", &imp.pronunciation_entry.get(), "text")
                .sync_create()
                .bidirectional()
                .build(),
        );

        bindings.push(
            word.bind_property("use-xsampa", &imp.xsampa_toggle_button.get(), "active")
                .sync_create()
                .bidirectional()
                .build(),
        );

        bindings.push(
            word.bind_property("xsampa-pronunciation", &imp.xsampa_entry.get(), "text")
                .sync_create()
                .bidirectional()
                .build(),
        );

        bindings.push(
            word.bind_property("part-of-speech", &imp.pos_dropdown.get(), "selected")
                .sync_create()
                .bidirectional()
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
        imp.romanization_entry.set_text("");
        imp.translation_entry.set_text("");
        imp.pronunciation_entry.set_text("");
        imp.pos_dropdown.set_selected(0);
    }

    /// Handler for `clicked` signal `from convert_from_ipa_button`
    #[template_callback]
    fn handle_convert_from_ipa_button_clicked(&self, _button: &gtk::Button) {
        let imp = self.imp();
        let pronunciation = imp.pronunciation_entry.text().to_string();
        let romanization = self
            .project_model()
            .query(|project| {
                project
                    .language()
                    .phonemic_inventory()
                    .get_romanization(&pronunciation)
            })
            .unwrap_or_default();
        imp.romanization_entry.set_text(&romanization);
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
