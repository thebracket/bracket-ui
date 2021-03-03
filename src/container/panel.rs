use crate::element::{ElementId, UiElement};
use bracket_lib::prelude::*;

#[derive(Debug)]
pub enum PanelLayout {
    Absolute(Rect),
    Fill,
    Left {
        percent: Option<i32>,
        min: Option<i32>,
        max: Option<i32>,
    },
    Top {
        percent: Option<i32>,
        min: Option<i32>,
        max: Option<i32>,
    },
}

pub struct Panel {
    children: Vec<Box<dyn UiElement>>,
    id: ElementId,
    layout: PanelLayout,
    cached_bounds: Rect,
}

impl UiElement for Panel {
    fn id(&self) -> ElementId {
        self.id
    }

    fn render(&mut self, parent_bounds: Rect, batch: &mut DrawBatch) {
        let bounds = match self.layout {
            PanelLayout::Absolute(r) => Rect::with_exact(r.x1, r.y1, r.x2, r.y2 + 1),
            PanelLayout::Fill => parent_bounds,
            PanelLayout::Left { percent, min, max } => {
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

                Rect::with_size(parent_bounds.x1, parent_bounds.y1, w, h)
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

                Rect::with_size(parent_bounds.x1, parent_bounds.y1, w, h)
            }
        };
        self.cached_bounds = bounds;
        super::panel_inner_render(bounds, batch, &mut self.children);
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

    fn measure_y(&self) -> i32 {
        self.cached_bounds.height() + 1
    }

    fn measure_x(&self) -> i32 {
        self.cached_bounds.width()
    }

    fn same_line(&self) -> bool {
        true
    }
}

impl Panel {
    pub fn new(layout: PanelLayout) -> Box<Self> {
        Box::new(Self {
            children: Vec::new(),
            id: ElementId::new(),
            layout,
            cached_bounds: Rect::zero(),
        })
    }

    pub fn split_vertical(&mut self, percent: i32) -> (Box<Self>, Box<Self>) {
        let top = Panel::new(PanelLayout::Top {
            percent: Some(percent),
            min: None,
            max: None,
        });
        let bottom = Panel::new(PanelLayout::Top {
            percent: Some(100),
            min: None,
            max: None,
        });

        (top, bottom)
    }

    pub fn split_horizontal(&mut self, percent: i32) -> (Box<Self>, Box<Self>) {
        let top = Panel::new(PanelLayout::Left {
            percent: Some(percent),
            min: None,
            max: None,
        });
        let bottom = Panel::new(PanelLayout::Left {
            percent: Some(100),
            min: None,
            max: None,
        });

        (top, bottom)
    }
}
