use printpdf::PdfDocumentReference;
use weekly::{save_one_page_document, AsPdfLine, Colors, Instructions, NumericUnit, WLine, WRect};

// Remarkable claims to want 1404×1872 pixel images. (4/3 aspect ratio)
// These dimensions below are producing a 928x1237 pixel image.
const REMARKABLE_WIDTH_MM: f64 = 157.2;
const REMARKABLE_HEIGHT_MM: f64 = 209.6;

const RULE_HEIGHT_IN: f64 = 9.0 / 32.0;
const NOTE_HORIZ_PCT: f64 = 70.0;
const NOTE_VERT_PCT: f64 = 82.0;

fn remarkable_bounds() -> WRect {
    WRect::with_dimensions(REMARKABLE_WIDTH_MM.mm(), REMARKABLE_HEIGHT_MM.mm())
        .move_to(0.0.mm(), REMARKABLE_HEIGHT_MM.mm())
}

fn render_cornell(_: &PdfDocumentReference, device_rect: &WRect) -> weekly::Result<Instructions> {
    let mut instructions = Instructions::default();
    instructions.set_fill_color(&Colors::red());
    instructions.set_stroke_width(0.75);
    instructions.set_stroke_color(&Colors::gray(0.5));

    let bottom_line_y = device_rect.height().pct(100.0 - NOTE_VERT_PCT);

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

    instructions.set_stroke_width(0.0);
    instructions.set_stroke_color(&Colors::gray(0.8));

    std::iter::successors(Some(bottom_line_y), |prev| {
        Some(*prev + RULE_HEIGHT_IN.inches())
    })
    .take_while(|y| *y < device_rect.top() - RULE_HEIGHT_IN.inches())
    .map(|y| WLine::line(left_line_x, y, device_rect.right(), y))
    .for_each(|l| instructions.push_shape(l.as_pdf_line()));

    Ok(instructions)
}

pub fn main() -> weekly::Result<()> {
    let doc_title = "Cornell note page";
    let output_filename = "cornell.pdf";
    let device_rect = remarkable_bounds();

    save_one_page_document(doc_title, output_filename, &device_rect, render_cornell)
}
