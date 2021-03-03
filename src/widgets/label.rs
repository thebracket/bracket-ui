use bracket_lib::prelude::*;
use crate::element::{ElementId, UiElement};
use crate::mouse_coverage::MouseCoverage;

pub struct Label {
    text: String,
    color: ColorPair,
    id: ElementId,
    same_line: bool,
    visible: bool,
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

    fn render(&mut self, parent_bounds: Rect, batch: &mut DrawBatch, _mouse_coverage: &mut MouseCoverage) {
        if self.visible {
            batch.print_color(
                Point::new(parent_bounds.x1, parent_bounds.y1),
                &self.text,
                self.color,
            );
        }
    }

    fn measure_y(&self) -> i32 {
        1
    }

    fn measure_x(&self) -> i32 {
        self.text.len() as i32
    }

    fn same_line(&self) -> bool {
        self.same_line
    }

    fn text(&self) -> String {
        self.text.clone()
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn show(&mut self) {
        self.visible = true;
    }

    fn hide(&mut self) {
        self.visible = false;
    }
}

impl Label {
    pub fn new<S: ToString>(text: S, color: ColorPair, same_line: bool) -> Box<Self> {
        Box::new(Self {
            text: text.to_string(),
            color,
            id: ElementId::new(),
            same_line,
            visible: true,
        })
    }
}
