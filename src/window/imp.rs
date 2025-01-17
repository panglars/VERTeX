use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, Box, CompositeTemplate, Entry, ListBox, ListView};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/panglars/vertex/window.ui")]
pub struct Window {
    #[template_child]
    pub model_panel: TemplateChild<ListBox>,
    #[template_child]
    pub sidebar_list: TemplateChild<Box>,
    #[template_child]
    pub messages_list: TemplateChild<ListView>,
    #[template_child]
    pub message_entry: TemplateChild<Entry>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "VERTeX";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
