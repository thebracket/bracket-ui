use bracket_lib::prelude::*;

use crate::element::{ElementId, UiElement};
use crate::mouse_coverage::MouseCoverage;

pub struct BorderTitle {
    pub title: String,
    pub color: ColorPair,
}

pub struct Border {
    double_line: bool,
    color: ColorPair,
    id: ElementId,
    children: Vec<Box<dyn UiElement>>,
    title: Option<BorderTitle>,
}

impl UiElement for Border {
    fn id(&self) -> ElementId {
        self.id
    }

    fn render(
        &mut self,
        parent_bounds: Rect,
        batch: &mut DrawBatch,
        mouse_coverage: &mut MouseCoverage,
    ) {
        if self.double_line {
            batch.draw_double_box(parent_bounds, self.color);
            if let Some(bt) = &self.title {
                batch.print_color(
                    Point::new(parent_bounds.x1 + 1, parent_bounds.y1),
                    &format!("╣ {} ╠", bt.title),
                    bt.color,
                );
            }
        } else {
            batch.draw_box(parent_bounds, self.color);
            if let Some(bt) = &self.title {
                batch.print_color(
                    Point::new(parent_bounds.x1 + 1, parent_bounds.y1),
                    &format!("┤ {} ├", bt.title),
                    bt.color,
                );
            }
        }
        let bounds = Rect::with_exact(
            parent_bounds.x1 + 1,
            parent_bounds.y1 + 1,
            parent_bounds.x2 - 1,
            parent_bounds.y2 - 1,
        );
        super::panel_inner_render(bounds, batch, &mut self.children, mouse_coverage);
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

    fn text(&self) -> String {
        if let Some(t) = &self.title {
            t.title.clone()
        } else {
            String::new()
        }
    }

    fn set_text(&mut self, text: String) {
        if let Some(t) = self.title.as_mut() {
            t.title = text;
        }
    }
}

impl Border {
    pub fn new(double_line: bool, color: ColorPair, title: Option<BorderTitle>) -> Box<Self> {
        Box::new(Self {
            double_line,
            color,
            id: ElementId::new(),
            children: Vec::new(),
            title,
        })
    }
}
