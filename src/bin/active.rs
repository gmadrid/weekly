use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use weekly::{Colors, Instructions, NumericUnit, Unit, WLine, WRect};

fn main() {
    let doc_title = "Simple task list";
    let output_filename = "task-list.pdf";

    // Make the page box and shift it to account for Q1 math.
    let page_bounds =
        WRect::with_dimensions(5.5.inches(), 8.5.inches()).move_to(0.0.inches(), 8.5.inches());

    let half_page = page_bounds.resize(page_bounds.width() / 2, page_bounds.height());
    let left_bounds = half_page.inset_all_q1(
        0.25.inches() + 0.125.inches(),
        0.25.inches(),
        0.25.inches(),
        0.25.inches(),
    );
    let right_bounds = left_bounds.move_to(half_page.right() + 0.25.inches(), left_bounds.top());

    let mut instructions = Instructions::default();

    let task_height = 0.25.inches();

    draw_tasks_in_bounds(left_bounds, &mut instructions, task_height);
    draw_tasks_in_bounds(right_bounds, &mut instructions, task_height);

    let (doc, page, layer) = PdfDocument::new(
        doc_title,
        page_bounds.width().into(),
        page_bounds.height().into(),
        "Layer 1",
    );
    //let times_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
    let layer = doc.get_page(page).get_layer(layer);
    instructions.draw_to_layer(&layer);

    let (page_2_index, layer_2_index) = doc.add_page(page_bounds.width().into(), page_bounds.height().into(), "Layer 2");
    let layer_2 = doc.get_page(page_2_index).get_layer(layer_2_index);
    instructions.draw_to_layer(&layer_2);

    doc.save(&mut BufWriter::new(File::create(output_filename).unwrap()))
        .unwrap();

    //    Ok(())
}

fn draw_tasks_in_bounds(bounds: WRect, instructions: &mut Instructions, task_height: Unit) {
    instructions.set_stroke_width(0.0);

    let base_check_rect = WRect::with_dimensions(task_height / 2, task_height / 2);

    let mut curr_y = bounds.top() - task_height;
    while curr_y > bounds.bottom_q1() {
        instructions.set_stroke_color(&Colors::black());
        instructions.set_dash(3, 2);

        let line = WLine::line(bounds.left(), curr_y, bounds.right(), curr_y);
        instructions.push_shape(line.as_shape());

        instructions.set_stroke_color(&Colors::gray(0.9));
        instructions.clear_dash();

        let check_rect =
            base_check_rect.move_to(bounds.left() + task_height / 4, curr_y - task_height / 4);
        if check_rect.bottom_q1() > bounds.bottom_q1() {
            let mut shape = check_rect.as_shape();
            shape.has_fill = false;
            shape.has_stroke = true;
            instructions.push_shape(shape);
        }

        curr_y = curr_y - task_height;
    }
}
