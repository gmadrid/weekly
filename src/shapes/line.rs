use crate::pdfutils::point_pair;
use crate::shapes::AsPdfLine;
use crate::units::Unit;
use printpdf::Line;

#[derive(Debug, Clone)]
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
}

impl AsPdfLine for WLine {
    fn as_pdf_line(&self) -> Line {
        Line {
            points: vec![
                point_pair(self.x1, self.y1, false),
                point_pair(self.x2, self.y2, false),
            ],
            has_stroke: true,
            ..Line::default()
        }
    }
}
