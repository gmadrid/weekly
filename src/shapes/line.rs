use crate::pdfutils::point_pair;
use crate::Unit;
use printpdf::Line;

#[derive(Debug)]
pub struct WLine {
    x1: Unit,
    y1: Unit,
    x2: Unit,
    y2: Unit,
}

impl WLine {
    pub fn line(x1: Unit, y1: Unit, x2: Unit, y2: Unit) -> WLine {
        WLine { x1, y1, x2, y2 }
    }

    pub fn as_shape(&self, page_height: Unit) -> Line {
        Line {
            points: vec![
                point_pair(self.x1, page_height - self.y1),
                point_pair(self.x2, page_height - self.y2),
            ],
            has_stroke: true,
            ..Line::default()
        }
    }
}
