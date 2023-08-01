use adw::subclass::prelude::*;
use gtk::glib::FromVariant;
use gtk::prelude::*;
use gtk::{gio, glib};

use khazanah_core::Word;
use uuid::Uuid;

const EXPECTED_LIST_ITEM: &str = "Expected object to be `GtkListItem`";
const EXPECTED_WORD_OBJECT: &str = "Expected object to be `KhzWordObject`";
const EXPECTED_WORD_LIST_ROW: &str = "Expected object to be `KhzWordListRow`";

use crate::models::{self, WordObject};
use crate::ui;

use super::WordListRow;

#[doc(hidden)]
#[allow(clippy::enum_variant_names)]
mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::glib::subclass::{Signal, SignalType};
    use once_cell::sync::Lazy;

    use crate::models;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::Sidebar)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/view/dictionary/sidebar.ui")]
    pub struct Sidebar {
        #[template_child]
        pub list_view: TemplateChild<gtk::ListView>,

        #[template_child]
        pub search_word_button: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub sort_word_button: TemplateChild<gtk::MenuButton>,
        #[template_child]
        pub add_word_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub edit_word_button: TemplateChild<gtk::ToggleButton>,

        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub main_page: TemplateChild<gtk::Box>,
        #[template_child]
        pub list_empty_page: TemplateChild<adw::StatusPage>,

        #[template_child]
        pub search_bar: TemplateChild<gtk::SearchBar>,
        #[template_child]
        pub search_entry: TemplateChild<gtk::SearchEntry>,

        #[template_child]
        pub search_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub scrolled_window: TemplateChild<gtk::ScrolledWindow>,
        #[template_child]
        pub search_result_empty: TemplateChild<adw::StatusPage>,

        #[property(get, set)]
        pub project_model: RefCell<models::ProjectModel>,

        #[property(get, set)]
        pub word_list_model: RefCell<Option<gio::ListStore>>,
        #[property(get, set)]
        pub filter_model: RefCell<Option<gtk::FilterListModel>>,
        #[property(get, set)]
        pub sort_model: RefCell<Option<gtk::SortListModel>>,
        #[property(get, set)]
        pub selection_model: RefCell<Option<gtk::SingleSelection>>,

        #[property(get, set)]
        pub filter_category: RefCell<String>,

        pub action_group: RefCell<gio::SimpleActionGroup>,

        pub selected_id: Cell<Uuid>,
        pub old_selected_word: RefCell<Option<WordObject>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Sidebar {
        const NAME: &'static str = "KhzDictionaryViewSidebar";
        type Type = super::Sidebar;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Sidebar {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.setup_gactions();
            obj.setup_list();
            obj.setup_search();
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
                vec![
                    Signal::builder("word-selected")
                        .param_types(Vec::<SignalType>::new())
                        .build(),
                    Signal::builder("word-activated")
                        .param_types(Vec::<SignalType>::new())
                        .build(),
                    Signal::builder("search-changed")
                        .param_types(Vec::<SignalType>::new())
                        .build(),
                ]
            });
            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for Sidebar {}
    impl BinImpl for Sidebar {}
}

