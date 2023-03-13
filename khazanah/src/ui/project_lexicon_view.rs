use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use adw::prelude::*;
use adw::subclass::prelude::*;

use crate::models;
use crate::ui;

use conlang::ALL_PARTS_OF_SPEECH;

pub use self::word_list_row::ProjectLexiconWordListRow;
pub use word_list_view::ProjectLexiconWordListView;

mod word_list_row;
mod word_list_view;

#[doc(hidden)]
mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::ProjectLexiconView)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/project_lexicon_view.ui")]
    pub struct ProjectLexiconView {
        #[template_child]
        pub pos_dropdown: TemplateChild<adw::ComboRow>,

        #[template_child]
        pub romanization_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub translation_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub pronunciation_entry: TemplateChild<adw::EntryRow>,

        #[template_child]
        pub word_list_view: TemplateChild<ProjectLexiconWordListView>,

        #[property(get, set)]
        pub project_model: RefCell<models::ProjectModel>,

       pub form_binding: RefCell<Vec<glib::Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ProjectLexiconView {
        const NAME: &'static str = "KhzProjectLexiconView";
        type Type = super::ProjectLexiconView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();

        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ProjectLexiconView {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.setup_dropdown();
            // obj.setup_list();
            obj.setup_callbacks();
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

    impl WidgetImpl for ProjectLexiconView {}
    impl BinImpl for ProjectLexiconView {}
}

glib::wrapper! {
    /// The view to edit project lexicon.
    pub struct ProjectLexiconView(ObjectSubclass<imp::ProjectLexiconView>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl ProjectLexiconView {
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

    /// Setups callbacks.
    fn setup_callbacks(&self) {
        let imp = self.imp();

        imp.word_list_view.connect_closure(
            "word-selected",
            false,
            glib::closure_local!(@strong self as view => move |_: &ProjectLexiconWordListView| {
                view.load_selected_word();
            }),
        );
    }

    /// Initializes forms.
    pub fn init_form(&self) {
        let mut bindings = self.imp().form_binding.borrow_mut();

        for binding in bindings.drain(..) {
            binding.unbind();
        }
    }

    /// Loads form contents with word data.
    pub fn load_selected_word(&self) {
        let imp = self.imp();

        let mut bindings = self.imp().form_binding.borrow_mut();

        for binding in bindings.drain(..) {
            binding.unbind();
        }

        let word = imp.word_list_view.selected_word();

        if let Some(word) = word {
            log::debug!("selected word: {}", word.id());

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
                word.bind_property("part-of-speech", &imp.pos_dropdown.get(), "selected")
                    .sync_create()
                    .bidirectional()
                    .build(),
            );
        }
    }
}

impl ui::View for ProjectLexiconView {
    fn load_state(&self) {
        log::debug!("Loading view state.");

        let imp = self.imp();
        let dirty = self.project_model().dirty();

        imp.word_list_view.load_state();

        self.load_selected_word();

        self.project_model().set_dirty(dirty);
    }

    fn commit_state(&self) {
        log::debug!("Committing view state.");

        let imp = self.imp();
        imp.word_list_view.commit_state();

        self.project_model().set_dirty(true);
    }

    fn connect_headerbar(&self, header_bar: &ui::HeaderBar) {
        let imp = self.imp();
        imp.word_list_view.connect_headerbar(header_bar);
    }
}
