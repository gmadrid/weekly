use crate::pdfutils::point_pair;
use crate::units::Unit;
use printpdf::*;

/// A representation of rectangles and operations on them.
#[derive(Debug, Clone)]
pub struct WRect {
    top: Unit,
    left: Unit,
    height: Unit,
    width: Unit,
}

impl WRect {
    pub fn with_dimensions(width: Unit, height: Unit) -> WRect {
        WRect {
            top: Unit::zero(),
            left: Unit::zero(),
            width,
            height,
        }
    }

    pub fn at(left: Unit, top: Unit) -> WRect {
        WRect {
            top,
            left,
            width: Unit::zero(),
            height: Unit::zero(),
        }
    }

    pub fn move_to(&self, left: Unit, top: Unit) -> WRect {
        WRect { top, left, ..*self }
    }

    pub fn resize(&self, width: Unit, height: Unit) -> WRect {
        WRect {
            width,
            height,
            ..*self
        }
    }

    pub fn height(&self) -> Unit {
        self.height
    }

    pub fn width(&self) -> Unit {
        self.width
    }

    pub fn left(&self) -> Unit {
        self.left
    }

    pub fn right(&self) -> Unit {
        self.left + self.width
    }

    pub fn top(&self) -> Unit {
        self.top
    }

    pub fn bottom_q1(&self) -> Unit {
        // In Q1, the top is the greater y coord.
        self.top - self.height
    }

    pub fn inset_q1(&self, xdelta: Unit, ydelta: Unit) -> WRect {
        self.inset_all_q1(xdelta, ydelta, xdelta, ydelta)
    }

    pub fn inset_all_q1(
        &self,
        left_inset: Unit,
        top_inset: Unit,
        right_inset: Unit,
        bottom_inset: Unit,
    ) -> WRect {
        // In Q1, the top goes down when inset
        WRect {
            left: self.left + left_inset,
            top: self.top - top_inset,
            width: self.width - left_inset - right_inset,
            height: self.height - top_inset - bottom_inset,
        }
    }

    pub fn as_shape(&self) -> Line {
        Line {
            // In Q1, rects grow downward toward the bottom.
            points: vec![
                point_pair(self.left, self.top),
                point_pair(self.left + self.width, self.top),
                point_pair(self.left + self.width, self.top - self.height),
                point_pair(self.left, self.top - self.height),
            ],
            has_fill: true,
            is_closed: true,
            ..Line::default()
        }
    }
}
