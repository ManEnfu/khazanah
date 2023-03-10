use std::path::{Path, PathBuf};

use conlang::Project;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use adw::subclass::prelude::*;

use crate::ui::{self, MainViews, View};

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
        pub project_overview_view: TemplateChild<ui::ProjectOverviewView>,
        #[template_child]
        pub project_lexicon_view: TemplateChild<ui::ProjectLexiconView>,
        
        #[template_child]
        pub view_switcher: TemplateChild<ui::ViewSwitcherDropDown>,
        #[template_child]
        pub start_controls: TemplateChild<ui::ToolbarStartControls>,
        #[template_child]
        pub end_controls: TemplateChild<ui::ToolbarEndControls>,
        #[template_child]
        pub main_menu_button: TemplateChild<ui::MainMenuButton>,

        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
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

        pub current_view_index: Cell<MainViews>,

        pub file_dialog: RefCell<Option<gtk::FileChooserNative>>,
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
            // Send the current window width and height to all views.
            self.obj().on_window_size(width, height);

            self.parent_size_allocate(width, height, baseline);
        }
    }

    impl WindowImpl for ApplicationWindow {}

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
            .activate(Self::open_file_dialog)
            .build();
        // New project
        let new_action = gio::ActionEntry::builder("new")
            .activate(|window: &Self, _, _| {
                window.emit_by_name::<()>("new-project", &[]);
            })
            .build();
        // Save project
        let save_action = gio::ActionEntry::builder("save")
            .activate(|window: &Self, action, v| {
                if window.project_model().project().is_none() {
                    return;
                }
                match window.project_model().path() {
                    Some(path) => window.save_project_file(path),
                    None => window.save_file_dialog(action, v),
                }
            })
            .build();
        // Save project as another file
        let save_as_action = gio::ActionEntry::builder("save-as")
            .activate(|window: &Self, action, v| {
                if window.project_model().project().is_some() {
                    window.save_file_dialog(action, v);
                }
            })
            .build();

        self.add_action_entries([open_action, new_action, save_action, save_as_action]);
        self.action_set_enabled("win.save", false);
        self.action_set_enabled("win.save-as", false);
    }

    /// Shows `Open File` dialog.
    fn open_file_dialog(&self, _action: &gio::SimpleAction, _v: Option<&glib::Variant>) {
        let imp = self.imp();
        // Skip if dialog is already opened
        if imp.file_dialog.borrow().is_some() {
            return;
        }

        let dialog = gtk::FileChooserNative::builder()
            .title("Open File")
            .transient_for(self)
            .action(gtk::FileChooserAction::Open)
            .accept_label("_Open")
            .cancel_label("_Cancel")
            .build();

        dialog.connect_response(
            glib::clone!(@strong self as window => move |dialog, response| {
                if response == gtk::ResponseType::Accept {
                    if let Some(Some(path)) = dialog.file().map(|f| f.path()) {
                        window.emit_by_name::<()>(
                            "open-project",
                            &[&path.to_string_lossy().to_string()]
                        );
                    } else {
                        log::error!("Invalid file.");
                    }
                }
                window.imp().file_dialog.replace(None);
            }),
        );

        imp.file_dialog.replace(Some(dialog.clone()));

        log::info!("Open file dialog.");
        dialog.show();
    }

    /// Opens a project file for this window.
    pub fn open_project_file<P: AsRef<Path>>(&self, path: P) {
        log::info!("Opening file: {:?}", path.as_ref());

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
        self.project_model().set_project(Some(Project::new()));
        self.finish_open_project();
        self.imp()
            .toast_overlay
            .add_toast(adw::Toast::new("Created a New Project"));
    }

    /// Called after a project is opened or created for this window.
    fn finish_open_project(&self) {
        if self.project_model().project().is_some() {
            self.set_project_opened(true);
            self.action_set_enabled("win.save", true);
            self.action_set_enabled("win.save-as", true);

            self.update_title();
            self.load_all_views();

            self.switch_view(MainViews::Overview);
        }
    }

    /// Shows `Save File` dialog.
    fn save_file_dialog(&self, _action: &gio::SimpleAction, _v: Option<&glib::Variant>) {
        let imp = self.imp();
        // Skip if dialog is already opened
        if imp.file_dialog.borrow().is_some() {
            return;
        }

        let dialog = gtk::FileChooserNative::builder()
            .title("Save File")
            .transient_for(self)
            .action(gtk::FileChooserAction::Save)
            .accept_label("_Save")
            .cancel_label("_Cancel")
            .build();

        dialog.connect_response(
            glib::clone!(@strong self as window => move |dialog, response| {
                if response == gtk::ResponseType::Accept {
                    if let Some(Some(path)) = dialog.file().map(|f| f.path()) {
                        window.save_project_file(path);
                    } else {
                        log::error!("Invalid file.");
                    }
                }
                window.imp().file_dialog.replace(None);
            }),
        );

        imp.file_dialog.replace(Some(dialog.clone()));

        log::info!("Open file dialog.");
        dialog.show();
    }

    /// Save the current project.
    pub fn save_project_file<P: AsRef<Path>>(&self, path: P) {
        log::info!("Saving file: {:?}", path.as_ref());

        self.commit_all_views();

        let ctx = glib::MainContext::default();
        let self_weak = glib::SendWeakRef::from(self.downgrade());

        let path = path.as_ref().to_path_buf();
        ctx.spawn_local(async move {
            if let Some(window) = self_weak.upgrade() {
                match window.project_model().save_file(&path) {
                    Ok(_) => {
                        log::info!("Project Saved: {:?}", &path);
                        let msg = format!(
                            "Saved \"{}\"",
                            path.file_name()
                                .unwrap_or_default()
                                .to_str()
                                .unwrap_or_default()
                        );
                        window.imp().toast_overlay.add_toast(adw::Toast::new(&msg));
                        window.update_title();
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

    // VIEWS

    // Updates window title. 
    // If a project is opened, shows project name and file name as well.
    pub fn update_title(&self) {
        if self.project_model().project().is_none() {
            self.set_title(Some("Khazanah"));
            return;
        }

        let title = if let Some(p) = self.project_model().path() {
            let file = PathBuf::from(p)
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string();
            let name = &self
                .project_model()
                .project()
                .as_ref()
                .unwrap()
                .meta()
                .name
                .to_owned();
            format!("{} [{}] - Khazanah", name, file)
        } else {
            "New Project - Khazanah".to_string()
        };

        self.set_title(Some(&title));
    }

    /// Switches to a view. This will set an internal property to sync with all view switchers in
    /// the window.
    pub fn switch_view(&self, view: MainViews) {
        if view != MainViews::Unknown {
            self.set_selected_view_index(u32::from(view));
        }
    }

    /// Responds to change of view selection by switching to said view.
    #[template_callback]
    fn handle_selected_view_index_changed(&self, _pspec: glib::ParamSpec, _s: &Self) {
        let idx = self.selected_view_index();
        let view = MainViews::from(idx);
        log::debug!("Switching to view: {:?} ({})", view, idx);
        
        let imp = self.imp();
        let current_view = imp.current_view_index.get();

        if current_view != MainViews::Unknown {
            self.commit_view_state(current_view);
        }

        self.load_view_state(view);
        let main_stack = imp.main_stack.get();

        match view {
            MainViews::Overview => main_stack.set_visible_child(&*imp.project_overview_view),
            MainViews::Lexicon => main_stack.set_visible_child(&*imp.project_lexicon_view),
            _ => log::warn!("Attempting to switch to unknown view."),
        }
    
        imp.header_bar.remove_css_class("flat");
        imp.current_view_index.set(view);
    }
    
    /// Loads view state from the project model.
    pub fn load_view_state(&self, view: MainViews) {
        log::debug!("Loading view state: {:?}", view);
        let imp = self.imp();

        match view {
            MainViews::Overview => imp.project_overview_view.load_state(),
            MainViews::Lexicon => imp.project_overview_view.load_state(),
            _ => log::warn!("Attempting to load unknown view."),
        }
    }

    /// Loads all view states from the project model.
    pub fn load_all_views(&self) {
        for view in ui::ALL_MAIN_VIEWS.iter() {
            self.load_view_state(*view);
        }
    }

    /// Commits view state to the project model.
    pub fn commit_view_state(&self, view: MainViews) {
        log::debug!("Committing view state: {:?}", view);
        let imp = self.imp();

        match view {
            MainViews::Overview => imp.project_overview_view.commit_state(),
            MainViews::Lexicon => imp.project_overview_view.commit_state(),
            MainViews::Unknown => {}
            _ => log::warn!("Attempting to commit unknown view."),
        }
    }

    /// Commits all view states to the project model.
    pub fn commit_all_views(&self) {
        for view in ui::ALL_MAIN_VIEWS.iter() {
            self.commit_view_state(*view);
        }
    }
    
    pub fn on_window_size(&self, width: i32, _height: i32) {
        if self.is_realized() {
            let new_narrow = width <= 600;
            self.set_narrow(new_narrow);
        }
    }
}
