use std::path::Path;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use adw::subclass::prelude::*;

use crate::config;
use crate::ui;

#[doc(hidden)]
mod imp {
    use gtk::glib::WeakRef;

    use super::*;

    #[derive(Default)]
    pub struct Application {
        pub window: WeakRef<ui::ApplicationWindow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "KhzApplication";
        type Type = super::Application;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for Application {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.setup_shortcuts();
        }
    }

    impl ApplicationImpl for Application {
        fn startup(&self) {
            self.parent_startup();
            let obj = self.obj();
            obj.startup();
        }

        fn activate(&self) {
            self.parent_activate();
            let obj = self.obj();
            obj.window().present();
        }

        fn shutdown(&self) {
            self.parent_shutdown();
            let obj = self.obj();
            obj.cleanup();
        }
    }

    impl GtkApplicationImpl for Application {}

    impl AdwApplicationImpl for Application {}
}

glib::wrapper! {
    /// The application.
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends adw::Application, gtk::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Application {
    /// Creates a new application
    pub fn new(app_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", app_id)
            .property("flags", flags)
            .build()
    }

    pub fn builder() -> glib::object::ObjectBuilder<'static, Self> {
        glib::Object::builder()
    }

    // WINDOW MANAGEMENT

    /// Gets a window for the application or creates one if none exists.
    fn window(&self) -> ui::ApplicationWindow {
        let imp = self.imp();

        if let Some(window) = imp.window.upgrade() {
            return window;
        }

        log::info!("Creating window.");
        self.add_new_window()
    }

    /// Creates a new window, sets it up, and adds it to the application.
    fn add_new_window(&self) -> ui::ApplicationWindow {
        let window = ui::ApplicationWindow::new(self);

        window.connect_closure(
            "open-project",
            false,
            glib::closure_local!(@strong self as app => move |window: &ui::ApplicationWindow, path: &str| {
                app.handle_open_project(window, path);
            })
        );

        window.connect_closure(
            "new-project",
            false,
            glib::closure_local!(@strong self as app => move |window: &ui::ApplicationWindow| {
                app.handle_new_project(window);
            }),
        );

        self.add_window(&window);
        window
    }

    // CALLBACKS

    /// Called on application startup.
    fn startup(&self) {
        log::info!("Starting up.");
    }

    /// Setup `GAction`s for the application.
    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]);
    }

    /// Setup shortcuts for the application.
    fn setup_shortcuts(&self) {
        self.set_accels_for_action("app.quit", &["<primary>q"]);
        self.set_accels_for_action("win.open", &["<primary>o"]);
        self.set_accels_for_action("win.new", &["<primary>n"]);
    }

    /// Called on application shutdown.
    fn cleanup(&self) {
        log::info!("Shutting down.");
    }

    /// Show about window for the application
    fn show_about(&self) {
        if let Some(window) = self.active_window() {
            let about = adw::AboutWindow::builder()
                .transient_for(&window)
                .application_name(config::APP_NAME)
                // .application_icon(config::APP_ID)
                .application_icon("accessories-dictionary-symbolic")
                .developer_name(config::AUTHOR)
                .version(config::VERSION)
                .developers(vec![config::AUTHOR.to_string()])
                .copyright("2023 Manenfu")
                .build();

            about.present();
        }
    }

    // HANDLERS

    /// Handles `open-project` signal for an `ApplicationWindow`.
    /// If the current window already has a project opened, the handler will
    /// spawn a new window.
    fn handle_open_project<P: AsRef<Path>>(&self, window: &ui::ApplicationWindow, path: P) {
        let p = path.as_ref();
        log::info!("Opening project {:?}", p);
        window.open_project_file(path);
    }

    /// Handles `new-project` signal for an `ApplicationWindow`.
    /// If the current window already has a project opened, the handler will
    /// spawn a new window.
    fn handle_new_project(&self, window: &ui::ApplicationWindow) {
        log::info!("Creating new project.");
        window.new_project();
    }
}
