use std::path::Path;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use adw::subclass::prelude::*;

use crate::ui::{self, MainView, View};

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::glib::{
        once_cell::sync::Lazy,
        subclass::{Signal, SignalType},
    };

    use crate::models;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::ApplicationWindow)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/window.ui")]
    pub struct ApplicationWindow {
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,

        #[template_child]
        pub main_stack: TemplateChild<gtk::Stack>,

        #[template_child]
        pub start_view: TemplateChild<ui::StartView>,
        #[template_child]
        pub language_view: TemplateChild<ui::LanguageView>,
        #[template_child]
        pub dictionary_view: TemplateChild<ui::DictionaryView>,
        #[template_child]
        pub inventory_view: TemplateChild<ui::InventoryView>,

        #[template_child]
        pub header_bar: TemplateChild<ui::HeaderBar>,
        #[template_child]
        pub action_bar: TemplateChild<gtk::ActionBar>,

        #[property(get, set)]
        pub project_opened: Cell<bool>,
        #[property(get, set)]
        pub project_model: RefCell<models::ProjectModel>,

        #[property(get, set)]
        pub selected_view_index: Cell<u32>,

        #[property(get, set)]
        pub narrow: Cell<bool>,

        #[property(get, set)]
        pub force_action: Cell<bool>,

        pub selected_view: Cell<MainView>,

        pub file_dialog: RefCell<Option<gtk::FileDialog>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ApplicationWindow {
        const NAME: &'static str = "KhzApplicationWindow";
        type Type = super::ApplicationWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ApplicationWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.setup_bindings();

            let header_bar = &self.header_bar.get();
            self.language_view.connect_headerbar(header_bar);
            self.inventory_view.connect_headerbar(header_bar);
            self.dictionary_view.connect_headerbar(header_bar);
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
                    Signal::builder("open-project")
                        .param_types([Option::<String>::static_type()])
                        .build(),
                    Signal::builder("new-project")
                        .param_types(Vec::<SignalType>::new())
                        .build(),
                ]
            });
            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for ApplicationWindow {
        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            self.obj().on_window_size(width, height);

            self.parent_size_allocate(width, height, baseline);
        }
    }

    impl WindowImpl for ApplicationWindow {
        fn close_request(&self) -> glib::signal::Inhibit {
            if self.project_model.borrow().dirty() && !self.force_action.get() {
                log::debug!("Project is dirty.");
                self.obj().confirm_save_dialog(Some("window.close"));
                return glib::signal::Inhibit(true);
            }

            glib::signal::Inhibit(false)
        }
    }

    impl ApplicationWindowImpl for ApplicationWindow {}

    impl AdwApplicationWindowImpl for ApplicationWindow {}
}

