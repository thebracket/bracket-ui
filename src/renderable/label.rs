use std::cell::RefCell;

use crate::element::UiElement;
use bracket_lib::prelude::*;

pub struct Label {
    bounds: Rect,
    label: String,
    colors: ColorPair,
}

impl UiElement for Label {
    fn render(&self, batch: &mut DrawBatch) {
        batch.print_color(
            Point::new(self.bounds.x1, self.bounds.y1),
            &self.label,
            self.colors,
        );
    }

    fn reflow(&mut self, parent_bounds: Rect) -> Option<Rect> {
        self.bounds = parent_bounds;
        None
    }

    fn get_children(&self) -> Option<Vec<usize>> {
        None
    }

    fn measure(&self) -> (usize, usize) {
        (self.label.len(), 1)
    }
}

impl Label {
    pub fn new<S: ToString>(label: S, colors: ColorPair) -> RefCell<Box<Self>> {
        let label_str = label.to_string();
        RefCell::new(Box::new(Self {
            bounds: Rect::with_size(0, 0, label_str.len(), 1),
            label: label_str,
            colors,
        }))
    }
}
