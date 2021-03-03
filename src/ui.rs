use bracket_lib::prelude::*;
use crate::{container::EmptyRoot, element::{ElementId, UiElement}};

pub struct UserInterface {
    root_element: Box<dyn UiElement>,
    layer: usize,
    batch_index: usize,
}

impl UserInterface {
    pub fn new(layer: usize, batch_index: usize) -> Self {
        let ui = Self {
            root_element: EmptyRoot::new(),
            layer,
            batch_index,
        };

        ui
    }

    pub fn render_to_batch(&self, ctx: &mut BTerm) -> BError {
        ctx.set_active_console(self.layer);
        let (w,h) = ctx.get_char_size();
        let bounds = Rect::with_size(0, 0, w-1, h-1);
        let mut batch = DrawBatch::new();
        self.root_element.render(bounds, &mut batch);
        batch.submit(self.batch_index)?;
        Ok(())
    }

    pub fn root(&self) -> ElementId {
        self.root_element.id()
    }

    pub fn insert(&mut self, parent_id: ElementId, element: Box<dyn UiElement>) -> ElementId {
        let id = element.id();
        let parent = self.root_element.find(parent_id);
        if let Some(parent) = parent {
            parent.insert_child(element);
        }
        id
    }
}
