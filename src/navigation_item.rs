use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use glib;

use std::cell::RefCell;

pub enum Section {
    PuppetMeta,
    PuppetPhysics,
    PuppetNode,
    PuppetParams,
    ModelTextures,
    VendorData,
}

pub enum PathComponent {
    Section(Section)
}

#[derive(Default)]
pub struct NavigationItemImp {
    pub path: RefCell<Option<PathComponent>>
}

#[glib::object_subclass]
impl ObjectSubclass for NavigationItemImp {
    const NAME: &'static str = "INPExNavigationItem";
    type Type = NavigationItem;
}

impl ObjectImpl for NavigationItemImp {

}

glib::wrapper! {
    pub struct NavigationItem(ObjectSubclass<NavigationItemImp>);
}

impl NavigationItem {
    pub fn new(path: PathComponent) -> Self {
        let selfpoi: Self = glib::Object::builder().build();

        *(selfpoi.imp().path.borrow_mut()) = Some(path);

        selfpoi
    }
}