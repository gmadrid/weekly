use printpdf::PdfDocumentReference;
use weekly::{
    save_one_page_document, AsPdfLine, Colors, Instructions, LineModifiers, NumericUnit, Unit,
    WLine, WRect,
};

/// Draw horizontal lines starting at 'offset' from the top of the box and at every 'gap'
/// after it. Fill the box with these lines.
///
/// Assumes the box is in Q1.
fn fill_box_with_lines(boxx: &WRect, offset: Unit, gap: Unit, instructions: &mut Instructions) {
    let mut curr_y = boxx.top() - offset;

    while curr_y > boxx.bottom_q1() {
        let line = WLine::line(boxx.left(), curr_y, boxx.right(), curr_y);
        instructions.push_shape(line.as_pdf_line());
        curr_y = curr_y - gap;
    }
}

fn render_tester(_: &PdfDocumentReference, page_bounds: &WRect) -> Instructions {
    let top_left = page_bounds.resize(page_bounds.width() / 2, page_bounds.height() / 2);
    top_left.inset_q1(0.125.inches(), 0.125.inches());
    let bottom_right = top_left.move_to(page_bounds.width() / 2, page_bounds.height() / 2);
    let top_right = page_bounds
        .resize(page_bounds.width() / 2, page_bounds.height() / 2)
        .move_to(page_bounds.width() / 2, page_bounds.top());

    let mut instructions = Instructions::default();

    instructions.set_stroke_width(1.0);
    instructions.set_stroke_color(&Colors::black());
    instructions.push_shape(top_left.as_pdf_line().fill(false).stroke(true));

    let mut shape = bottom_right.as_pdf_line();
    shape.has_fill = false;
    shape.has_stroke = true;
    instructions.push_shape(shape);

    instructions.push_shape(top_right.as_pdf_line().fill(false).stroke(true));

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
    let mut bam = abox.as_pdf_line();
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

    instructions
}

fn main() {
    let doc_title = "Testing new stuff";
    let output_filename = "tester.pdf";

    // Make the page box and shift it to account for Q1 math.
    let page_bounds =
        WRect::with_dimensions(5.5.inches(), 8.5.inches()).move_to(0.0.inches(), 8.5.inches());

    save_one_page_document(doc_title, output_filename, &page_bounds, render_tester);
}
