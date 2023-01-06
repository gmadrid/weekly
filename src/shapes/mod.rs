use crate::LineModifiers;
use printpdf::Line;

pub(crate) mod circle;
pub(crate) mod line;
pub(crate) mod rect;

pub trait ToPdfLine {
    // Returns an unstroked, unfilled Line. May be 'closed' depending on context.
    fn to_pdf_line_basic(self) -> Line;

    fn to_stroked_line(self) -> Line
    where
        Self: Sized,
    {
        self.to_pdf_line_basic().stroke(true)
    }

    fn to_filled_line(self) -> Line
    where
        Self: Sized,
    {
        self.to_pdf_line_basic().fill(true)
    }
}

impl ToPdfLine for Line {
    fn to_pdf_line_basic(self) -> Line {
        self
    }
}
