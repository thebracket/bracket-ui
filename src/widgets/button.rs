use bracket_lib::prelude::*;
use crate::element::{ElementId, UiElement};
use crate::mouse_coverage::MouseCoverage;

pub enum ButtonDecor {
    Plain,
    Border,
}

pub struct Button {
    text: String,
    color: ColorPair,
    id: ElementId,
    same_line: bool,
    visible: bool,
    decoration: ButtonDecor,
    cached_bounds: Rect,
}

impl UiElement for Button {
    fn id(&self) -> ElementId {
        self.id
    }

    fn find(&mut self, id: ElementId) -> Option<&mut dyn UiElement> {
        if id == self.id {
            return Some(self);
        }
        None
    }

    fn render(&mut self, parent_bounds: Rect, batch: &mut DrawBatch, mouse_coverage: &mut MouseCoverage) {
        if self.visible {
            match self.decoration {
                ButtonDecor::Plain => {
                    batch.print_color(
                        Point::new(parent_bounds.x1, parent_bounds.y1),
                        &self.text,
                        self.color,
                    );
                    self.cached_bounds = Rect::with_size(parent_bounds.x1, parent_bounds.y1, self.text.len() as i32, 1);
                }
                ButtonDecor::Border => {
                    batch.draw_box(Rect::with_size(parent_bounds.x1, parent_bounds.y1, self.text.len() as i32 + 1, 2), self.color);
                    batch.print_color(
                        Point::new(parent_bounds.x1+1, parent_bounds.y1+1),
                        &self.text,
                        self.color,
                    );
                    self.cached_bounds = Rect::with_size(parent_bounds.x1, parent_bounds.y1, self.text.len() as i32 + 2, 3);
                }
            }
            mouse_coverage.push(self.id, self.cached_bounds);
        }
    }

    fn measure_y(&self) -> i32 {
        match self.decoration {
           ButtonDecor::Plain => 1,
           ButtonDecor::Border => 3,
        }
    }

    fn measure_x(&self) -> i32 {
        match self.decoration {
            ButtonDecor::Plain => self.text.len() as i32,
            ButtonDecor::Border => self.text.len() as i32 + 3,
        }
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

impl Button {
    pub fn new<S: ToString>(
        text: S,
        color: ColorPair,
        decoration: ButtonDecor,
        same_line: bool,
    ) -> Box<Self> {
        Box::new(Self {
            text: text.to_string(),
            color,
            id: ElementId::new(),
            same_line,
            visible: true,
            decoration,
            cached_bounds: Rect::zero()
        })
    }
}
