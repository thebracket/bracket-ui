use std::cell::RefCell;

use crate::element::UiElement;
use bracket_lib::prelude::*;

pub struct RootElement {
    bounds: Rect,
    children: Vec<usize>,
}

impl UiElement for RootElement {
    fn render(&self, _batch: &mut DrawBatch) {
        // The default root element is invisible
    }

    fn reflow(&mut self, parent_bounds: Rect) -> Option<Rect> {
        self.bounds = parent_bounds;
        None
    }

    fn get_children(&self) -> Option<Vec<usize>> {
        Some(self.children.clone())
    }

    fn add_child(&mut self, child_id: usize) {
        self.children.push(child_id);
    }
}

impl RootElement {
    pub fn new() -> RefCell<Box<Self>> {
        RefCell::new(Box::new(Self {
            bounds: Rect::with_size(0, 0, 1, 1),
            children: Vec::new(),
        }))
    }
}
