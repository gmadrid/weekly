use crate::pdfutils::Attributes;
use crate::{GridDescription, Instructions, Unit, WRect};
use printpdf::{Color, IndirectFontRef};
use std::borrow::Cow;

#[derive(Debug)]
pub struct RenderParams<D>
where
    D: GridDescription,
{
    description: D,

    pub grid_bounds: WRect,

    pub row_height: Unit,
    pub col_width: Unit,
    pub num_rows: usize,
    pub num_cols: usize,

    pub has_row_labels: bool,
    pub row_label_width: Unit,

    pub has_col_labels: bool,
    pub col_label_height: Unit,

    pub font: IndirectFontRef,
}

impl<D> RenderParams<D>
where
    D: GridDescription,
{
    pub fn row_label(&self, index: usize) -> Cow<'static, str> {
        self.description.row_label(index)
    }

    pub fn col_label(&self, index: usize) -> Cow<'static, str> {
        self.description.col_label(index)
    }

    pub fn horiz_line_style(&self, index: usize) -> Option<Attributes> {
        self.description.horiz_line_style(index)
    }

    pub fn vert_line_style(&self, index: usize) -> Option<(f64, Color, ())> {
        self.description.vert_line_style(index)
    }

    pub fn column_background(&self, index: usize) -> Option<Color> {
        self.description.column_background(index)
    }

    pub fn render_cell_contents(
        &self,
        row: usize,
        col: usize,
        rect: &WRect,
        instructions: &mut Instructions,
    ) {
        self.description
            .render_cell_contents(row, col, rect, instructions);
    }
}

impl<D> From<D> for RenderParams<D>
where
    D: GridDescription,
{
    fn from(description: D) -> Self {
        if description.row_height().is_none() && description.num_rows().is_none() {
            panic!("either row height or num rows must be set");
        }
        if description.col_width().is_none() && description.num_cols().is_none() {
            panic!("either col width or num cols must be set");
        }

        let grid_bounds = description.bounds();

        let has_row_labels = description.row_label_width().is_some();
        let has_col_labels = description.col_label_height().is_some();
        let row_label_width = description.row_label_width().unwrap_or_else(Unit::zero);
        let col_label_height = description.col_label_height().unwrap_or_else(Unit::zero);

        let num_rows = description.num_rows().unwrap_or_else(|| {
            // unwrap: we check that both num_rows and row_height cannot be none.
            // If num_rows isn't set, we compute it from the label size, bounds, and cell size.
            (grid_bounds.height() - col_label_height) / description.row_height().unwrap()
        });
        let num_cols = description.num_cols().unwrap_or_else(|| {
            // unwrap: we check that both num_cols and col_width cannot be none.
            // If num_cols isn't set, we compute it from the label size, bounds, and cell size.
            (grid_bounds.width() - row_label_width) / description.col_width().unwrap()
        });

        let row_height = description.row_height().unwrap_or_else(|| {
            // unwrap: we check that both num_rows and row_height cannot be none.
            // If row_height isn't set, we compute is from the label size, bounds, and num rows.
            (grid_bounds.height() - col_label_height) / description.num_rows().unwrap()
        });
        let col_width = description.col_width().unwrap_or_else(|| {
            // unwrap: we check that both num_cols and col_width cannot be none.
            // If col_width isn't set, we compute is from the label size, bounds, and num cols.
            (grid_bounds.width() - row_label_width) / description.num_cols().unwrap()
        });

        let font = description.font().clone();

        RenderParams {
            description,
            grid_bounds,
            row_height,
            col_width,
            num_rows,
            num_cols,
            has_row_labels,
            row_label_width,
            has_col_labels,
            col_label_height,
            font,
        }
    }
}
