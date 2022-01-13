use printpdf::PdfDocumentReference;
use weekly::{save_one_page_document, ColorProxy, Instructions, NumericUnit, Unit, WLine, WRect};

/// Draw horizontal lines starting at 'offset' from the top of the box and at every 'gap'
/// after it. Fill the box with these lines.
///
/// Assumes the box is in Q1.
fn fill_box_with_lines(boxx: &WRect, offset: Unit, gap: Unit, instructions: &mut Instructions) {
    let mut curr_y = boxx.top() - offset;

    while curr_y > boxx.bottom_q1() {
        let line = WLine::line(boxx.left(), curr_y, boxx.right(), curr_y);
        instructions.push_line(line);
        curr_y = curr_y - gap;
    }
}

fn render_tester(_: &PdfDocumentReference, page_bounds: &WRect) -> weekly::Result<Instructions> {
    let top_left = page_bounds.resize(page_bounds.width() / 2, page_bounds.height() / 2);
    top_left.inset_q1(0.125.inches(), 0.125.inches());
    let bottom_right = top_left.move_to(page_bounds.width() / 2, page_bounds.height() / 2);
    let top_right = page_bounds
        .resize(page_bounds.width() / 2, page_bounds.height() / 2)
        .move_to(page_bounds.width() / 2, page_bounds.top());

    let mut instructions = Instructions::default();

    instructions.set_stroke_width(1.0);
    instructions.set_stroke_color(ColorProxy::black());
    instructions.push_rect(top_left.fill(false).stroke(true));

    let shape = bottom_right.fill(false).stroke(true);
    instructions.push_rect(shape);

    instructions.push_rect(top_right.fill(false).stroke(true));

    instructions.set_fill_color(ColorProxy::green());
    instructions.set_stroke_color(ColorProxy::red());
    let _center_rect = page_bounds
        .resize(page_bounds.width() / 2, page_bounds.height() / 2)
        .move_to(
            page_bounds.left() + page_bounds.width() / 4,
            page_bounds.top() - page_bounds.height() / 4,
        );
    // instructions.push_rect(
    //     center_rect
    //         .as_rounded_rect_shape(0.125.inches())
    //         .stroke(true),
    // );
    instructions.set_stroke_color(ColorProxy::black());

    let abox = top_left
        .inset_q1(0.125.inches(), 0.125.inches())
        .fill(false)
        .stroke(true);
    instructions.push_rect(abox.clone());
    fill_box_with_lines(&abox, 0.5.inches(), 0.3.inches(), &mut instructions);

    fill_box_with_lines(
        &bottom_right.inset_q1(0.125.inches(), 0.125.inches()),
        0.5.inches(),
        0.15.inches(),
        &mut instructions,
    );

    Ok(instructions)
}

fn main() -> weekly::Result<()> {
    let doc_title = "Testing new stuff";
    let output_filename = "tester.pdf";

    // Make the page box and shift it to account for Q1 math.
    let page_bounds =
        WRect::with_dimensions(5.5.inches(), 8.5.inches()).move_to(0.0.inches(), 8.5.inches());

    save_one_page_document(doc_title, output_filename, &page_bounds, render_tester)
}