glib::wrapper! {
    /// A List view of words in the lexicon with related controls.
    pub struct Sidebar(ObjectSubclass<imp::Sidebar>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl Sidebar {
    // SETUPS

    /// Setups models and list
    fn setup_list(&self) {
        let imp = self.imp();

        // Setup list models
        let word_list_model = gio::ListStore::new(WordObject::static_type());
        self.set_word_list_model(word_list_model.clone());

        let filter_model = gtk::FilterListModel::new(
            Some(word_list_model),
            Some(models::WordFilter::new(models::WordFilterBy::None)),
        );
        self.set_filter_model(filter_model.clone());

        let sort_model = gtk::SortListModel::new(
            Some(filter_model),
            Some(models::WordSorter::new(models::WordSortBy::Romanization)),
        );
        self.set_sort_model(sort_model.clone());

        let selection_model = gtk::SingleSelection::new(Some(sort_model));
        self.set_selection_model(selection_model.clone());

        // Setup list factory
        let factory = gtk::SignalListItemFactory::new();

        factory.connect_setup(glib::clone!(@weak self as view => move |_, item| {
            let row = WordListRow::new();

            view.imp().edit_word_button
                .bind_property("active", &row, "reveal-action-buttons")
                .sync_create()
                .build();

            item.downcast_ref::<gtk::ListItem>()
                .expect(EXPECTED_LIST_ITEM)
                .set_child(Some(&row));
        }));

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
                .and_downcast::<WordListRow>()
                .expect(EXPECTED_WORD_LIST_ROW);

            row.bind(&word_object);
        });

        factory.connect_unbind(move |_, item| {
            let list_item = item
                .downcast_ref::<gtk::ListItem>()
                .expect(EXPECTED_LIST_ITEM);

            let row = list_item
                .child()
                .and_downcast::<WordListRow>()
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

    /// Setups search bar
    pub fn setup_search(&self) {
        let imp = self.imp();

        imp.search_bar.connect_entry(&imp.search_entry.get());
        self.set_filter_category("all");
    }

    /// Setups actions for this view.
    pub fn setup_gactions(&self) {
        let imp = self.imp();
        let action_group = &*imp.action_group.borrow();

        self.insert_action_group("dictionary", Some(action_group));

        action_group.add_action_entries([
            // Adds word to list.
            gio::ActionEntry::builder("add-word")
                .activate(glib::clone!(@weak self as view => move |_, _, _| {
                    view.add_word();
                }))
                .build(),
            // Deletes word of id `id` to list.
            gio::ActionEntry::builder("delete-word")
                .parameter_type(Some(&String::static_variant_type()))
                .activate(glib::clone!(@weak self as view => move |_, _, v| {
                    if let Some(id) = v
                        .and_then(String::from_variant)
                        .and_then(|s| Uuid::try_parse(&s).ok())
                    {
                        view.delete_word_by_id(id);
                    }
                }))
                .build(),
            // Sorts by category.
            gio::ActionEntry::builder("sort-category")
                .parameter_type(Some(&String::static_variant_type()))
                .state("romanization".to_variant())
                .activate(glib::clone!(@weak self as view => move |_, action, v| {
                    if let Some(v) = v.and_then(String::from_variant) {
                        let sort_by = match v.as_str() {
                            "romanization" => models::WordSortBy::Romanization,
                            "translation" => models::WordSortBy::Translation,
                            "part-of-speech" => models::WordSortBy::PartOfSpeech,
                            s => {
                                log::warn!("Unknown sort category: {}", s);
                                models::WordSortBy::Romanization
                            }
                        };
                        view.set_sort_category(sort_by);
                        action.set_state(v.to_variant());
                    }
                }))
                .build(),
            // Sets sorting order.
            gio::ActionEntry::builder("sort-order")
                .parameter_type(Some(&bool::static_variant_type()))
                .state(false.to_variant())
                .activate(glib::clone!(@weak self as view => move |_, action, v| {
                    if let Some(descending) = v.and_then(bool::from_variant) {
                        view.set_sort_order(descending);
                        action.set_state(descending.to_variant());
                    }
                }))
                .build(),
            // Filters by category.
            gio::ActionEntry::builder("filter-category")
                .parameter_type(Some(&String::static_variant_type()))
                .state("all".to_variant())
                .activate(glib::clone!(@weak self as view => move |_, action, v| {
                    if let Some(v) = v.and_then(String::from_variant) {
                        action.set_state(v.to_variant());
                        log::debug!("Set filter category to {}.", &v);
                        view.set_filter_category(v);
                        view.handle_search_entry_changed(&view.imp().search_entry.get());
                    }
                }))
                .build(),
        ]);
    }

    // LIST OPERATIONS

    /// Gets current selected word.
    pub fn selected_word(&self) -> Option<WordObject> {
        self.selection_model()?.selected_item().and_downcast()
    }

    /// Adds a new word to the model.
    pub fn add_word(&self) {
        if let Some(id) = self.project_model().update(|project| {
            project
                .language_mut()
                .dictionary_mut()
                .add_word(Word::new())
        }) {
            // Exits search mode first.
            self.imp().search_bar.set_search_mode(false);

            log::debug!("Added word of id {}", id);
            let word_object = WordObject::query_project(self.project_model(), id);

            self.word_list_model()
                .expect("word list model is not initialized")
                .append(&word_object);

            self.select_word_by_id(id);

            self.imp().edit_word_button.set_active(false);
            self.emit_by_name::<()>("word-activated", &[]);
        }

        self.switch_stack_page();
    }

    /// Deletes a word by its id.
    pub fn delete_word_by_id(&self, id: Uuid) {
        if let Some(true) = self.project_model().update(|project| {
            project
                .language_mut()
                .dictionary_mut()
                .delete_word_by_id(id)
        }) {
            log::debug!("Deleted word of id {}", id);

            let word_list_model = self
                .word_list_model()
                .expect("word list model is not initialized");

            if let Some(position) = word_list_model
                .iter::<glib::Object>()
                .position(|w| w.unwrap().downcast_ref::<WordObject>().unwrap().id() == id)
            {
                word_list_model.remove(position as u32);
            };

            self.handle_selection_changed();

            self.switch_stack_page();
        }
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

    /// Callback to `selection-changed` signal.
    pub fn handle_selection_changed(&self) {
        if let Some(word) = self.imp().old_selected_word.borrow().as_ref() {
            self.notify_changes_to_model(word);
        }

        self.imp().old_selected_word.replace(self.selected_word());

        self.emit_by_name::<()>("word-selected", &[])
    }

    /// Callback to 'activate' signal.
    #[template_callback]
    pub fn handle_row_activated(&self, _position: u32, _list_view: &gtk::ListView) {
        if !self.imp().edit_word_button.is_active() {
            self.emit_by_name::<()>("word-activated", &[]);
        }
    }

    /// Notify the model that a word is updates.
    pub fn notify_changes_to_model(&self, word: &WordObject) {
        let word_list_model = self
            .word_list_model()
            .expect("Word list model is not initialized.");

        if let Some(position) = word_list_model.find(word) {
            word_list_model.items_changed(position, 1, 1);
        }
    }

    // SEARCHING

    /// Responds to `search-changed` signal from search entry.
    #[template_callback]
    pub fn handle_search_entry_changed(&self, entry: &gtk::SearchEntry) {
        let filter_text = entry.text().to_string();

        let filter_by = match self.filter_category().as_str() {
            "all" => models::WordFilterBy::AllAttrs(filter_text),
            "romanization" => models::WordFilterBy::Romanization(filter_text),
            "translation" => models::WordFilterBy::Translation(filter_text),
            s => {
                log::warn!("Unknown filter category: {}", s);
                models::WordFilterBy::AllAttrs(filter_text)
            }
        };
        log::debug!("Searching by {:?}.", &filter_by);
        self.search_word(filter_by);
    }

    /// Responds to `notify::search-mode-enabled` signal from search bar.
    #[template_callback]
    pub fn handle_search_mode_toggle(&self, _pspec: glib::ParamSpec, bar: &gtk::SearchBar) {
        let imp = self.imp();

        if bar.is_search_mode() {
        } else {
            imp.search_entry.set_text("");
            self.search_word(models::WordFilterBy::None);
        }
    }

    /// Updates the search.
    pub fn search_word(&self, filter_by: models::WordFilterBy) {
        if let Some(filter_model) = self.filter_model() {
            if let Some(filter) = filter_model
                .filter()
                .and_then(|f| f.downcast::<models::WordFilter>().ok())
            {
                filter.set_filter_by(filter_by);
            }

            // Displays status page if search result is empty.
            let imp = self.imp();
            if filter_model.n_items() > 0 {
                imp.search_stack.set_visible_child(&*imp.scrolled_window);
            } else {
                imp.search_stack
                    .set_visible_child(&*imp.search_result_empty);
            }
            self.emit_by_name::<()>("search-changed", &[])
        }
    }

    // SORTING

    // Sets the category by which the list will be sorted.
    pub fn set_sort_category(&self, sort_by: models::WordSortBy) {
        if let Some(sorter) = self
            .sort_model()
            .and_then(|sm| sm.sorter())
            .and_then(|s| s.downcast::<models::WordSorter>().ok())
        {
            log::debug!("Sort by: {:?}", &sort_by);
            sorter.set_sort_by(sort_by);
        }
    }

    // Sets the order of the sorting. `false` for ascending, `true` for descending.
    pub fn set_sort_order(&self, descending: bool) {
        if let Some(sorter) = self
            .sort_model()
            .and_then(|sm| sm.sorter())
            .and_then(|s| s.downcast::<models::WordSorter>().ok())
        {
            log::debug!("Sort order descending: {}", descending);
            sorter.set_descending(descending);
            self.imp().sort_word_button.set_icon_name(match descending {
                false => "view-sort-ascending-symbolic",
                true => "view-sort-descending-symbolic",
            })
        }
    }

    // VIEW SWITCHING

    /// Switches to a stack page according to this view's state.
    pub fn switch_stack_page(&self) {
        let imp = self.imp();
        let stack = imp.stack.get();

        if self
            .word_list_model()
            .map(|wl| wl.n_items())
            .unwrap_or_default()
            > 0
        {
            // stack.set_visible_child(&*imp.main_page);
            stack.set_visible_child_name("list");
        } else {
            // stack.set_visible_child(&*imp.list_empty_page);
            stack.set_visible_child_name("empty")
        }
    }
}

impl ui::View for Sidebar {
    fn load_state(&self) {
        log::debug!("Loading view state.");

        // reload list
        if let Some(mut word_list_model) = self.word_list_model() {
            word_list_model.remove_all();
            if let Some(project) = self.project_model().project().as_ref() {
                word_list_model.extend(
                    project
                        .language()
                        .dictionary()
                        .ids()
                        .map(|i| WordObject::query_project(self.project_model(), *i)),
                );
            }

            self.select_word_by_id(self.imp().selected_id.get());
        }

        self.imp().edit_word_button.set_active(false);

        self.switch_stack_page();
    }

    fn unload_state(&self) {
        log::debug!("Unload view state.");

        if let Some(word) = self.selected_word() {
            self.imp().selected_id.set(word.id());
        }

        self.imp().edit_word_button.set_active(false);
    }
}
