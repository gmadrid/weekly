use printpdf::Line;
use crate::point_pair;

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct WLine {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

impl WLine {
    pub fn horiz(x: f64, y: f64: width: f64) -> WLine {
        WLine {
            x1: x,
            y1: y,
            x2: x + width,
            y2: y,
        }
    }

    pub fn vert(x: f64, y: f64, height: f64) -> WLine {
        WLine {
            x1: x,
            y1: y,
            x2: x,
            y2: y + height,
        }
    }
}

impl From<WLine> for Line {
    fn from(_: WLine) -> Self {
        Line {
            points: vec![
                point_pair(self.x, self.y),
                point_pair(self.x + self.x_offset, self.y + self.y_offset),
            ],
            .. Default::default()
        }
    }
}