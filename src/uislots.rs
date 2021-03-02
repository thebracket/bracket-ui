use crate::element::UiElement;
use bracket_lib::prelude::{BResult, DrawBatch, Rect};
use std::{cell::RefCell, collections::HashMap};

pub(crate) struct UiSlots {
    id_map: HashMap<String, usize>,
    element_arena: Vec<Option<RefCell<Box<dyn UiElement>>>>,
    arena_open: Vec<usize>,
}

impl UiSlots {
    pub(crate) fn new() -> Self {
        let mut element_arena = Vec::new();
        let mut arena_open = Vec::new();
        for i in 0..32 {
            element_arena.push(None);
            arena_open.push(i);
        }

        Self {
            id_map: HashMap::new(),
            element_arena,
            arena_open,
        }
    }

    pub(crate) fn index_by_name<S: ToString>(&self, name: S) -> Option<usize> {
        if let Some(id) = self.id_map.get(&name.to_string()) {
            Some(*id)
        } else {
            None
        }
    }

    pub(crate) fn insert_root<S: ToString>(
        &mut self,
        id: S,
        element: RefCell<Box<dyn UiElement>>,
    ) -> BResult<()> {
        // Ensure that there is an available slot for the element
        if self.arena_open.is_empty() {
            let len = self.element_arena.len();
            for i in 0..32 {
                self.element_arena.push(None);
                self.arena_open.push(i + len);
            }
        }

        // Insert it
        let new_id = self.arena_open[0];
        self.arena_open.remove(0);
        self.element_arena[new_id] = Some(element);
        self.id_map.insert(id.to_string(), new_id);

        Ok(())
    }

    pub(crate) fn insert<S: ToString>(
        &mut self,
        parent_id: S,
        id: S,
        element: RefCell<Box<dyn UiElement>>,
    ) {
        // Ensure that there is an available slot for the element
        if self.arena_open.is_empty() {
            let len = self.element_arena.len();
            for i in 0..32 {
                self.element_arena.push(None);
                self.arena_open.push(i + len);
            }
        }

        // Insert it
        let new_id = self.arena_open[0];
        self.arena_open.remove(0);
        self.element_arena[new_id] = Some(element);
        self.id_map.insert(id.to_string(), new_id);

        // Map the parent: TODO - Error checking
        let parent_id_n = self.index_by_name(parent_id).unwrap();
        self.element_arena[parent_id_n]
            .as_mut()
            .unwrap()
            .borrow_mut()
            .add_child(new_id);
    }

    pub(crate) fn reflow(&mut self, id: usize, bounds: Rect) {
        // Start by measuring all the static children
        let mut height = 0;
        let mut width = 0;
        let mut cb = bounds;
        if let Some(e) = self.element_arena[id].as_ref() {
            if let Some(children) = e.borrow().get_children().as_ref() {
                children.iter().for_each(|c| {
                    if let Some(child) = self.element_arena[*c].as_ref() {
                        let (w, h) = child.borrow().measure();
                        height += h;
                        width = usize::max(width, w);
                        cb.y1 += h as i32;
                        child.borrow_mut().reflow(cb);
                    }
                })
            }
        }
    }

    pub(crate) fn render(&self, id: usize, batch: &mut DrawBatch) {
        if let Some(e) = self.element_arena[id].as_ref() {
            e.borrow().render(batch);
            if let Some(children) = e.borrow().get_children().as_ref() {
                children.iter().for_each(|c| {
                    self.render(*c, batch);
                });
            }
        }
    }
}
