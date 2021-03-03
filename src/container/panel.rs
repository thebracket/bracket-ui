use crate::element::{ElementId, UiElement};
use bracket_lib::prelude::*;

#[derive(Debug)]
pub enum PanelLayout {
    Absolute(Rect),
    Fill,
    Left{percent: Option<i32>, min: Option<i32>, max: Option<i32>},
    Top{percent: Option<i32>, min: Option<i32>, max: Option<i32>},
}

pub struct Panel {
    children: Vec<Box<dyn UiElement>>,
    id: ElementId,
    layout: PanelLayout,
    border: bool,
}

impl UiElement for Panel {
    fn id(&self) -> ElementId {
        self.id
    }

    fn render(&self, parent_bounds: Rect, batch: &mut DrawBatch) {
        let mut bounds = match self.layout {
            PanelLayout::Absolute(r) => Rect::with_exact(r.x1, r.y1, r.x2, r.y2 + 1),
            PanelLayout::Fill => parent_bounds,
            PanelLayout::Left{percent, min, max} => {
                let mut w = 0;
                if let Some(pct) = percent {
                    w = (parent_bounds.width() as f32 * (pct as f32 / 100.0)) as i32;
                }
                if let Some(min) = min {
                    w = i32::max(min, w);
                }
                if let Some(max) = max {
                    w = i32::min(max, w);
                }
                let h = parent_bounds.height();

                Rect::with_size(
                    parent_bounds.x1,
                    parent_bounds.y1,
                    w,
                    h
                )
            }
            PanelLayout::Top { percent, min, max } => {
                let mut h = 0;
                if let Some(pct) = percent {
                    h = (parent_bounds.height() as f32 * (pct as f32 / 100.0)) as i32;
                }
                if let Some(min) = min {
                    h = i32::max(min, h);
                }
                if let Some(max) = max {
                    h = i32::min(max, h);
                }
                let w = parent_bounds.width();

                Rect::with_size(
                    parent_bounds.x1,
                    parent_bounds.y1,
                    w,
                    h
                )
            }
        };
        if self.border {
            batch.draw_box(bounds, ColorPair::new(GREY, BLACK));
            bounds.x1 +=1;
            bounds.y1 += 1;
            bounds.x2 -=1;
            bounds.y2 -=1;
        }
        //println!("{:?} : {:?}", self.layout, bounds);
        super::panel_inner_render(bounds, batch, &self.children);
    }

    fn find(&mut self, id: ElementId) -> Option<&mut dyn UiElement> {
        if id == self.id {
            return Some(self);
        }

        for c in &mut self.children {
            let found = c.find(id);
            if let Some(found) = found {
                return Some(found);
            }
        }

        None
    }

    fn insert_child(&mut self, e: Box<dyn UiElement>) {
        self.children.push(e);
    }
}

impl Panel {
    pub fn new(layout: PanelLayout, border: bool) -> Box<Self> {
        Box::new(Self{
            children : Vec::new(),
            id: ElementId::new(),
            layout,
            border,
        })
    }
}