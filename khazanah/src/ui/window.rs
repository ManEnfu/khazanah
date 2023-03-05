use std::path::Path;

use conlang::Project;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use adw::subclass::prelude::*;

mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/window.ui")]
    pub struct ApplicationWindow {
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

        self.add_action_entries([open_action]);
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
                    match dialog.file() {
                        Some(f) => match f.path() {
                            Some(p) => {
                                log::info!("Opening {:?}.", &p);
                                window.open_project_file(p);
                            }
                            None => log::error!("Invalid path.")
                        }
                        None => log::error!("No file requested."),
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
    fn open_project_file<P: AsRef<Path> + 'static>(&self, path: P) {
        let ctx = glib::MainContext::default();
        let self_weak = glib::SendWeakRef::from(self.downgrade());

        ctx.spawn_local(async move {
            if let Some(window) = self_weak.upgrade() {
                match Project::load_file(path) {
                    Ok(project) => {
                        log::info!("Project opened.");
                        window.set_title(Some("hello"));
                        window.set_project(project);
                    }
                    Err(e) => log::error!("Error opening file: {:?}", e),
                }
            }
        });
    }

    /// Sets the current project of this window.
    fn set_project(&self, project: Project) {
        // todo!()
    }
}
