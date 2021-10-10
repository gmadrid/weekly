use crate::pdfutils::point_pair;
use crate::units::Unit;
use printpdf::*;

#[derive(Debug, Clone)]
pub struct WRect {
    x1: Unit,
    y1: Unit,
    x2: Unit,
    y2: Unit,
}

impl WRect {
    pub fn with_dimensions(width: Unit, height: Unit) -> WRect {
        WRect {
            x1: Unit::zero(),
            y1: Unit::zero(),
            x2: width,
            y2: height,
        }
    }

    // TODO: change these names to something better.
    pub fn new(x1: Unit, y1: Unit, x2: Unit, y2: Unit) -> WRect {
        WRect { x1, y1, x2, y2 }
    }

    pub fn height(&self) -> Unit {
        (self.y2 - self.y1).abs()
    }

    pub fn width(&self) -> Unit {
        (self.x2 - self.x1).abs()
    }

    pub fn left(&self) -> Unit {
        self.x1.min(self.x2)
    }

    pub fn right(&self) -> Unit {
        self.x1.max(self.x2)
    }

    pub fn top(&self) -> Unit {
        self.y1.min(self.y2)
    }

    pub fn bottom(&self) -> Unit {
        self.y1.max(self.y2)
    }

    pub fn inset(&self, xdelta: Unit, ydelta: Unit) -> WRect {
        WRect {
            x1: self.x1 + xdelta,
            y1: self.y1 + ydelta,
            x2: self.x2 - xdelta,
            y2: self.y2 - ydelta,
        }
    }

    pub fn inset_all(
        &self,
        left_inset: Unit,
        top_inset: Unit,
        right_inset: Unit,
        bottom_inset: Unit,
    ) -> WRect {
        WRect {
            x1: self.x1 + left_inset,
            y1: self.y1 + top_inset,
            x2: self.x2 - right_inset,
            y2: self.y2 - bottom_inset,
        }
    }

    pub fn as_shape(&self, page_height: Unit) -> Line {
        Line {
            points: vec![
                point_pair(self.x1, page_height - self.y1),
                point_pair(self.x2, page_height - self.y1),
                point_pair(self.x2, page_height - self.y2),
                point_pair(self.x1, page_height - self.y2),
            ],
            has_fill: true,
            is_closed: true,
            ..Line::default()
        }
    }
}
