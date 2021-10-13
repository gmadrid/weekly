use printpdf::PdfDocument;
use std::fs::File;
use std::io::BufWriter;
use weekly::{Colors, Instructions, LineModifiers, NumericUnit, Unit, WLine, WRect};

/// Draw horizontal lines starting at 'offset' from the top of the box and at every 'gap'
/// after it. Fill the box with these lines.
///
/// Assumes the box is in Q1.
fn fill_box_with_lines(boxx: &WRect, offset: Unit, gap: Unit, instructions: &mut Instructions) {
    let mut curr_y = boxx.top() - offset;

    while curr_y > boxx.bottom_q1() {
        let line = WLine::line(boxx.left(), curr_y, boxx.right(), curr_y);
        instructions.push_shape(line.as_shape());
        curr_y = curr_y - gap;
    }
}

fn main() {
    let doc_title = "Testing new stuff";
    let output_filename = "tester.pdf";

    // Make the page box and shift it to account for Q1 math.
    let page_bounds =
        WRect::with_dimensions(5.5.inches(), 8.5.inches()).move_to(0.0.inches(), 8.5.inches());

    let top_left = page_bounds.resize(page_bounds.width() / 2, page_bounds.height() / 2);
    top_left.inset_q1(0.125.inches(), 0.125.inches());
    let bottom_right = top_left.move_to(page_bounds.width() / 2, page_bounds.height() / 2);
    let top_right = page_bounds
        .resize(page_bounds.width() / 2, page_bounds.height() / 2)
        .move_to(page_bounds.width() / 2, page_bounds.top());

    let mut instructions = Instructions::default();

    instructions.set_stroke_width(1.0);
    instructions.set_stroke_color(&Colors::black());
    instructions.push_shape(top_left.as_shape().fill(false).stroke(true));

    let mut shape = bottom_right.as_shape();
    shape.has_fill = false;
    shape.has_stroke = true;
    instructions.push_shape(shape);

    instructions.push_shape(top_right.as_shape().fill(false).stroke(true));

    instructions.set_fill_color(&Colors::green());
    instructions.set_stroke_color(&Colors::red());
    let center_rect = page_bounds
        .resize(page_bounds.width() / 2, page_bounds.height() / 2)
        .move_to(
            page_bounds.left() + page_bounds.width() / 4,
            page_bounds.top() - page_bounds.height() / 4,
        );
    instructions.push_shape(
        center_rect
            .as_rounded_rect_shape(0.125.inches())
            .stroke(true),
    );
    instructions.set_stroke_color(&Colors::black());

    let abox = top_left.inset_q1(0.125.inches(), 0.125.inches());
    let mut bam = abox.as_shape();
    bam.has_fill = false;
    bam.has_stroke = true;
    instructions.push_shape(bam);
    fill_box_with_lines(&abox, 0.5.inches(), 0.3.inches(), &mut instructions);

    fill_box_with_lines(
        &bottom_right.inset_q1(0.125.inches(), 0.125.inches()),
        0.5.inches(),
        0.15.inches(),
        &mut instructions,
    );

    let (doc, page, layer) = PdfDocument::new(
        doc_title,
        page_bounds.width().into(),
        page_bounds.height().into(),
        "Layer 1",
    );
    //let times_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
    let layer = doc.get_page(page).get_layer(layer);
    instructions.draw_to_layer(&layer);

    doc.save(&mut BufWriter::new(File::create(output_filename).unwrap()))
        .unwrap();
}
