mod imp;

use glib::{BindingFlags, Object};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::message_object::MessageObject;

glib::wrapper! {
    pub struct MessageRow(ObjectSubclass<imp::MessageRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for MessageRow {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, message_object: &MessageObject) {}

    pub fn unbind(&self) {}
}
