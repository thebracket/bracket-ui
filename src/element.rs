use bracket_lib::prelude::{DrawBatch, Rect};

pub trait UiElement {
    fn render(&self, batch: &mut DrawBatch);

    fn reflow(&mut self, _parent_bounds: Rect) -> Option<Rect> {
        None
    }

    fn get_children(&self) -> Option<Vec<usize>> {
        None
    }

    fn add_child(&mut self, _child_id: usize) {}

    fn measure(&self) -> (usize, usize) {
        (0, 0)
    }
}
