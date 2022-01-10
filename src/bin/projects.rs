use printpdf::PdfDocumentReference;
use weekly::{
    save_one_page_document, AsPdfLine, Colors, Instructions, LineModifiers, NumericUnit, Unit,
    WLine, WRect,
};

fn render_projects(_: &PdfDocumentReference, page_bounds: &WRect) -> weekly::Result<Instructions> {
    let content_bounds =
        page_bounds.inset_all_q1(0.325.inches(), 0.25.inches(), 0.25.inches(), 0.25.inches());

    let top_left = content_bounds.resize(
        content_bounds.width() / 2 - 0.0625.inches(),
        content_bounds.height() / 2 - 0.0625.inches(),
    );
    let top_right = top_left.move_to(
        content_bounds.left() + content_bounds.width() / 2 + 0.0625.inches(),
        top_left.top(),
    );
    let bottom_left = top_left.move_to(
        top_left.left(),
        content_bounds.height() / 2 + 0.125.inches(),
    );
    let bottom_right = top_right.move_to(
        top_right.left(),
        content_bounds.height() / 2 + 0.125.inches(),
    );

    let mut instructions = Instructions::default();
    fill_project_into_rect(top_left, &mut instructions);
    fill_project_into_rect(top_right, &mut instructions);
    fill_project_into_rect(bottom_left, &mut instructions);
    fill_project_into_rect(bottom_right, &mut instructions);

    Ok(instructions)
}

fn main() -> weekly::Result<()> {
    let doc_title = "Project template";
    let output_filename = "projects.pdf";

    let page_bounds =
        WRect::with_dimensions(5.5.inches(), 8.5.inches()).move_to(0.0.inches(), 8.5.inches());

    save_one_page_document(doc_title, output_filename, &page_bounds, render_projects)
}

fn fill_project_into_rect(rect: WRect, instructions: &mut Instructions) {
    instructions.set_stroke_color(&Colors::gray(0.50));
    instructions.set_stroke_width(1.0);

    // Outline
    instructions.push_shape(
        rect.as_rounded_rect_shape(0.125.inches())
            .fill(false)
            .stroke(true),
    );

    // Project title line
    instructions.push_shape(
        WLine::line(
            rect.left(),
            rect.top() - 0.25.inches(),
            rect.right(),
            rect.top() - 0.25.inches(),
        )
        .as_pdf_line(),
    );

    instructions.set_stroke_color(&Colors::gray(0.75));
    let inner_rect = rect.inset_all_q1(0.125.inches(), 0.25.inches(), 0.125.inches(), 0.0.inches());
    fill_box_with_lines(&inner_rect, 0.25.inches(), 0.195.inches(), instructions);
}

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
