mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct MessageObject(ObjectSubclass<imp::MessageObject>);
}

#[derive(Default)]
pub struct MessageData {
    pub user: String,
    pub content: String,
}

impl MessageObject {
    pub fn new(user: String, content: String) -> Self {
        Object::builder()
            .property("user", user)
            .property("content", content)
            .build()
    }
}
