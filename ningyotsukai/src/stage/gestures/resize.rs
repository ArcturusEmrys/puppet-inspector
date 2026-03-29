use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use generational_arena::Index;

use crate::document::Document;
use crate::stage::StageWidget;
use gtk4::prelude::*;

pub struct ResizeGestures(Rc<RefCell<ResizeGesturesImp>>);
pub struct ResizeGesturesImp {
    my_bounds: Option<graphene::Rect>,
    document: Arc<Mutex<Document>>,
    selected: Vec<Index>,
    resize_handles: Option<[gtk4::Image; 8]>,
}

impl ResizeGestures {
    pub fn new(document: Arc<Mutex<Document>>) -> Self {
        ResizeGestures(Rc::new(RefCell::new(ResizeGesturesImp {
            my_bounds: None,
            document,
            selected: vec![],
            resize_handles: None,
        })))
    }

    pub fn set_document(&self, document: Arc<Mutex<Document>>) {
        let mut state = self.0.borrow_mut();
        state.my_bounds = None;
        state.selected = vec![];
        state.document = document;
    }

    /// Called by the stage to inform this gesture that the selection has
    /// changed.
    pub fn selection_changed<'a>(
        &self,
        stage: &StageWidget,
        selection: impl Iterator<Item = &'a Index>,
        gizmos: &HashMap<Index, impl IsA<gtk4::Widget>>,
    ) {
        let mut new_bounds = None;
        let mut selected = vec![];

        for index in selection {
            let gizmo = gizmos.get(index);
            if let Some(gizmo) = gizmo {
                let gizmo_bounds = gizmo.compute_bounds(stage);

                match (new_bounds, gizmo_bounds) {
                    (None, gizmo_bounds) => new_bounds = gizmo_bounds,
                    (Some(nb), Some(gizmo_bounds)) => new_bounds = Some(nb.union(&gizmo_bounds)),
                    _ => {}
                };

                selected.push(*index);
            }
        }

        let mut state = self.0.borrow_mut();

        state.my_bounds = new_bounds;
        state.selected = selected;

        drop(state);

        self.place_resize_handles(stage);
    }

    pub fn place_resize_handles(&self, stage: &StageWidget) {
        let mut state = self.0.borrow_mut();
        if state.resize_handles.is_none() {
            let handles = [
                "/live/arcturus/ningyotsukai/stage/gestures/resize/ne-sw-resize.svg", //Northeast
                "/live/arcturus/ningyotsukai/stage/gestures/resize/ns-resize.svg",    //North
                "/live/arcturus/ningyotsukai/stage/gestures/resize/nw-se-resize.svg", //Northwest
                "/live/arcturus/ningyotsukai/stage/gestures/resize/ew-resize.svg",    //West
                "/live/arcturus/ningyotsukai/stage/gestures/resize/ne-sw-resize.svg", //Southwest
                "/live/arcturus/ningyotsukai/stage/gestures/resize/ns-resize.svg",    //South
                "/live/arcturus/ningyotsukai/stage/gestures/resize/nw-se-resize.svg", //Southeast
                "/live/arcturus/ningyotsukai/stage/gestures/resize/ew-resize.svg",    //East
            ];

            let mut handle_widgets = Vec::with_capacity(8);

            for handle in handles {
                let handle_widget = gtk4::Image::builder().resource(handle).build();
                handle_widget.set_parent(stage);

                handle_widgets.push(handle_widget);
            }

            state.resize_handles = Some(handle_widgets[0..8].as_array().unwrap().clone());
        }

        for (index, handle) in state.resize_handles.as_ref().unwrap().iter().enumerate() {
            let (width_minimum, _, _, _) = handle.measure(gtk4::Orientation::Horizontal, 32);
            let (height_minimum, _, _, _) = handle.measure(gtk4::Orientation::Vertical, 32);

            if let Some(bounds) = state.my_bounds {
                let mut x = bounds.x();
                let mut y = bounds.y();

                match index {
                    0..3 => y -= height_minimum as f32,
                    3 | 7 => y += bounds.height() / 2.0 - height_minimum as f32 / 2.0,
                    4..7 => y += bounds.height(),
                    8.. => unreachable!(),
                }

                match index {
                    0 | 6..8 => x -= width_minimum as f32,
                    1 | 5 => x += bounds.width() / 2.0 - width_minimum as f32 / 2.0,
                    2..5 => x += bounds.width(),
                    8.. => unreachable!(),
                }

                handle.allocate(
                    width_minimum,
                    height_minimum,
                    -1,
                    Some(gsk4::Transform::new().translate(&graphene::Point::new(x, y))),
                );
                handle.set_visible(true);
            } else {
                handle.set_visible(false);
            }
        }
    }

    pub fn snapshot(&self, stage: &StageWidget, snapshot: &gtk4::Snapshot) {
        let state = self.0.borrow_mut();

        if let Some(handles) = state.resize_handles.as_ref() {
            for handle in handles {
                stage.snapshot_child(handle, snapshot);
            }
        }
    }
}
