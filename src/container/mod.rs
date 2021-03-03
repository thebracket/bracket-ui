pub mod root;
use bracket_lib::prelude::{DrawBatch, Rect};
pub use root::*;
pub mod panel;
pub use panel::*;
pub mod border;
pub use border::*;

use crate::element::UiElement;

fn panel_inner_render(
    mut bounds: Rect,
    batch: &mut DrawBatch,
    children: &mut [Box<dyn UiElement>],
) {
    for c in children {
        batch.set_clipping(Some(Rect::with_exact(
            bounds.x1,
            bounds.y1,
            bounds.x2 + 1,
            bounds.y2 + 1,
        )));
        c.render(bounds, batch);
        bounds.y1 += c.measure_y();
    }
    batch.set_clipping(None);
}
