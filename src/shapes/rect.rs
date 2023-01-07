use crate::pdfutils::point_pair;
use crate::shapes::{RenderAttrsImpl, ToPlainPdfLine};
use crate::units::Unit;
use printpdf::*;
use std::default::Default;

/// A representation of rectangles and operations on them.
#[derive(Debug, Default, Clone)]
pub struct WRect {
    render_attrs: RenderAttrsImpl,
    corner_radius: Option<Unit>,

    top: Unit,
    left: Unit,
    height: Unit,
    width: Unit,
}

impl WRect {
    pub const fn with_dimensions(width: Unit, height: Unit) -> WRect {
        WRect {
            render_attrs: RenderAttrsImpl::new(),
            corner_radius: None,
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
            ..Self::default()
        }
    }

    pub const fn move_to(&self, left: Unit, top: Unit) -> WRect {
        WRect { top, left, ..*self }
    }

    pub fn move_by(&self, left: Unit, top: Unit) -> WRect {
        WRect {
            top: self.top + top,
            left: self.left + left,
            ..*self
        }
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

    pub fn set_corner_radius(&mut self, radius: Unit) {
        self.corner_radius = Some(radius);
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

    fn as_rounded_rect_shape(&self, radius: Unit) -> Line {
        let pv = 1.0_f64 - 0.55228;
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
            is_closed: true,
            ..Line::default()
        }
    }

    fn as_rect_shape(&self) -> Line {
        Line {
            // In Q1, rects grow downward toward the bottom.
            points: vec![
                point_pair(self.left, self.top, false),
                point_pair(self.left + self.width, self.top, false),
                point_pair(self.left + self.width, self.top - self.height, false),
                point_pair(self.left, self.top - self.height, false),
            ],
            is_closed: true,
            ..Line::default()
        }
    }
}

impl AsRef<RenderAttrsImpl> for WRect {
    fn as_ref(&self) -> &RenderAttrsImpl {
        &self.render_attrs
    }
}

impl AsMut<RenderAttrsImpl> for WRect {
    fn as_mut(&mut self) -> &mut RenderAttrsImpl {
        &mut self.render_attrs
    }
}

impl ToPlainPdfLine for WRect {
    fn to_plain_pdf_line(&self) -> Line {
        if let Some(radius) = self.corner_radius {
            self.as_rounded_rect_shape(radius)
        } else {
            self.as_rect_shape()
        }
    }
}
