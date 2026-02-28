use glib;
use gtk4;
use gtk4::CompositeTemplate;
use gtk4::subclass::prelude::*;

use glib::subclass::InitializingObject;

use std::cell::RefCell;
use std::sync::Arc;

use crate::document::Document;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/live/arcturus/puppet-inspector/metadata_inspector.ui")]
pub struct MetadataInspectorImp {
    document: RefCell<Option<Arc<Document>>>,
}

#[glib::object_subclass]
impl ObjectSubclass for MetadataInspectorImp {
    const NAME: &'static str = "PIPuppetMetadataInspector";
    type Type = MetadataInspector;
    type ParentType = gtk4::Box;

    fn class_init(class: &mut Self::Class) {
        class.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MetadataInspectorImp {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for MetadataInspectorImp {}

impl BoxImpl for MetadataInspectorImp {}

glib::wrapper! {
    pub struct MetadataInspector(ObjectSubclass<MetadataInspectorImp>)
        @extends gtk4::Box, gtk4::Widget,
        @implements gtk4::Buildable, gtk4::Orientable, gtk4::ConstraintTarget, gtk4::Accessible;
}

impl MetadataInspector {
    pub fn new(document: Arc<Document>) -> Self {
        let selfish: Self = glib::Object::builder().build();

        *selfish.imp().document.borrow_mut() = Some(document);

        selfish
    }
}
