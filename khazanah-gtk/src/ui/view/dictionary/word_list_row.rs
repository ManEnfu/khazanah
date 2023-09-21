use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

use crate::models::WordObject;

#[doc(hidden)]
mod imp {
    use std::cell::{Cell, RefCell};

    use crate::utils::TemplateCallbacks;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::WordListRow)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/view/dictionary/word_list_row.ui")]
    pub struct WordListRow {
        #[template_child]
        pub word_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub pos_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub translation_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub pronunciation_label: TemplateChild<gtk::Label>,

        #[property(get, set)]
        pub word: RefCell<Option<WordObject>>,
        #[property(get, set)]
        pub reveal_action_buttons: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WordListRow {
        const NAME: &'static str = "KhzDictionaryViewWordListRow";
        type Type = super::WordListRow;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
            TemplateCallbacks::bind_template_callbacks(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for WordListRow {
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

    impl WidgetImpl for WordListRow {}

    impl BinImpl for WordListRow {}
}

glib::wrapper! {
    pub struct WordListRow(ObjectSubclass<imp::WordListRow>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl WordListRow {
    /// Creates a new list row.
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    #[template_callback]
    pub fn handle_delete_button(&self, _button: &gtk::Button) {
        if let Some(id) = self.word().map(|w| w.id()) {
            self.activate_action(
                "dictionary.delete-word",
                Some(&glib::Variant::from(id.to_string())),
            )
            .unwrap_or_default();
        }
    }

    #[template_callback(function)]
    fn display_romanization(s: Option<String>) -> Option<String> {
        if let Some(s) = s {
            if !s.is_empty() {
                return Some(s);
            }
        }
        Some("(New word)".to_string())
    }
}

impl Default for WordListRow {
    fn default() -> Self {
        Self::new()
    }
}
