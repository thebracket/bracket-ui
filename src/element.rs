use bracket_lib::prelude::*;
use lazy_static::*;
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElementId(usize);

lazy_static! {
    static ref ID_BAKER: Mutex<ElementId> = Mutex::new(ElementId(0));
}

impl ElementId {
    pub(crate) fn new() -> Self {
        let mut lock = ID_BAKER.lock();
        let inner = lock.as_mut().unwrap();
        let new_id = inner.0;
        inner.0 += 1;
        Self(new_id)
    }
}

pub trait UiElement {
    fn id(&self) -> ElementId;
    fn render(&mut self, parent_bounds: Rect, batch: &mut DrawBatch);
    fn find(&mut self, id: ElementId) -> Option<&mut dyn UiElement>;
    fn insert_child(&mut self, _e: Box<dyn UiElement>) {}
    fn measure_y(&self) -> i32 {
        0
    }
    fn measure_x(&self) -> i32 {
        0
    }
    fn same_line(&self) -> bool {
        false
    }
    fn text(&self) -> String {
        String::new()
    }
    fn set_text(&mut self, _text: String) {}
    fn visible(&self) -> bool {
        true
    }
    fn show(&mut self) {}
    fn hide(&mut self) {}
}
