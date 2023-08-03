use adw::subclass::prelude::*;
use gtk::glib::FromVariant;
use gtk::prelude::*;
use gtk::{gio, glib};
use khazanah_core::Phoneme;
use uuid::Uuid;

use crate::models;
use crate::ui;

use super::phoneme_list_row::PhonemeListRow;

const EXPECTED_LIST_ITEM: &str = "Expected object to be `GtkListItem`";
const EXPECTED_PHONEME_OBJECT: &str = "Expected object to be `KhzPhonemeObject`";
const EXPECTED_PHONEME_LIST_ROW: &str = "Expected object to be `KhzPhonemeListRow`";

#[doc(hidden)]
#[allow(clippy::enum_variant_names)]
mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::glib::subclass::{Signal, SignalType};
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::Sidebar)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/view/inventory/sidebar.ui")]
    pub struct Sidebar {
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,

        #[template_child]
        pub edit_phoneme_button: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub list_view: TemplateChild<gtk::ListView>,

        #[property(get, set)]
        pub project_model: RefCell<models::ProjectModel>,

        #[property(get, set)]
        pub list_model: RefCell<Option<models::OrderedSet>>,
        #[property(get, set)]
        pub filter_model: RefCell<Option<gtk::FilterListModel>>,
        #[property(get, set)]
        pub sort_model: RefCell<Option<gtk::SortListModel>>,
        #[property(get, set)]
        pub selection_model: RefCell<Option<gtk::SingleSelection>>,

        pub action_group: RefCell<gio::SimpleActionGroup>,

        pub old_selected_id: Cell<Option<Uuid>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Sidebar {
        const NAME: &'static str = "KhzInventoryViewSidebar";
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
                    Signal::builder("phoneme-selected")
                        .param_types(Vec::<SignalType>::new())
                        .build(),
                    Signal::builder("phoneme-activated")
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
    pub struct Sidebar(ObjectSubclass<imp::Sidebar>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl Sidebar {
    fn setup_gactions(&self) {
        let imp = self.imp();
        let action_group = &*imp.action_group.borrow();

        self.insert_action_group("inventory", Some(action_group));

        action_group.add_action_entries([
            // Adds phoneme to list.
            gio::ActionEntry::builder("add-phoneme")
                .activate(glib::clone!(@weak self as view => move |_, _, _| {
                    view.add_phoneme();
                }))
                .build(),
            // Deletes phoneme of id `id` to list.
            gio::ActionEntry::builder("delete-phoneme")
                .parameter_type(Some(&String::static_variant_type()))
                .activate(glib::clone!(@weak self as view => move |_, _, v| {
                    if let Some(id) = v
                        .and_then(String::from_variant)
                        .and_then(|s| Uuid::try_parse(&s).ok())
                    {
                        view.delete_phoneme_by_id(id);
                    }
                }))
                .build(),
        ]);
    }

    fn setup_list(&self) {
        let imp = self.imp();

        // let list_model = gio::ListStore::new(models::PhonemeObject::static_type());
        let list_model = models::OrderedSet::new(models::PhonemeObject::static_type());
        self.set_list_model(list_model.clone());

        let filter_model =
            gtk::FilterListModel::new(Some(list_model), Option::<gtk::CustomFilter>::None);
        self.set_filter_model(filter_model.clone());

        let sort_model =
            gtk::SortListModel::new(Some(filter_model), Option::<gtk::CustomSorter>::None);
        self.set_sort_model(sort_model.clone());

        let selection_model = gtk::SingleSelection::new(Some(sort_model));
        self.set_selection_model(selection_model.clone());

        // Setup list factory
        let factory = gtk::SignalListItemFactory::new();

        factory.connect_setup(glib::clone!(@weak self as view => move |_, item| {
            let row = PhonemeListRow::new();

            view.imp().edit_phoneme_button
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
                .and_downcast::<models::PhonemeObject>()
                .expect(EXPECTED_PHONEME_OBJECT);

            let row = list_item
                .child()
                .and_downcast::<PhonemeListRow>()
                .expect(EXPECTED_PHONEME_LIST_ROW);

            row.bind(&word_object);
        });

        factory.connect_unbind(move |_, item| {
            let list_item = item
                .downcast_ref::<gtk::ListItem>()
                .expect(EXPECTED_LIST_ITEM);

            let row = list_item
                .child()
                .and_downcast::<PhonemeListRow>()
                .expect(EXPECTED_PHONEME_LIST_ROW);

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

    fn setup_search(&self) {}

    pub fn selected_phoneme(&self) -> Option<models::PhonemeObject> {
        self.selection_model()?.selected_item().and_downcast()
    }

    /// Adds a phoneme by its id.
    pub fn add_phoneme(&self) {
        let imp = self.imp();

        if let Some(id) = self.project_model().update(|project| {
            project
                .language_mut()
                .phonemic_inventory_mut()
                .add_phoneme(Phoneme::new())
        }) {
            // Exits search mode first.
            // self.imp().search_bar.set_search_mode(false);

            log::debug!("Added phoneme of id {}", id);
            let phoneme_object = models::PhonemeObject::query_project(self.project_model(), id);

            self.list_model()
                .expect("phoneme list model is not initialized")
                .insert(phoneme_object.id(), &phoneme_object);

            self.select_phoneme_by_id(id);

            imp.edit_phoneme_button.set_active(false);
            self.emit_by_name::<()>("phoneme-activated", &[]);
        }

        self.switch_stack_page();
    }

    /// Removes a phoneme by its id.
    pub fn delete_phoneme_by_id(&self, id: Uuid) {
        if self
            .project_model()
            .update(|project| {
                project
                    .language_mut()
                    .phonemic_inventory_mut()
                    .delete_phoneme_by_id(id)
            })
            .flatten()
            .is_some()
        {
            log::debug!("Deleted phoneme of id {}", id);

            let list_model = self
                .list_model()
                .expect("phoneme list model is not initialized");

            list_model.remove_by_id(&id);

            self.handle_selection_changed();

            self.switch_stack_page();
        }
    }

    /// Select a phoneme by its id.
    pub fn select_phoneme_by_id(&self, id: Uuid) -> bool {
        let selection_model = self
            .selection_model()
            .expect("Selection model is not initialized.");

        if let Some(position) = selection_model
            .iter::<glib::Object>()
            .position(|w| w.unwrap().downcast::<models::PhonemeObject>().unwrap().id() == id)
        {
            let result = selection_model.select_item(position as u32, true);
            if result && selection_model.n_items() == 1 {
                self.handle_selection_changed();
            }
            return result;
        }

        false
    }

    /// Callback to `selection-changed` signal
    pub fn handle_selection_changed(&self) {
        let imp = self.imp();

        if let Some(oid) = imp.old_selected_id.get() {
            self.list_model()
                .expect("Phoneme list model is not initialized.")
                .updated_by_id(&oid);
        }

        self.imp()
            .old_selected_id
            .set(self.selected_phoneme().map(|p| p.id()));

        self.emit_by_name::<()>("phoneme-selected", &[]);
    }

    /// Callback to 'activate' signal.
    #[template_callback]
    pub fn handle_row_activated(&self, _position: u32, _list_view: &gtk::ListView) {
        if !self.imp().edit_phoneme_button.is_active() {
            self.emit_by_name::<()>("phoneme-activated", &[]);
        }
    }

    /// Notifies the list model that a phoneme is updated.
    pub fn phoneme_updated(&self, phoneme: &models::PhonemeObject) {
        self.list_model()
            .expect("Phoneme list model is not initialized.")
            .updated_by_id(&phoneme.id());
    }

    // VIEW SWITCHING

    /// Switches to a stack page according to this view's state.
    pub fn switch_stack_page(&self) {
        let imp = self.imp();
        let stack = imp.stack.get();

        if self.list_model().map(|l| l.n_items()).unwrap_or_default() > 0 {
            stack.set_visible_child_name("list");
        } else {
            stack.set_visible_child_name("empty")
        }
    }
}

impl ui::View for Sidebar {
    fn load_state(&self) {
        log::debug!("Loading view state.");

        if let Some(mut list_model) = self.list_model() {
            list_model.remove_all();
            if let Some(project) = self.project_model().project().as_ref() {
                list_model.extend(project.language().phonemic_inventory().ids().map(|i| {
                    (
                        *i,
                        models::PhonemeObject::query_project(self.project_model(), *i),
                    )
                }));
            }
        }

        self.imp().edit_phoneme_button.set_active(false);

        self.switch_stack_page();
    }

    fn unload_state(&self) {
        log::debug!("Unloading view state.")
    }
}
