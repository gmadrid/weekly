use printpdf::{BuiltinFont, IndirectFontRef, PdfDocumentReference};
use weekly::{
    save_one_page_document, AsPdfLine, Attributes, Colors, GridDescription, Instructions, TGrid,
    Unit, WLine, WRect,
};

const NOTE_HORIZ_PCT: f64 = 70.0;
const NOTE_VERT_PCT: f64 = 82.0;

struct CornellDescription {
    bounds: WRect,
    font: IndirectFontRef,
}

impl CornellDescription {
    pub fn with_bounds(bounds: WRect, font: IndirectFontRef) -> CornellDescription {
        CornellDescription { bounds, font }
    }
}

impl GridDescription for CornellDescription {
    fn bounds(&self) -> WRect {
        self.bounds.clone()
    }

    fn num_cols(&self) -> Option<usize> {
        Some(1)
    }

    fn row_height(&self) -> Option<Unit> {
        Some(weekly::sizes::cornell_rule_height())
    }

    fn horiz_line_style(&self, index: usize, num_rows: usize) -> Option<Attributes> {
        if index == num_rows {
            // Don't render the last line because it coincides with the horizontal rule at the
            // top of the notes area.
            None
        } else {
            Some(
                Attributes::default()
                    .with_stroke_width(0.0)
                    .with_stroke_color(&Colors::gray(0.8)),
            )
        }
    }

    fn vert_line_style(&self, _index: usize, _num_cols: usize) -> Option<Attributes> {
        None
    }

    fn font(&self) -> &IndirectFontRef {
        &self.font
    }
}

fn compute_bottom_line_y(device_rect: &WRect) -> Unit {
    let cornell_height = device_rect.height().pct(NOTE_VERT_PCT);
    let rule_height = weekly::sizes::cornell_rule_height();
    let lines = cornell_height / rule_height;
    device_rect.height()
        - if rule_height * lines != cornell_height {
            rule_height * (lines + 1)
        } else {
            rule_height
        }
}

fn render_cornell(doc: &PdfDocumentReference, device_rect: &WRect) -> weekly::Result<Instructions> {
    let mut instructions = Instructions::default();
    instructions.set_fill_color(Colors::red());
    instructions.set_stroke_width(0.75);
    instructions.set_stroke_color(Colors::gray(0.6));

    let bottom_line_y = compute_bottom_line_y(device_rect);

    let notes_bottom_line = WLine::line(
        device_rect.left(),
        bottom_line_y,
        device_rect.right(),
        bottom_line_y,
    );
    instructions.push_shape(notes_bottom_line.as_pdf_line());

    let left_line_x = device_rect.width().pct(100.0 - NOTE_HORIZ_PCT);

    let notes_left_line = WLine::line(left_line_x, bottom_line_y, left_line_x, device_rect.top());
    instructions.push_shape(notes_left_line.as_pdf_line());

    let grid_rect = WRect::with_dimensions(
        device_rect.right() - left_line_x,
        device_rect.top() - bottom_line_y,
    )
    .move_to(left_line_x, device_rect.top());
    let font = doc.add_builtin_font(BuiltinFont::TimesBold)?;

    TGrid::with_description(CornellDescription::with_bounds(grid_rect, font))
        .append_to_instructions(&mut instructions);

    Ok(instructions)
}

pub fn main() -> weekly::Result<()> {
    let doc_title = "Cornell note page";
    let output_filename = "cornell.pdf";
    let device_rect = weekly::sizes::remarkable2();

    save_one_page_document(doc_title, output_filename, &device_rect, render_cornell)
}
