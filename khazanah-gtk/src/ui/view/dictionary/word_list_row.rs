use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

use crate::models::WordObject;

use uuid::Uuid;

#[doc(hidden)]
mod imp {
    use std::cell::{Cell, RefCell};

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
        pub reveal_action_buttons: Cell<bool>,

        pub id: Cell<Uuid>,

        pub bindings: RefCell<Vec<glib::Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WordListRow {
        const NAME: &'static str = "KhzDictionaryViewWordListRow";
        type Type = super::WordListRow;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
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

    /// Binds widget to a word object.
    pub fn bind(&self, word_object: &WordObject) {
        let word_label = self.imp().word_label.get();
        let pos_label = self.imp().pos_label.get();
        let translation_label = self.imp().translation_label.get();
        let pronunciation_label = self.imp().pronunciation_label.get();

        let mut bindings = self.imp().bindings.borrow_mut();

        bindings.push(
            word_object
                .bind_property("romanization", &word_label, "label")
                .sync_create()
                .transform_to(|_, s: Option<String>| {
                    if let Some(s) = s {
                        if !s.is_empty() {
                            return Some(s);
                        }
                    }
                    Some("(New word)".to_string())
                })
                .build(),
        );

        bindings.push(
            word_object
                .bind_property("pronunciation", &pronunciation_label, "label")
                .sync_create()
                .transform_to(|_, s: Option<String>| {
                    if let Some(s) = s {
                        if !s.is_empty() {
                            return Some(format!("/{}/", s));
                        } else {
                            return Some(s);
                        }
                    }
                    Some("".to_string())
                })
                .build(),
        );

        bindings.push(
            word_object
                .bind_property("part-of-speech-label", &pos_label, "label")
                .sync_create()
                .build(),
        );

        bindings.push(
            word_object
                .bind_property("translation", &translation_label, "label")
                .sync_create()
                .build(),
        );

        self.imp().id.set(word_object.id());
    }

    /// Unbinds widget.
    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }

    #[template_callback]
    pub fn handle_delete_button(&self, _button: &gtk::Button) {
        let id = self.imp().id.get();
        self.activate_action(
            "dictionary.delete-word",
            Some(&glib::Variant::from(id.to_string())),
        )
        .unwrap_or_default();
    }
}

impl Default for WordListRow {
    fn default() -> Self {
        Self::new()
    }
}
