use glib::Properties;
use glib::subclass::InitializingObject;
use gtk4::CompositeTemplate;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;

use std::cell::RefCell;

use ningyo_extensions::prelude::*;

#[derive(CompositeTemplate, Default, Properties)]
#[template(resource = "/live/arcturus/ningyotsukai/bindings/form.ui")]
#[properties(wrapper_type=BindingForm)]
pub struct BindingFormImp {
    #[template_child]
    name: gtk4::TemplateChild<gtk4::Label>,
    #[template_child]
    tracker_param_factory: gtk4::TemplateChild<gtk4::SignalListItemFactory>,
    #[template_child]
    tracker_param_select: gtk4::TemplateChild<gtk4::SingleSelection>,
    #[template_child]
    tracker_param_model: gtk4::TemplateChild<gio::ListStore>,
    #[template_child]
    dampening_entry: gtk4::TemplateChild<gtk4::Entry>,
    #[template_child]
    value_in_from_entry: gtk4::TemplateChild<gtk4::Entry>,
    #[template_child]
    value_in_to_entry: gtk4::TemplateChild<gtk4::Entry>,
    #[template_child]
    value_in_display: gtk4::TemplateChild<gtk4::Range>,
    #[template_child]
    value_out_from_entry: gtk4::TemplateChild<gtk4::Entry>,
    #[template_child]
    value_out_to_entry: gtk4::TemplateChild<gtk4::Entry>,
    #[template_child]
    value_out_display: gtk4::TemplateChild<gtk4::Range>,

    #[property(name="binding-name", get=|me: &&BindingFormImp| { me.name.label().into() }, set=|me: &&BindingFormImp, label: &str| { me.name.set_label(&label.escape_nulls()); })]
    _synths: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for BindingFormImp {
    const NAME: &'static str = "NGTBindingForm";
    type Type = BindingForm;
    type ParentType = gtk4::Grid;

    fn class_init(class: &mut Self::Class) {
        class.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for BindingFormImp {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for BindingFormImp {}

impl GridImpl for BindingFormImp {}

impl BindingFormImp {}

glib::wrapper! {
    pub struct BindingForm(ObjectSubclass<BindingFormImp>)
        @extends gtk4::Grid, gtk4::Widget,
        @implements gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Accessible, gtk4::Orientable;
}

impl BindingForm {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
