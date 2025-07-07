use gtk::{Application, CssProvider, gio::MenuModel, glib, prelude::*};

use crate::window::Window;

mod window;

const APP_ID: &'static str = "org.my_gtk_app.MyGtkApp";

fn main() -> glib::ExitCode {
    // build.rs target is relative to OUTDIR
    gtk::gio::resources_register_include!("my_gtk_app.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    // load css on app startup
    app.connect_startup(|_| load_css());

    app.connect_activate(build_ui);

    app.run()
}

pub fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("../resources/style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
