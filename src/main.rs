mod task_object;
mod task_row;
mod window;

use gtk::prelude::*;
use gtk::{gio, glib};
use window::Window;

const APP_ID: &'static str = "org.panglars.VERTeX";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("vertex.gresource").expect("Failed to register resources.");

    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &adw::Application) {
    // Create a new custom window and present it
    let window = Window::new(app);
    window.present();
}
