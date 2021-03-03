use bracket_lib::prelude::*;

use crate::element::{ElementId, UiElement};

pub struct Filler {
    glyph: FontCharType,
    color: ColorPair,
    id: ElementId
}

impl UiElement for Filler {
    fn id(&self) -> ElementId {
        self.id
    }

    fn find(&mut self, id: ElementId) -> Option<&mut dyn UiElement> {
        if id == self.id {
            return Some(self);
        }
        None
    }

    fn render(&mut self, parent_bounds: Rect, batch: &mut DrawBatch) {
        parent_bounds.for_each(|p| {
            batch.set(p, self.color, self.glyph);
        });
    }
}

impl Filler {
    pub fn new(glyph: FontCharType, color: ColorPair) -> Box<Self> {
        Box::new(Self {
            glyph,
            color,
            id : ElementId::new()
        })
    }
}