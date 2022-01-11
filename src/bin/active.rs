use printpdf::*;
use weekly::{
    save_one_page_document, AsPdfLine, Colors, GridDescription, Instructions, NumericUnit, TGrid,
    Unit, WLine, WRect,
};

struct ActiveDescription {
    bounds: WRect,
    font: IndirectFontRef,
    task_height: Unit,
}

impl ActiveDescription {
    pub fn with_bounds(
        bounds: WRect,
        font: IndirectFontRef,
        task_height: Unit,
    ) -> ActiveDescription {
        ActiveDescription {
            bounds,
            font,
            task_height,
        }
    }
}

impl GridDescription for ActiveDescription {
    fn bounds(&self) -> WRect {
        self.bounds.clone()
    }

    fn num_cols(&self) -> Option<usize> {
        Some(1)
    }

    fn row_height(&self) -> Option<Unit> {
        Some(self.task_height)
    }

    fn vert_line_style(&self, _index: usize) -> Option<(f64, Color, ())> {
        None
    }

    fn render_cell_contents(
        &self,
        _row: usize,
        _col: usize,
        cell_rect: &WRect,
        instructions: &mut Instructions,
    ) {
        let check_rect = WRect::with_dimensions(self.task_height / 2, self.task_height / 2)
            .move_to(
                cell_rect.left() + self.task_height / 4,
                cell_rect.top() - self.task_height / 4,
            );
        let mut shape = check_rect.as_pdf_line();
        shape.has_fill = false;
        shape.has_stroke = true;
        instructions.push_shape(shape);
    }

    fn font(&self) -> &IndirectFontRef {
        &self.font
    }
}

fn render_active(doc: &PdfDocumentReference, page_bounds: &WRect) -> weekly::Result<Instructions> {
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
    //draw_tasks_in_bounds(right_bounds, &mut instructions, task_height);

    let font = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;
    let description = ActiveDescription::with_bounds(right_bounds, font, task_height);
    let grid = TGrid::with_description(description);
    grid.append_to_instructions(&mut instructions);

    Ok(instructions)
}

fn main() -> weekly::Result<()> {
    let doc_title = "Simple task list";
    let output_filename = "task-list.pdf";
    let page_bounds = weekly::sizes::letter();

    save_one_page_document(doc_title, output_filename, &page_bounds, render_active)
}

fn draw_tasks_in_bounds(bounds: WRect, instructions: &mut Instructions, task_height: Unit) {
    instructions.set_stroke_width(0.0);

    let base_check_rect = WRect::with_dimensions(task_height / 2, task_height / 2);

    let mut curr_y = bounds.top() - task_height;
    while curr_y > bounds.bottom_q1() {
        instructions.set_stroke_color(&Colors::black());
        instructions.set_dash(3, 2);

        let line = WLine::line(bounds.left(), curr_y, bounds.right(), curr_y);
        instructions.push_shape(line.as_pdf_line());

        instructions.set_stroke_color(&Colors::gray(0.5));
        instructions.clear_dash();

        let check_rect =
            base_check_rect.move_to(bounds.left() + task_height / 4, curr_y - task_height / 4);
        if check_rect.bottom_q1() > bounds.bottom_q1() {
            let mut shape = check_rect.as_pdf_line();
            shape.has_fill = false;
            shape.has_stroke = true;
            instructions.push_shape(shape);
        }

        curr_y = curr_y - task_height;
    }
}
