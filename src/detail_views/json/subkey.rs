use glib::subclass::InitializingObject;
use glib::subclass::prelude::*;
use glib::wrapper;

use std::cell::RefCell;

use crate::navigation_item::JsonNavigationPath;

#[derive(Default)]
pub struct SubkeyImp(RefCell<Option<JsonNavigationPath>>);

#[glib::object_subclass]
impl ObjectSubclass for SubkeyImp {
    const NAME: &'static str = "PIPuppetJsonSubkey";
    type Type = Subkey;
    type ParentType = glib::Object;

    fn class_init(class: &mut Self::Class) {}

    fn instance_init(obj: &InitializingObject<Self>) {}
}

impl ObjectImpl for SubkeyImp {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

glib::wrapper! {
    pub struct Subkey(ObjectSubclass<SubkeyImp>);
}

impl Subkey {
    pub fn new_object_key(key: String) -> Self {
        let selfish: Subkey = glib::Object::builder().build();

        *selfish.imp().0.borrow_mut() = Some(JsonNavigationPath::ObjectKey(key));

        selfish
    }

    pub fn new_list_index(index: usize) -> Self {
        let selfish: Subkey = glib::Object::builder().build();

        *selfish.imp().0.borrow_mut() = Some(JsonNavigationPath::ListIndex(index));

        selfish
    }

    pub fn as_jsonnavpath(&self) -> JsonNavigationPath {
        self.imp().0.borrow().as_ref().unwrap().clone()
    }
}
