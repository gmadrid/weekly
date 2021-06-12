use crate::point_pair;
use printpdf::Line;

pub struct PRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl PRect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> PRect {
        PRect {
            x,
            y,
            width,
            height,
        }
    }

    /// Creates a new PRect, identical to self, but with a new width.
    pub fn with_width(&self, width: f64) -> PRect {
        PRect { width, ..*self }
    }

    /// Creates a new PRect, identical to self, but with a new height.
    pub fn with_height(&self, height: f64) -> PRect {
        PRect { height, ..*self }
    }

    /// Moves the rectangle keeping the dimensions the same.
    pub fn rmove(&self, x_delta: f64, y_delta: f64) -> PRect {
        PRect {
            x: self.x + x_delta,
            y: self.y + y_delta,
            ..*self
        }
    }

    /// Insets the rectangle. (Negative values will "outset" it.)
    pub fn inset(&self, x_inset: f64, y_inset: f64) -> Self {
        PRect {
            x: self.x + x_inset,
            y: self.y + y_inset,
            width: self.width - 2.0 * x_inset,
            height: self.height - 2.0 * y_inset,
        }
    }

    /// Outputs a Line suitable for adding to the pdfprint Page.
    pub fn shape(&self) -> Line {
        let points = vec![
            point_pair(self.x, self.y),
            point_pair(self.x, self.y + self.height),
            point_pair(self.x + self.width, self.y + self.height),
            point_pair(self.x + self.width, self.y),
        ];
        Line {
            points,
            is_closed: true,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        }
    }
}
