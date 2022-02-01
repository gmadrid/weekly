use printpdf::PdfDocumentReference;
use weekly::{save_one_page_document, ColorProxy, Instructions, NumericUnit, WRect};

const REMARKABLE_WIDTH: f64 = 157.2;
const REMARKABLE_HEIGHT: f64 = 209.6;

fn render_remtest(_: &PdfDocumentReference, page_bounds: &WRect) -> weekly::Result<Instructions> {
    let mut instructions = Instructions::default();
    instructions.set_fill_color(ColorProxy::red());
    instructions.set_stroke_width(1.0);
    instructions.set_stroke_color(ColorProxy::black());

    let black_rect = dbg!(page_bounds.inset_q1(2.0.mm(), 2.0.mm()));
    let rect_shape = black_rect.fill(false).stroke(true);
    instructions.push_rect(rect_shape);

    Ok(instructions)
}

fn main() -> weekly::Result<()> {
    let doc_title = "Remarkable test";
    let output_filename = "remtest.pdf";

    let page_bounds = WRect::with_dimensions(REMARKABLE_WIDTH.mm(), REMARKABLE_HEIGHT.mm())
        .move_to(0.0.mm(), REMARKABLE_HEIGHT.mm());

    save_one_page_document(doc_title, output_filename, &page_bounds, render_remtest)
}
