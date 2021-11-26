use std::fs::File;
use std::io::BufWriter;
use printpdf::PdfDocument;
use weekly::{Colors, Instructions, LineModifiers, NumericUnit, Unit, WRect};

const REMARKABLE_WIDTH: f64 = 157.2;
const REMARKABLE_HEIGHT: f64 =209.6;

fn main() {
    let doc_title = "Remarkable test";
    let output_filename = "remtest.pdf";

    let page_bounds =
    WRect::with_dimensions(REMARKABLE_WIDTH.mm(), REMARKABLE_HEIGHT.mm())
        .move_to(0.0.mm(), REMARKABLE_HEIGHT.mm());

    let mut instructions = Instructions::default();
    instructions.set_fill_color(&Colors::red());
    instructions.set_stroke_width(1.0);
    instructions.set_stroke_color(&Colors::black());

    let black_rect = dbg!(page_bounds.inset_q1(2.0.mm(), 2.0.mm()));
    let rect_shape = black_rect.as_shape().fill(false).stroke(true);
    instructions.push_shape(rect_shape);

    let (doc, page, layer) = PdfDocument::new(
        doc_title,
        page_bounds.width().into(),
        page_bounds.height().into(),
        "Layer 1",
    );

    let layer = doc.get_page(page).get_layer(layer);
    instructions.draw_to_layer(&layer);

    doc.save(&mut BufWriter::new(File::create(output_filename).unwrap()))
        .unwrap();
}