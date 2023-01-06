use crate::LineModifiers;
use printpdf::Line;

pub(crate) mod circle;
pub(crate) mod line;
pub(crate) mod rect;

/// A trait indicating whether a shape should be rendered as a stroke or a filled shape (or both).
pub trait HasRenderAttrs
where
    Self: Sized,
{
    fn stroke(mut self) -> Self {
        self.set_stroke(true);
        self
    }
    fn fill(mut self) -> Self {
        self.set_fill(true);
        self
    }

    fn set_stroke(&mut self, stroke: bool);
    fn is_stroked(&self) -> bool;
    fn set_fill(&mut self, fill: bool);
    fn is_filled(&self) -> bool;
}

/// A basic implementation of HasRenderAttrs.
#[derive(Debug, Default, Copy, Clone)]
struct RenderAttrsImpl {
    stroke: bool,
    fill: bool,
}

impl RenderAttrsImpl {
    const fn new() -> RenderAttrsImpl {
        RenderAttrsImpl {
            stroke: false,
            fill: false,
        }
    }
}

impl HasRenderAttrs for RenderAttrsImpl {
    fn set_stroke(&mut self, stroke: bool) {
        self.stroke = stroke;
    }

    fn is_stroked(&self) -> bool {
        self.stroke
    }

    fn set_fill(&mut self, fill: bool) {
        self.fill = fill
    }

    fn is_filled(&self) -> bool {
        self.fill
    }
}

/// A type composing RendAttrsImpl and allowing AsRef and AsMut conversions will get the
/// implementation for free.
impl<T: AsMut<RenderAttrsImpl> + AsRef<RenderAttrsImpl>> HasRenderAttrs for T {
    fn set_stroke(&mut self, stroke: bool) {
        self.as_mut().set_stroke(stroke)
    }

    fn is_stroked(&self) -> bool {
        self.as_ref().is_stroked()
    }

    fn set_fill(&mut self, fill: bool) {
        self.as_mut().set_fill(fill)
    }

    fn is_filled(&self) -> bool {
        self.as_ref().is_filled()
    }
}

trait ToPlainPdfLine {
    /// Converts a shape into an (unstroke, unfilled) printpdf::line.
    /// The implementor is responsible for setting the 'closed' flag.
    fn to_plain_pdf_line(&self) -> Line;
}

pub trait ToPdfLine {
    /// Converts a shape into a printpdf::Line that is marked for rendering as a stroke
    /// or a filled shape (or both).
    fn to_pdf_line(&self) -> Line;
}

impl<T> ToPdfLine for T
where
    T: ToPlainPdfLine + HasRenderAttrs,
{
    fn to_pdf_line(&self) -> Line {
        // TODO: Do you want to check that plain pdf line is unstroked and unfilled?
        let stroked = self.is_stroked();
        let filled = self.is_filled();
        let mut line = self.to_plain_pdf_line();
        // TODO: This is a little awkward.
        if stroked {
            line = line.stroke(true);
        }
        if filled {
            line = line.fill(true);
        }
        line
    }
}
