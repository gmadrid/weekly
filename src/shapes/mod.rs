use printpdf::Line;

pub(crate) mod circle;
pub(crate) mod line;
pub(crate) mod rect;

pub trait AsPdfLine {
    fn as_pdf_line(self) -> Line;
}

impl AsPdfLine for Line {
    fn as_pdf_line(self) -> Line {
        self
    }
}
