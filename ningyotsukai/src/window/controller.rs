use gio;
use glib;
use gtk4;

use glib::subclass::InitializingObject;
use gtk4::CompositeTemplate;
use gtk4::subclass::prelude::*;

use std::cell::RefCell;

/// For some reason, glib-rs does not support mutating private/impl structs.
/// Hence the mutability hack.
#[derive(Default)]
pub struct WindowControllerState {
}

#[derive(CompositeTemplate, Default)]
#[template(resource = "/live/arcturus/ningyotsukai/window/controller.ui")]
pub struct WindowControllerImp {
    #[template_child]
    main_menu: TemplateChild<gio::MenuModel>,
    #[template_child]
    main_menu_button: TemplateChild<gtk4::MenuButton>,
    state: RefCell<WindowControllerState>,
}

#[glib::object_subclass]
impl ObjectSubclass for WindowControllerImp {
    const NAME: &'static str = "NGTWindowController";
    type Type = WindowController;
    type ParentType = gtk4::ApplicationWindow;

    fn class_init(class: &mut Self::Class) {
        class.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for WindowControllerImp {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for WindowControllerImp {}

impl WindowImpl for WindowControllerImp {}

impl ApplicationWindowImpl for WindowControllerImp {}

glib::wrapper! {
    pub struct WindowController(ObjectSubclass<WindowControllerImp>)
        @extends gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk4::Accessible, gtk4::Buildable,
                    gtk4::ConstraintTarget, gtk4::Native, gtk4::Root, gtk4::ShortcutManager;
}

impl WindowController {
    pub fn new(app: &gtk4::Application) -> Self {
        let selfish: WindowController =
            glib::Object::builder().property("application", app).build();

        let main_menu = selfish.imp().main_menu.clone();
        let main_menu_button = selfish.imp().main_menu_button.clone();
        main_menu_button.set_menu_model(Some(&main_menu));

        selfish
    }
}
