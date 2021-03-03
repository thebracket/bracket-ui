use bracket_lib::prelude::{BTerm, Rect};
use crate::{element::ElementId, prelude::UiMessage};
use std::collections::VecDeque;

pub struct MouseCoverage {
    tiles : Vec<Option<ElementId>>,
    width : i32
}

impl MouseCoverage {
    pub fn new(w: u32, h: u32) -> Self {
        let sz = (w * h) as usize;
        let tiles = vec![None; sz];
        Self{
            tiles,
            width: w as i32
        }
    }

    pub fn push(&mut self, id: ElementId, area: Rect) {
        area.for_each(|p| {
            let idx = (p.y * self.width) + p.x;
            self.tiles[idx as usize] = Some(id);
        });
    }

    pub(crate) fn message_pump(&self, ctx: &BTerm, mailbox: &mut VecDeque<UiMessage>) {
        let mouse_pos = ctx.mouse_pos();
        let idx = ((mouse_pos.1 * self.width) + mouse_pos.0) as usize;
        if let Some(id) = self.tiles[idx] {
            if ctx.left_click {
                mailbox.push_front(UiMessage::MouseClick(id));
            }
        }
    }
}
