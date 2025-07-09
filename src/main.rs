use {crate::window::Window, adw::prelude::*, gtk::glib};

pub mod custom_button;
mod window;

const APP_ID: &'static str = "org.my_gtk_app.MyGtkApp";

fn main() -> glib::ExitCode {
    // build.rs target is relative to OUTDIR
    gtk::gio::resources_register_include!("my_gtk_app.gresource")
        .expect("Failed to register resources.");

    let app = adw::Application::builder().application_id(APP_ID).build();

    // load css on app startup
    app.connect_startup(|_| load_appropriate_css());

    app.connect_activate(build_ui);

    app.run()
}

pub fn build_ui(app: &adw::Application) {
    let window = Window::new(app);
    window.present();
}

fn load_appropriate_css() {
    let display = gtk::gdk::Display::default().expect("Failed to get default display");
    let provider = gtk::CssProvider::new();
    let manager = adw::StyleManager::default();

    let css_path = if manager.is_high_contrast() {
        if manager.is_dark() {
            "/org/my_gtk_app/MyGtkApp/style-hc-dark.css"
        } else {
            "/org/my_gtk_app/MyGtkApp/style-hc.css"
        }
    } else if manager.is_dark() {
        "/org/my_gtk_app/MyGtkApp/style-dark.css"
    } else {
        "/org/my_gtk_app/MyGtkApp/style.css"
    };

    provider.load_from_resource(css_path);

    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
