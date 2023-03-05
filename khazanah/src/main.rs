use gtk::gio;
use gtk::prelude::*;

mod application;
mod config;
mod models;
mod ui;

/// Entry point.
fn main() {
    env_logger::init();

    log::info!(
        "{} {} ({})",
        config::APP_NAME,
        config::VERSION,
        config::APP_ID
    );
    log::info!("Author: {}", config::AUTHOR);
    log::info!("Report bugs and issues at {}", config::HOMEPAGE);

    // Initialize stuffs
    gtk::init()
        .map_err(|e| {
            log::error!("GTK initialization failed.");
            e
        })
        .expect("GTK initialization failed.");

    adw::init()
        .map_err(|e| {
            log::error!("libadwaita initialization failed.");
            e
        })
        .expect("libadwaita initialization failed.");

    // Load resources first
    gio::resources_register_include!("resources.gresource")
        .map_err(|e| {
            log::error!("Failed to register resources.");
            e
        })
        .expect("Failed to register resources.");

    // Main application
    let app = application::Application::new(config::APP_ID, &gio::ApplicationFlags::empty());

    // Run the application
    log::info!("Starting application.");
    std::process::exit(app.run().into());
}
