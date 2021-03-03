use crate::{container::EmptyRoot, element::{ElementId, UiElement}};
use bracket_lib::prelude::*;
use std::collections::HashMap;

pub struct UserInterface {
    root_element: Box<dyn UiElement>,
    layer: usize,
    batch_index: usize,
    name_to_id: HashMap<String, ElementId>,
}

impl UserInterface {
    pub fn new(layer: usize, batch_index: usize) -> Self {
        let root_element = EmptyRoot::new();
        let mut name_to_id = HashMap::new();
        name_to_id.insert("default_root".to_string(), root_element.id());
        let ui = Self {
            root_element,
            layer,
            batch_index,
            name_to_id: name_to_id,
        };

        ui
    }

    pub fn render_to_batch(&mut self, ctx: &mut BTerm) -> BError {
        ctx.set_active_console(self.layer);
        let (w, h) = ctx.get_char_size();
        let bounds = Rect::with_size(0, 0, w - 1, h - 1);
        let mut batch = DrawBatch::new();
        self.root_element.render(bounds, &mut batch);
        batch.submit(self.batch_index)?;
        Ok(())
    }

    pub fn root(&self) -> ElementId {
        self.root_element.id()
    }

    pub fn insert<S: ToString>(
        &mut self,
        name: S,
        parent_id: ElementId,
        element: Box<dyn UiElement>,
    ) -> ElementId {
        let id = element.id();
        self.name_to_id.insert(name.to_string(), id);
        let parent = self.root_element.find(parent_id);
        if let Some(parent) = parent {
            parent.insert_child(element);
        }
        id
    }

    pub fn by_name<S: ToString>(&self, name: S) -> Option<ElementId> {
        if let Some(opt) = self.name_to_id.get(&name.to_string()) {
            Some(*opt)
        } else {
            None
        }
    }
}
