use crate::element::{ElementId, UiElement};
use crate::mouse_coverage::MouseCoverage;
use bracket_lib::prelude::*;

pub struct EmptyRoot {
    children: Vec<Box<dyn UiElement>>,
    id: ElementId,
}

impl UiElement for EmptyRoot {
    fn id(&self) -> ElementId {
        self.id
    }

    fn render(&mut self, parent_bounds: Rect, batch: &mut DrawBatch, mouse_coverage: &mut MouseCoverage) {
        // This doesn't draw anything by itself, but passes to children
        super::panel_inner_render(parent_bounds, batch, &mut self.children, mouse_coverage);
    }

    fn find(&mut self, id: ElementId) -> Option<&mut dyn UiElement> {
        if id == self.id {
            return Some(self);
        }

        for c in &mut self.children {
            let found = c.find(id);
            if let Some(found) = found {
                return Some(found);
            }
        }

        None
    }

    fn insert_child(&mut self, e: Box<dyn UiElement>) {
        self.children.push(e);
    }
}

impl EmptyRoot {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            children: Vec::new(),
            id: ElementId::new(),
        })
    }
}
