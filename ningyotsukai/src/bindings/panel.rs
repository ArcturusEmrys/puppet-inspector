use glib::WeakRef;
use glib::subclass::InitializingObject;
use gtk4::CompositeTemplate;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;

use crate::bindings::form::BindingForm;
use crate::document::Document;
use crate::stage::{StageWidget, StageWidgetExt};
use generational_arena::Index;

use std::cell::RefCell;
use std::sync::{Arc, Mutex};

struct State {
    document: Arc<Mutex<Document>>,

    stage: WeakRef<StageWidget>,

    current_selection: Option<Index>,
}

#[derive(CompositeTemplate, Default)]
#[template(resource = "/live/arcturus/ningyotsukai/bindings/panel.ui")]
pub struct BindingPanelImp {
    state: RefCell<Option<State>>,

    #[template_child]
    bindings_contents: gtk4::TemplateChild<gtk4::Box>,
}

#[glib::object_subclass]
impl ObjectSubclass for BindingPanelImp {
    const NAME: &'static str = "NGTBindingPanel";
    type Type = BindingPanel;
    type ParentType = gtk4::Box;

    fn class_init(class: &mut Self::Class) {
        class.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for BindingPanelImp {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for BindingPanelImp {}

impl BoxImpl for BindingPanelImp {}

impl BindingPanelImp {
    fn populate_list(&self) {
        let mut state = self.state.borrow_mut();
        let state = state.as_mut().unwrap();

        // We make no attempt at showing multiple selected puppets' bindings
        // at once. We probably should treat this as "no selection" and clear
        // everything.
        let stage = state.stage.upgrade().unwrap();
        let new_selection = stage.selection().iter().next().copied();
        if new_selection == state.current_selection {
            // We can get spurious selection notifications, in which case do
            // nothing.
            return;
        }

        state.current_selection = new_selection;

        while let Some(child) = self.bindings_contents.first_child() {
            self.bindings_contents.remove(&child);
        }

        let document = state.document.lock().unwrap();
        let mut widgets = vec![];

        if let Some(index) = new_selection {
            if let Some(puppet) = document.stage().puppet(index) {
                for (binding_index, binding) in puppet.bindings().iter().enumerate() {
                    let form = BindingForm::new();

                    form.set_binding_name(binding.name.as_str());

                    self.bindings_contents.append(&form);
                    widgets.push((form, index, binding_index));
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct BindingPanel(ObjectSubclass<BindingPanelImp>)
        @extends gtk4::Box, gtk4::Widget,
        @implements gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Accessible;
}

impl BindingPanel {
    pub fn bind(&self, document: Arc<Mutex<Document>>, stage: StageWidget) {
        *self.imp().state.borrow_mut() = Some(State {
            document,
            stage: stage.downgrade(),
            current_selection: None,
        });

        stage.connect_selection_changed({
            let callback_self = self.clone().downgrade();
            move |_| {
                if let Some(callback_self) = callback_self.upgrade() {
                    callback_self.imp().populate_list();
                }
            }
        });

        self.imp().populate_list();
    }
}
