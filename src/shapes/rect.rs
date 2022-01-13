use crate::pdfutils::point_pair;
use crate::shapes::AsPdfLine;
use crate::units::Unit;
use printpdf::*;

/// A representation of rectangles and operations on them.
#[derive(Debug, Clone)]
pub struct WRect {
    top: Unit,
    left: Unit,
    height: Unit,
    width: Unit,

    has_stroke: bool,
    has_fill: bool,
}

impl WRect {
    pub const fn with_dimensions(width: Unit, height: Unit) -> WRect {
        WRect {
            top: Unit::zero(),
            left: Unit::zero(),
            width,
            height,

            has_stroke: false,
            has_fill: true,
        }
    }

    pub fn at(left: Unit, top: Unit) -> WRect {
        WRect {
            top,
            left,
            width: Unit::zero(),
            height: Unit::zero(),

            has_stroke: false,
            has_fill: true,
        }
    }

    pub const fn move_to(&self, left: Unit, top: Unit) -> WRect {
        WRect { top, left, ..*self }
    }

    pub fn resize(&self, width: Unit, height: Unit) -> WRect {
        WRect {
            width,
            height,
            ..*self
        }
    }

    fn stroke(&self, value: bool) -> Self {
        WRect {
            has_stroke: value,
            ..*self
        }
    }

    fn fill(& self, value: bool) -> Self {
        WRect {
            has_fill: value,
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
            ..*self
        }
    }

    pub fn as_rounded_rect_shape(&self, radius: Unit) -> Line {
        let pv = Unit::from(1.0 - 0.55228);
        Line {
            points: vec![
                point_pair(self.right() - radius, self.top, true),
                point_pair(self.right() - (radius * pv), self.top, true),
                point_pair(self.right(), self.top - (radius * pv), false),
                point_pair(self.right(), self.top - radius, false),
                point_pair(self.right(), self.bottom_q1() + radius, true),
                point_pair(self.right(), self.bottom_q1() + (radius * pv), true),
                point_pair(self.right() - radius * pv, self.bottom_q1(), false),
                point_pair(self.right() - radius, self.bottom_q1(), false),
                point_pair(self.left() + radius, self.bottom_q1(), true),
                point_pair(self.left + radius * pv, self.bottom_q1(), true),
                point_pair(self.left(), self.bottom_q1() + radius * pv, false),
                point_pair(self.left(), self.bottom_q1() + radius, false),
                point_pair(self.left(), self.top() - radius, true),
                point_pair(self.left(), self.top() - radius * pv, true),
                point_pair(self.left() + radius * pv, self.top(), false),
                point_pair(self.left() + radius, self.top(), false),
            ],
            has_fill: true,
            is_closed: true,
            ..Line::default()
        }
    }
}

impl AsPdfLine for WRect {
    fn as_pdf_line(&self) -> Line {
        Line {
            // In Q1, rects grow downward toward the bottom.
            points: vec![
                point_pair(self.left, self.top, false),
                point_pair(self.left + self.width, self.top, false),
                point_pair(self.left + self.width, self.top - self.height, false),
                point_pair(self.left, self.top - self.height, false),
            ],
            has_fill: true,
            is_closed: true,
            ..Line::default()
        }
    }
}
