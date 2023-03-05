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
        fn activate(&self) {
            self.parent_activate();
            let obj = self.obj();
            obj.window().present();
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

    /// Gets a window for the application or creates one if none exists.
    fn window(&self) -> ui::ApplicationWindow {
        let imp = self.imp();

        if let Some(window) = imp.window.upgrade() {
            return window;
        }

        log::info!("Creating window.");
        let window = ui::ApplicationWindow::new(self);
        self.add_window(&window);
        imp.window.set(Some(&window));
        window
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
}
