#[allow(unused_imports)]
use {
    gtk::{
        Button, CompositeTemplate, Grid, Label, PasswordEntry, Switch, TemplateChild,
        gio::Settings,
        glib::{
            self, clone,
            subclass::{
                InitializingObject,
                object::{ObjectImpl, ObjectImplExt},
                types::{ObjectSubclass, ObjectSubclassExt},
            },
        },
        prelude::*,
        subclass::{
            prelude::ApplicationWindowImpl,
            widget::{
                CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetClassExt,
                WidgetImpl,
            },
            window::WindowImpl,
        },
    },
    std::{
        cell::{Cell, OnceCell},
        rc::Rc,
    },
};
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/my_gtk_app/MyGtkApp/window.ui")]
pub struct Window {
    #[template_child]
    pub my_switch: TemplateChild<Switch>,
    #[template_child]
    pub my_grid: TemplateChild<Grid>,
    #[template_child]
    pub my_password_entry: TemplateChild<PasswordEntry>,
    pub settings: OnceCell<Settings>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkApp";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[allow(unused_variables)]
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // setup
        let obj = self.obj();
        obj.setup_settings();

        let settings = obj.settings();
        // Connect to "clicked" signal of `button_increase`
        // self.button_increase.connect_clicked(clone!(
        //     #[weak(rename_to = obj)]
        //     self,
        //     move |_| {
        //         obj.counter.set(obj.counter.get() + 1);
        //         obj.label_counter.set_label(&obj.counter.get().to_string());
        //     }
        // ));
    }
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