glib::wrapper! {
    /// The main window for the application.
    pub struct ApplicationWindow(ObjectSubclass<imp::ApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

#[gtk::template_callbacks]
impl ApplicationWindow {
    /// Creates a new window.
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    /// Setups `GAction`s for the window.
    fn setup_gactions(&self) {
        // Open project
        let open_action = gio::ActionEntry::builder("open")
            .activate(|window: &Self, _, _| {
                if window.project_model().dirty() && !window.force_action() {
                    log::debug!("Project is dirty.");
                    window.confirm_save_dialog(Some("win.open"));
                } else {
                    window.set_force_action(false);
                    window.open_file_dialog();
                }
            })
            .build();
        // New project
        let new_action = gio::ActionEntry::builder("new")
            .activate(|window: &Self, _, _| {
                if window.project_model().dirty() && !window.force_action() {
                    log::debug!("Project is dirty.");
                    window.confirm_save_dialog(Some("win.new"));
                } else {
                    window.set_force_action(false);
                    window.emit_by_name::<()>("new-project", &[]);
                }
            })
            .build();
        // Save project
        let save_action = gio::ActionEntry::builder("save")
            .activate(|window: &Self, _, _| {
                if !window.project_model().opened() {
                    return;
                }
                match window.project_model().path() {
                    Some(path) => window.save_project_file(path, None),
                    None => window.save_file_dialog(None),
                }
            })
            .build();
        // Save project as another file
        let save_as_action = gio::ActionEntry::builder("save-as")
            .activate(|window: &Self, _, _| {
                if window.project_model().opened() {
                    window.save_file_dialog(None);
                }
            })
            .build();

        let go_back_action = gio::ActionEntry::builder("go-back")
            .activate(|window: &Self, _, _| {
                window.go_back();
            })
            .build();

        self.add_action_entries([
            open_action,
            new_action,
            save_action,
            save_as_action,
            go_back_action,
        ]);
        self.action_set_enabled("win.save", false);
        self.action_set_enabled("win.save-as", false);

        let shortcuts_window =
            gtk::Builder::from_resource("/com/github/manenfu/Khazanah/ui/shortcuts_window.ui")
                .object::<gtk::ShortcutsWindow>("shortcuts_window")
                .expect("Error building shortcuts window.");
        self.set_help_overlay(Some(&shortcuts_window));
    }

    fn setup_bindings(&self) {
        self.project_model()
            .bind_property("title", self, "title")
            .sync_create()
            .transform_to(|_, s: Option<String>| {
                Some(s.map_or("Khazanah".to_string(), |s| format!("{} - Khazanah", s)))
            })
            .build();

        self.project_model()
            .bind_property("title", &self.imp().header_bar.get(), "title")
            .sync_create()
            .transform_to(|_, s: Option<String>| Some(s.unwrap_or("Khazanah".to_string())))
            .build();

        self.project_model()
            .bind_property("path", &self.imp().header_bar.get(), "subtitle")
            .sync_create()
            .build();
    }

    /// Shows `Open File` dialog.
    fn open_file_dialog(&self) {
        let imp = self.imp();
        // Skip if dialog is already opened
        if imp.file_dialog.borrow().is_some() {
            return;
        }

        let filter = gtk::FileFilter::new();
        filter.add_suffix(khazanah_core::PROJECT_FILE_EXT);

        let dialog = gtk::FileDialog::builder()
            .title("Open File")
            .accept_label("_Open")
            .default_filter(&filter)
            .build();

        imp.file_dialog.replace(Some(dialog.clone()));

        log::debug!("Open file dialog.");

        dialog.open(
            Some(self),
            Option::<&gio::Cancellable>::None,
            glib::clone!(@strong self as window => move |response| {
                match response {
                    Ok(f) => {
                        if let Some(path) = f.path() {
                            window.emit_by_name::<()>("open-project", &[&path]);
                        } else {
                            log::error!("Open file dialog error: Invalid path");
                        }
                    }
                    Err(e) => match e.kind::<gtk::DialogError>() {
                        Some(gtk::DialogError::Dismissed) => log::debug!("Open file dialog dismissed"),
                        _ => log::error!("Open file dialog error: {e:?}")
                    }
                }
                window.imp().file_dialog.replace(None);
            }),
        );
    }

    /// Opens a project file for this window.
    pub fn open_project_file<P: AsRef<Path>>(&self, path: P) {
        log::debug!("Opening file: {:?}", path.as_ref());

        let ctx = glib::MainContext::default();
        let self_weak = glib::SendWeakRef::from(self.downgrade());

        let path = path.as_ref().to_path_buf();
        ctx.spawn_local(async move {
            if let Some(window) = self_weak.upgrade() {
                match window.project_model().load_file(&path) {
                    Ok(_) => {
                        log::info!("Project opened: {:?}", &path);
                        window.finish_open_project();
                        let msg = format!(
                            "Opened \"{}\"",
                            path.file_name()
                                .unwrap_or_default()
                                .to_str()
                                .unwrap_or_default()
                        );
                        window.imp().toast_overlay.add_toast(adw::Toast::new(&msg));
                    }
                    Err(e) => {
                        log::error!("Error opening file: {}", e);
                        let msg = format!(
                            "Unable to Open \"{}\"",
                            path.file_name()
                                .unwrap_or_default()
                                .to_str()
                                .unwrap_or_default()
                        );
                        window.imp().toast_overlay.add_toast(adw::Toast::new(&msg));
                    }
                }
            }
        });
    }

    /// Creates a new project and sets it as the current project.
    pub fn new_project(&self) {
        // self.project_model().set_project(Some(Project::new()));
        self.project_model().new_project();
        self.finish_open_project();
        self.imp()
            .toast_overlay
            .add_toast(adw::Toast::new("Created a New Project"));
    }

    /// Called after a project is opened or created for this window.
    fn finish_open_project(&self) {
        if self.project_model().opened() {
            self.set_project_opened(true);
            self.action_set_enabled("win.save", true);
            self.action_set_enabled("win.save-as", true);

            // self.update_title();
            self.load_all_views();

            self.switch_view(MainView::Language);
        }
    }

    /// Shows `Save File` dialog.
    fn save_file_dialog(&self, next_action: Option<&'static str>) {
        let imp = self.imp();
        // Skip if dialog is already opened
        if imp.file_dialog.borrow().is_some() {
            return;
        }

        let filter = gtk::FileFilter::new();
        filter.add_suffix(khazanah_core::PROJECT_FILE_EXT);

        let dialog = gtk::FileDialog::builder()
            .title("Open File")
            .accept_label("_Save")
            .default_filter(&filter)
            .build();

        imp.file_dialog.replace(Some(dialog.clone()));

        log::debug!("Save file dialog.");

        dialog.save(
            Some(self),
            Option::<&gio::Cancellable>::None,
            glib::clone!(@strong self as window => move |response| {
                match response {
                    Ok(f) => {
                        if let Some(path) = f.path() {
                            window.save_project_file(path, next_action)
                        } else {
                            log::error!("Save file dialog error: Invalid path");
                        }
                    }
                    Err(e) => match e.kind::<gtk::DialogError>() {
                        Some(gtk::DialogError::Dismissed) => log::debug!("Save file dialog dismissed"),
                        _ => log::error!("Save file dialog error: {e:?}")
                    }
                }
                window.imp().file_dialog.replace(None);
            }),
        );
    }

    /// Save the current project.
    pub fn save_project_file<P: AsRef<Path>>(&self, path: P, next_action: Option<&'static str>) {
        log::debug!("Saving file: {:?}", path.as_ref());

        self.commit_all_views();

        let ctx = glib::MainContext::default();
        let self_weak = glib::SendWeakRef::from(self.downgrade());

        let path = path.as_ref().to_path_buf();
        ctx.spawn_local(async move {
            if let Some(window) = self_weak.upgrade() {
                match window.project_model().save_file(&path) {
                    Ok(_) => {
                        log::info!("Project saved: {:?}", &path);
                        let msg = format!(
                            "Saved \"{}\"",
                            path.file_name()
                                .unwrap_or_default()
                                .to_str()
                                .unwrap_or_default()
                        );
                        window.imp().toast_overlay.add_toast(adw::Toast::new(&msg));
                        if let Some(action) = next_action {
                            gtk::prelude::WidgetExt::activate_action(&window, action, None)
                                .unwrap_or_default();
                        }
                    }
                    Err(e) => {
                        log::error!("Error saving file: {}", e);
                        let msg = format!(
                            "Unable to Save \"{}\"",
                            path.file_name()
                                .unwrap_or_default()
                                .to_str()
                                .unwrap_or_default()
                        );
                        window.imp().toast_overlay.add_toast(adw::Toast::new(&msg));
                    }
                }
            }
        });
    }

    /// Shows save confirmation dialog.
    pub fn confirm_save_dialog(&self, next_action: Option<&'static str>) {
        let builder =
            gtk::Builder::from_resource("/com/github/manenfu/Khazanah/ui/confirm_save_dialog.ui");
        let dialog = builder.object::<adw::MessageDialog>("dialog").unwrap();
        dialog.set_transient_for(Some(self));
        dialog.connect_closure(
            "response",
            false,
            glib::closure_local!(@strong self as window => move |_: &adw::MessageDialog, response: &str| {
                match response {
                    "save" => match window.project_model().path() {
                        Some(path) => window.save_project_file(path, next_action),
                        None => window.save_file_dialog(next_action),
                    }
                    "discard" => {
                        if let Some(action) = next_action {
                            window.set_force_action(true);
                            gtk::prelude::WidgetExt::activate_action(&window, action, None).unwrap_or_default();
                        }
                    }
                    _ => {}
                }
            })
        );
        dialog.present();
    }

    // VIEWS

    /// Switches to a view. This will set an internal property to sync with all view switchers in
    /// the window.
    pub fn switch_view(&self, view: MainView) {
        if view != MainView::Unknown {
            self.set_selected_view_index(u32::from(view));
        }
    }

    /// Responds to change of view selection by switching to said view.
    #[template_callback]
    fn handle_selected_view_index_changed(&self, _pspec: glib::ParamSpec, _s: &Self) {
        let idx = self.selected_view_index();
        let view = MainView::from(idx);
        log::debug!("Switching to view: {:?} ({})", view, idx);

        let imp = self.imp();
        let current_view = imp.selected_view.get();

        if current_view != MainView::Unknown {
            self.commit_view_state(current_view);
            self.unload_view_state(current_view);
        }

        self.load_view_state(view);
        let main_stack = imp.main_stack.get();

        match view {
            MainView::Language => main_stack.set_visible_child(&*imp.language_view),
            MainView::Inventory => main_stack.set_visible_child(&*imp.inventory_view),
            MainView::Dictionary => main_stack.set_visible_child(&*imp.dictionary_view),
            MainView::Unknown => log::warn!("Attempting to switch to unknown view."),
        }

        imp.header_bar.set_flat(false);
        imp.action_bar.remove_css_class("flat");
        imp.selected_view.set(view);
    }

    /// Loads view state from the project model.
    pub fn load_view_state(&self, view: MainView) {
        let imp = self.imp();

        match view {
            MainView::Language => imp.language_view.load_state(),
            MainView::Inventory => imp.inventory_view.load_state(),
            MainView::Dictionary => imp.dictionary_view.load_state(),
            MainView::Unknown => log::warn!("Attempting to load unknown view."),
        }
    }

    /// Loads all view states from the project model.
    pub fn load_all_views(&self) {
        for view in ui::MainView::SELECTABLES.iter() {
            self.load_view_state(*view);
        }
    }

    /// Commits view state to the project model.
    pub fn commit_view_state(&self, view: MainView) {
        let imp = self.imp();

        match view {
            MainView::Language => imp.language_view.commit_state(),
            MainView::Inventory => imp.inventory_view.commit_state(),
            MainView::Dictionary => imp.dictionary_view.commit_state(),
            MainView::Unknown => {} // _ => log::warn!("Attempting to commit unknown view."),
        }
    }

    /// Commits all view states to the project model.
    pub fn commit_all_views(&self) {
        for view in ui::MainView::SELECTABLES.iter() {
            self.commit_view_state(*view);
        }
    }

    /// Unloads view state from the project model.
    pub fn unload_view_state(&self, view: MainView) {
        let imp = self.imp();

        match view {
            MainView::Language => imp.language_view.unload_state(),
            MainView::Inventory => imp.inventory_view.unload_state(),
            MainView::Dictionary => imp.dictionary_view.unload_state(),
            MainView::Unknown => log::warn!("Attempting to load unknown view."),
        }
    }

    /// Unloads all view states from the project model.
    pub fn unload_all_views(&self) {
        for view in ui::MainView::SELECTABLES.iter() {
            self.unload_view_state(*view);
        }
    }

    pub fn go_back(&self) {
        let imp = self.imp();
        let view = imp.selected_view.get();

        match view {
            MainView::Language => imp.language_view.go_back(),
            MainView::Inventory => imp.inventory_view.go_back(),
            MainView::Dictionary => imp.dictionary_view.go_back(),
            MainView::Unknown => {}
        }
    }

    pub fn on_window_size(&self, width: i32, _height: i32) {
        if self.is_realized() {
            let narrow = width <= 600;
            self.set_narrow(narrow);
        }
    }
}
