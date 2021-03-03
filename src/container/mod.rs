pub mod root;
use bracket_lib::prelude::{DrawBatch, Rect};
pub use root::*;
pub mod panel;
pub use panel::*;
pub mod border;
pub use border::*;

use crate::element::UiElement;
use crate::mouse_coverage::MouseCoverage;

fn panel_inner_render(
    mut bounds: Rect,
    batch: &mut DrawBatch,
    children: &mut [Box<dyn UiElement>],
    mouse_coverage: &mut MouseCoverage,
) {
    let mut x = bounds.x1;
    for c in children {
        batch.set_clipping(Some(Rect::with_exact(
            x,
            bounds.y1,
            bounds.x2 + 1,
            bounds.y2 + 1,
        )));
        c.render(
            Rect::with_exact(x, bounds.y1, bounds.x2, bounds.y2),
            batch,
            mouse_coverage,
        );
        if c.same_line() {
            let x_extent = c.measure_x();
            x += x_extent;
            if x >= bounds.x2 {
                let y_extent = c.measure_y();
                bounds.y1 += y_extent;
                x = bounds.x1;
            }
        } else {
            let y_extent = c.measure_y();
            bounds.y1 += y_extent;
            x = bounds.x1;
        }
    }
    batch.set_clipping(None);
}
