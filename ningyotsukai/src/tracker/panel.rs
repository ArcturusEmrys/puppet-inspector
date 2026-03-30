use gio;
use glib;
use gtk4;

use glib::subclass::InitializingObject;
use gtk4::CompositeTemplate;
use gtk4::subclass::prelude::*;

use crate::document::DocumentController;
use crate::tracker::TrackerManager;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/live/arcturus/ningyotsukai/tracker/panel.ui")]
pub struct TrackerPanelImp {
    tracker_manager: RefCell<Option<Rc<TrackerManager>>>,
}

#[glib::object_subclass]
impl ObjectSubclass for TrackerPanelImp {
    const NAME: &'static str = "NGTTrackerPanel";
    type Type = TrackerPanel;
    type ParentType = gtk4::Box;

    fn class_init(class: &mut Self::Class) {
        class.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for TrackerPanelImp {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for TrackerPanelImp {}

impl BoxImpl for TrackerPanelImp {}

glib::wrapper! {
    pub struct TrackerPanel(ObjectSubclass<TrackerPanelImp>)
        @extends gtk4::Box, gtk4::Widget,
        @implements gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Accessible;
}

impl TrackerPanel {
    pub fn new(tracker_manager: Rc<TrackerManager>) -> Self {
        let selfish: TrackerPanel = glib::Object::builder().build();

        *selfish.imp().tracker_manager.borrow_mut() = Some(tracker_manager);

        selfish
    }

    fn bind(&self) {}
}
