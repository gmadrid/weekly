use printpdf::Line;

pub(crate) mod line;
pub(crate) mod rect;

pub trait AsPdfLine {
    fn as_pdf_line(&self) -> Line;
}
