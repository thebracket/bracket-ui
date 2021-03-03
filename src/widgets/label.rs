use bracket_lib::prelude::*;

use crate::element::{ElementId, UiElement};

pub struct Label {
    text: String,
    color: ColorPair,
    id: ElementId
}

impl UiElement for Label {
    fn id(&self) -> ElementId {
        self.id
    }

    fn find(&mut self, id: ElementId) -> Option<&mut dyn UiElement> {
        if id == self.id {
            return Some(self);
        }
        None
    }

    fn render(&self, parent_bounds: Rect, batch: &mut DrawBatch) {
        batch.print_color(Point::new(parent_bounds.x1, parent_bounds.y1), &self.text, self.color);
    }

    fn measure_y(&self) -> i32 {
        1
    }

    fn measure_x(&self) -> i32 {
        self.text.len() as i32
    }
}

impl Label {
    pub fn new<S: ToString>(text: S, color: ColorPair) -> Box<Self> {
        Box::new(Self {
            text: text.to_string(),
            color,
            id : ElementId::new()
        })
    }
}