mod audio;
mod gui;
use gtk::prelude::*;
use gtk::{glib, Application};

const APP_ID: &str = "org.gtk_rs.VTracker";

fn main() -> glib::ExitCode {
       // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(gui::window::build_ui);

    // Run the application
    app.run()
}

