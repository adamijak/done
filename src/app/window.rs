use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk4 as gtk;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct DoneWindow;

    #[glib::object_subclass]
    impl ObjectSubclass for DoneWindow {
        const NAME: &'static str = "DoneWindow";
        type Type = super::DoneWindow;
        type ParentType = gtk::ApplicationWindow;
    }

    impl ObjectImpl for DoneWindow {}
    impl WidgetImpl for DoneWindow {}
    impl WindowImpl for DoneWindow {}
    impl ApplicationWindowImpl for DoneWindow {}
}

glib::wrapper! {
    pub struct DoneWindow(ObjectSubclass<imp::DoneWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl DoneWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)]).expect("Failed to create DoneWindow")
    }
}
