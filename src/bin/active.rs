use printpdf::*;
use weekly::{
    save_one_page_document, Attributes, GridDescription, Instructions, NumericUnit, TGrid,
    ToPdfLine, Unit, WRect,
};

struct ActiveDescription {
    bounds: WRect,
    task_height: Unit,
}

impl ActiveDescription {
    pub fn with_bounds(bounds: WRect, task_height: Unit) -> ActiveDescription {
        ActiveDescription {
            bounds,
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

    fn horiz_line_style(&self, _index: usize, _num_rows: usize) -> Option<Attributes> {
        Some(Attributes::default().with_stroke_width(0.0).with_dash(3, 2))
    }

    fn vert_line_style(&self, _index: usize, _num_cols: usize) -> Option<Attributes> {
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
        instructions.push_shape(check_rect.to_stroked_line());
    }
}

fn render_active(_: &PdfDocumentReference, page_bounds: &WRect) -> weekly::Result<Instructions> {
    let half_page = page_bounds
        .resize(page_bounds.width() / 2, page_bounds.height())
        // A rounding error prevents rendering the last line,
        // so we add a smidge of extra vertical space.
        .inset_all_q1(Unit::zero(), Unit::zero(), Unit::zero(), (-0.1).inches());
    let left_bounds =
        half_page.inset_all_q1(0.25.inches(), 0.25.inches(), 0.125.inches(), 0.25.inches());
    let right_bounds = left_bounds.move_to(half_page.right() + 0.125.inches(), left_bounds.top());

    let mut instructions = Instructions::default();

    let task_height = 0.25.inches();

    let description = ActiveDescription::with_bounds(left_bounds, task_height);
    let grid = TGrid::with_description(description);
    grid.append_to_instructions(&mut instructions);

    let description = ActiveDescription::with_bounds(right_bounds, task_height);
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
