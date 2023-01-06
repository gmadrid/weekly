use crate::pdfutils::point_pair;
use crate::shapes::{RenderAttrsImpl, ToPlainPdfLine};
use crate::units::Unit;
use printpdf::Line;

#[derive(Debug)]
pub struct WLine {
    render_attrs: RenderAttrsImpl,

    x1: Unit,
    y1: Unit,
    x2: Unit,
    y2: Unit,
}

impl WLine {
    pub fn line(x1: Unit, y1: Unit, x2: Unit, y2: Unit) -> WLine {
        WLine {
            render_attrs: RenderAttrsImpl::default(),
            x1,
            y1,
            x2,
            y2,
        }
    }
}

impl AsRef<RenderAttrsImpl> for WLine {
    fn as_ref(&self) -> &RenderAttrsImpl {
        &self.render_attrs
    }
}

impl AsMut<RenderAttrsImpl> for WLine {
    fn as_mut(&mut self) -> &mut RenderAttrsImpl {
        &mut self.render_attrs
    }
}

impl ToPlainPdfLine for WLine {
    fn to_plain_pdf_line(self) -> Line {
        Line {
            points: vec![
                point_pair(self.x1, self.y1, false),
                point_pair(self.x2, self.y2, false),
            ],
            ..Line::default()
        }
    }
}
