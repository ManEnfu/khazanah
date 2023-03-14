use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

// use adw::prelude::*;
use adw::subclass::prelude::*;

use crate::models::WordObject;

use uuid::Uuid;

#[doc(hidden)]
mod imp {
    use std::cell::{Cell, RefCell};

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::ProjectLexiconWordListRow)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/project_lexicon_view/word_list_row.ui")]
    pub struct ProjectLexiconWordListRow {
        #[template_child]
        pub word_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub pos_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub translation_label: TemplateChild<gtk::Label>,

        #[property(get, set)]
        pub reveal_action_buttons: Cell<bool>,

        pub id: Cell<Uuid>,

        pub bindings: RefCell<Vec<glib::Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ProjectLexiconWordListRow {
        const NAME: &'static str = "KhzProjectLexiconWordListRow";
        type Type = super::ProjectLexiconWordListRow;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ProjectLexiconWordListRow {
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

    impl WidgetImpl for ProjectLexiconWordListRow {}

    impl BinImpl for ProjectLexiconWordListRow {}
}

glib::wrapper! {
    /// Row widget for `ProjectLexiconWordListView`.
    pub struct ProjectLexiconWordListRow(ObjectSubclass<imp::ProjectLexiconWordListRow>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl ProjectLexiconWordListRow {
    /// Creates a new list row.
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    /// Binds widget to a word object.
    pub fn bind(&self, word_object: &WordObject) {
        let word_label = self.imp().word_label.get();
        let pos_label = self.imp().pos_label.get();
        let translation_label = self.imp().translation_label.get();

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
            "lexicon-list.delete-word",
            Some(&glib::Variant::from(id.to_string())),
        )
        .unwrap_or_default();
    }
}

impl Default for ProjectLexiconWordListRow {
    fn default() -> Self {
        Self::new()
    }
}
