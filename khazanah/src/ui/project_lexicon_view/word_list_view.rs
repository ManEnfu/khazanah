use conlang::Word;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use adw::subclass::prelude::*;
use uuid::Uuid;

const EXPECTED_LIST_ITEM: &str = "Expected object to be `GtkListItem`";
const EXPECTED_WORD_OBJECT: &str = "Expected object to be `KhzWordObject`";
const EXPECTED_WORD_LIST_ROW: &str = "Expected object to be `KhzWordListRow`";

use crate::models::{self, WordObject};
use crate::ui;

use super::ProjectLexiconWordListRow;

#[doc(hidden)]
mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::glib::subclass::{Signal, SignalType};
    use once_cell::sync::Lazy;

    use crate::models;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::ProjectLexiconWordListView)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/project_lexicon_view/word_list_view.ui")]
    pub struct ProjectLexiconWordListView {
        #[template_child]
        pub list_view: TemplateChild<gtk::ListView>,

        #[template_child]
        pub search_word_button: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub add_word_button: TemplateChild<gtk::Button>,

        #[template_child]
        pub view_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub main_page: TemplateChild<gtk::ScrolledWindow>,
        #[template_child]
        pub list_empty_page: TemplateChild<adw::StatusPage>,

        #[property(get, set)]
        pub project_model: RefCell<models::ProjectModel>,

        #[property(get, set)]
        pub word_list_model: RefCell<Option<gio::ListStore>>,
        #[property(get, set)]
        pub filter_model: RefCell<Option<gtk::FilterListModel>>,
        #[property(get, set)]
        pub selection_model: RefCell<Option<gtk::SingleSelection>>,

        #[property(get, set)]
        pub reveal_header: Cell<bool>,

        pub selected_id: Cell<Uuid>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ProjectLexiconWordListView {
        const NAME: &'static str = "KhzProjectLexiconWordListView";
        type Type = super::ProjectLexiconWordListView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();

            klass.install_action("lexicon.add-word", None, move |widget, _, _| {
                widget.add_word();
            })
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ProjectLexiconWordListView {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.setup_list();
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
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("word-selected")
                    .param_types(Vec::<SignalType>::new())
                    .build()]
            });
            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for ProjectLexiconWordListView {}
    impl BinImpl for ProjectLexiconWordListView {}
}

glib::wrapper! {
    /// A List view of words in the lexicon with related controls.
    pub struct ProjectLexiconWordListView(ObjectSubclass<imp::ProjectLexiconWordListView>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl ProjectLexiconWordListView {
    /// Setup models and list
    fn setup_list(&self) {
        let imp = self.imp();

        // Setup list models
        let word_list_model = gio::ListStore::new(WordObject::static_type());
        self.set_word_list_model(word_list_model.clone());

        let filter_model =
            gtk::FilterListModel::new(Some(word_list_model), Option::<gtk::CustomFilter>::None);
        self.set_filter_model(filter_model.clone());

        let selection_model = gtk::SingleSelection::new(Some(filter_model));
        self.set_selection_model(selection_model.clone());

        // Setup list factory
        let factory = gtk::SignalListItemFactory::new();

        factory.connect_setup(move |_, item| {
            let row = ProjectLexiconWordListRow::new();
            item.downcast_ref::<gtk::ListItem>()
                .expect(EXPECTED_LIST_ITEM)
                .set_child(Some(&row));
        });

        factory.connect_bind(move |_, item| {
            let list_item = item
                .downcast_ref::<gtk::ListItem>()
                .expect(EXPECTED_LIST_ITEM);

            let word_object = list_item
                .item()
                .and_downcast::<models::WordObject>()
                .expect(EXPECTED_WORD_OBJECT);

            let row = list_item
                .child()
                .and_downcast::<ProjectLexiconWordListRow>()
                .expect(EXPECTED_WORD_LIST_ROW);

            row.bind(&word_object);
        });

        factory.connect_unbind(move |_, item| {
            let list_item = item
                .downcast_ref::<gtk::ListItem>()
                .expect(EXPECTED_LIST_ITEM);

            let row = list_item
                .child()
                .and_downcast::<ProjectLexiconWordListRow>()
                .expect(EXPECTED_WORD_LIST_ROW);

            row.unbind();
        });

        imp.list_view.set_model(Some(&selection_model));
        imp.list_view.set_factory(Some(&factory));

        // List callbacks
        selection_model.connect_selection_changed(
            glib::clone!(@weak self as widget => move |_, _, _| {
                 widget.handle_selection_changed();
            }),
        );
    }

    /// Gets current selected word.
    pub fn selected_word(&self) -> Option<WordObject> {
        self.selection_model()?.selected_item().and_downcast()
    }

    /// Select a word by its id.
    pub fn select_word_by_id(&self, id: Uuid) -> bool {
        let selection_model = self
            .selection_model()
            .expect("Selection model is not initialized.");

        if let Some(position) = selection_model
            .iter::<glib::Object>()
            .position(|w| w.unwrap().downcast_ref::<WordObject>().unwrap().id() == id)
        {
            let result = selection_model.select_item(position as u32, true);
            if result && selection_model.n_items() == 1 {
                self.handle_selection_changed();
            }
            return result;
        }

        false
    }

    /// Adds a new word to the model.
    pub fn add_word(&self) {
        if let Some(id) = self
            .project_model()
            .update(|project| project.lexicon_mut().add_word(Word::new()))
        {
            log::debug!("Added word by id {}", id);
            let word_object = WordObject::new(self.project_model(), id);

            self.word_list_model()
                .expect("word list model is not initialized")
                .append(&word_object);

            self.select_word_by_id(id);
        }
        
        self.switch_stack_page();
    }

    /// Callback to `selection-changed` signal
    pub fn handle_selection_changed(&self) {
        self.emit_by_name::<()>("word-selected", &[])
    }

    /// Switches to a stack page according to this view's state.
    pub fn switch_stack_page(&self) {
        let imp = self.imp();
        let stack = imp.view_stack.get();

        if self.word_list_model().map(|wl| wl.n_items()).unwrap_or_default() > 0 {
            stack.set_visible_child(&*imp.main_page); 
        } else {
            stack.set_visible_child(&*imp.list_empty_page);
        }
    }
}

impl ui::View for ProjectLexiconWordListView {
    fn load_state(&self) {
        log::debug!("Loading view state.");

        // reload list
        if let Some(mut word_list_model) = self.word_list_model() {
            word_list_model.remove_all();
            if let Some(project) = self.project_model().project().as_ref() {
                word_list_model.extend(
                    project
                        .lexicon()
                        .ids()
                        .map(|i| WordObject::new(self.project_model(), *i)),
                );
            }

            self.select_word_by_id(self.imp().selected_id.get());
        }

        self.switch_stack_page();
    }

    fn commit_state(&self) {
        log::debug!("Committing view state.");

        if let Some(word) = self.selected_word() {
            self.imp().selected_id.set(word.id());
        }
    }

    fn connect_headerbar(&self, header_bar: &ui::HeaderBar) {
        let imp = self.imp();

        let _binding = header_bar
            .imp()
            .search_word_button
            .bind_property("active", &imp.search_word_button.get(), "active")
            .sync_create()
            .bidirectional()
            .build();

        header_bar.imp().add_word_button.connect_clicked(
            glib::clone!(@strong self as view => move |_| {
                view.add_word();
            }),
        );
    }
}
