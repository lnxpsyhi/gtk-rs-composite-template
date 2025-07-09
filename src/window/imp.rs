use {
    crate::custom_button::CustomButton,
    adw::{prelude::*, subclass::prelude::*},
    gtk::{
        CompositeTemplate, Label,
        gio::Settings,
        glib::{self, subclass::InitializingObject},
    },
    std::{
        cell::{Cell, OnceCell},
        rc::Rc,
    },
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/my_gtk_app/MyGtkApp/window.ui")]
pub struct Window {
    pub settings: OnceCell<Settings>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkApp";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        // Register 'CustomButton'

        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        // Connect to "clicked" signal of `button_increase`
        // self.button_increase.connect_clicked(clone!(
        //     #[weak(rename_to = obj)]
        //     self,
        //     move |_| {
        //         obj.counter.set(obj.counter.get() + 1);
        //         obj.label_counter.set_label(&obj.counter.get().to_string());
        //     }
        // ));

        // Call "constructed" on parent
        self.parent_constructed();

        // setup
        let obj = self.obj();
        obj.setup_settings();
        let settings = obj.settings();
    }
}

#[gtk::template_callbacks]
impl Window {
    // #[template_callback]
    // fn handle_button_clicked(&self, button: &CustomButton) {
    //     let number_increased = self.number.get() + 1;
    //     self.number.set(number_increased);
    //     button.set_label(&number_increased.to_string());
    // }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {
    // fn close_request(&self) -> glib::Propagation {
    //     println!("Closing..");
    //     self.parent_close_request()
    // }
}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}

impl AdwApplicationWindowImpl for Window {}
