use std::cell::RefCell;

use crate::element::UiElement;
use crate::prelude::RootElement;
use crate::uislots::UiSlots;
use bracket_lib::prelude::*;

pub struct UserInterface {
    slots: UiSlots,
    layer: usize,
    batch_index: usize,
}

impl UserInterface {
    pub fn new(layer: usize, batch_index: usize) -> Self {
        let mut ui = Self {
            slots: UiSlots::new(),
            layer,
            batch_index,
        };

        ui.slots
            .insert_root("root", RootElement::new())
            .expect("Unable to create root");

        ui
    }

    pub fn render_to_batch(&mut self, ctx: &mut BTerm) {
        // Switch to the UI's console layer
        ctx.set_active_console(self.layer);
        self.reflow(ctx);

        let mut draw_batch = DrawBatch::new();
        if let Some(root_id) = self.slots.index_by_name("root") {
            self.slots.render(root_id, &mut draw_batch);
        }

        draw_batch
            .submit(self.batch_index)
            .expect("Unable to submit batch")
    }

    pub fn reflow(&mut self, ctx: &BTerm) {
        // Obtain the UI overall size
        let (width, height) = ctx.get_char_size();
        let screen_bounds = Rect::with_size(0, 0, width - 1, height - 1);
        if let Some(root_id) = self.slots.index_by_name("root") {
            self.slots.reflow(root_id, screen_bounds);
        }
    }

    pub fn insert<S: ToString>(
        &mut self,
        parent_id: S,
        id: S,
        element: RefCell<Box<dyn UiElement>>,
    ) {
        self.slots.insert(parent_id, id, element);
    }

    pub fn find_id<S: ToString>(&self, id: S) -> Option<usize> {
        self.slots.index_by_name(id)
    }
}
