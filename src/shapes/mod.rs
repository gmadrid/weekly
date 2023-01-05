use printpdf::Line;

pub(crate) mod circle;
pub(crate) mod line;
pub(crate) mod rect;

pub trait ToPdfLine {
    fn to_pdf_line(self) -> Line;
}

impl ToPdfLine for Line {
    fn to_pdf_line(self) -> Line {
        self
    }
}
