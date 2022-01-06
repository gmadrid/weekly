use printpdf::PdfDocument;
use std::fs::File;
use std::io::BufWriter;
use weekly::{GridDescription, NumericUnit, TGrid, WRect};

struct Empty;

impl GridDescription for Empty {}

fn main() {
    let doc_title = "Foo";
    let output_filename = "foo.pdf";

    let page_rect = WRect::with_dimensions(8.5.inches(), 11.0.inches())
        .move_to(0.0.inches(), 11.0.inches());

    let (doc, page, layer) = PdfDocument::new(
        doc_title,
        page_rect.width().into(),
        page_rect.height().into(),
        "Layer 1",
    );

    let description = Empty;
    let grid = TGrid::with_description(&description);

    grid.generate_instructions()
        .draw_to_layer(&doc.get_page(page).get_layer(layer));

    doc.save(&mut BufWriter::new(File::create(output_filename).unwrap()))
        .unwrap();
}
