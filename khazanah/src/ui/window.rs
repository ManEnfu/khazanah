use std::path::Path;

use conlang::Project;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use adw::subclass::prelude::*;
    
use crate::ui;

mod imp {
    use std::cell::RefCell;

    use gtk::glib::{
        once_cell::sync::Lazy,
        subclass::{Signal, SignalType},
    };

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/window.ui")]
    pub struct ApplicationWindow {
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        // #[template_child]
        // pub start_controls: TemplateChild<ui::ToolbarStartControls>,
        // #[template_child]
        // pub end_controls: TemplateChild<ui::ToolbarEndControls>,
        #[template_child]
        pub start_view: TemplateChild<ui::StartView>,

        pub file_dialog: RefCell<Option<gtk::FileChooserNative>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ApplicationWindow {
        const NAME: &'static str = "KhzApplicationWindow";
        type Type = super::ApplicationWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
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

    impl WidgetImpl for ApplicationWindow {}

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

impl ApplicationWindow {
    /// Creates a new window.
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    /// Setups `GAction`s for the window.
    fn setup_gactions(&self) {
        let open_action = gio::ActionEntry::builder("open")
            .activate(Self::open_file_dialog)
            .build();
        let new_action = gio::ActionEntry::builder("new")
            .activate(|window: &Self, _, _| {
                window.emit_by_name::<()>("new-project", &[]);
            })
            .build();

        self.add_action_entries([open_action, new_action]);
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
        let ctx = glib::MainContext::default();
        let self_weak = glib::SendWeakRef::from(self.downgrade());

        let path = path.as_ref().to_path_buf();
        ctx.spawn_local(async move {
            if let Some(window) = self_weak.upgrade() {
                match Project::load_file(&path) {
                    Ok(project) => {
                        log::info!("Project opened: {:?}", &path);
                        window.set_project(project);
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
                            "Unable to open \"{}\"", 
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

    /// Creates a new projet and sets it as the current project.
    pub fn new_project(&self) {
        self.set_project(Project::new());
        self.imp()
            .toast_overlay
            .add_toast(adw::Toast::new("Created a New Project"));
    }

    /// Sets the current project of this window.
    fn set_project(&self, project: Project) {
        let title = match project.file_path() {
            Some(p) => p.to_str().unwrap_or("Unknown"),
            None => "New Project",
        };
        self.set_title(Some(title));
    }
}
