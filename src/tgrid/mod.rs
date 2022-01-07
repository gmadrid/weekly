use crate::tgrid::renderparams::RenderParams;
use crate::{AsPdfLine, Instructions, NumericUnit, Unit, WLine};
use description::GridDescription;

pub mod description;
mod renderparams;

pub struct TGrid<D>
where
    D: GridDescription,
{
    params: RenderParams<D>,
}

impl<D> TGrid<D>
where
    D: GridDescription,
{
    pub fn with_description(description: D) -> TGrid<D> {
        TGrid {
            params: description.into(),
        }
    }

    fn render_horizontal_lines(&self, instructions: &mut Instructions) {
        let left = self.params.grid_bounds.left();
        let right =
            left + self.params.row_label_width + self.params.col_width * self.params.num_cols;
        let top = self.params.grid_bounds.top() - self.params.col_label_height;
        for row in 0..(self.params.num_rows + 1) {
            // TODO: don't call this unless it's changed.
            instructions.set_stroke_width(self.params.horiz_line_width(row));

            let y = top - self.params.row_height * row;
            instructions.push_shape(WLine::line(left, y, right, y).as_pdf_line())
        }
    }

    fn render_vertical_lines(&self, instructions: &mut Instructions) {
        let top = self.params.grid_bounds.top();
        let bottom =
            top - self.params.col_label_height - self.params.row_height * self.params.num_rows;
        let left = self.params.grid_bounds.left() + self.params.row_label_width;
        // NOTE: I'm not drawing the last line because it looks funny when it
        // gets styled.
        // TODO: find a way to indicate that it's the last line so we can
        // avoid styling it.
        for col in 0..self.params.num_cols {
            // TODO: don't call this unless it's changed.
            instructions.set_stroke_width(self.params.vert_line_width(col));

            let x = left + self.params.col_width * col;
            instructions.push_shape(WLine::line(x, top, x, bottom).as_pdf_line())
        }
    }

    fn row_y(&self, row: usize) -> Unit {
        self.params.grid_bounds.top() - self.params.col_label_height - self.params.row_height * row
    }

    fn col_x(&self, col: usize) -> Unit {
        self.params.grid_bounds.left() + self.params.row_label_width + self.params.col_width * col
    }

    fn render_row_labels(&self, instructions: &mut Instructions) {
        if !self.params.has_row_labels {
            return;
        }

        let row_height = self.params.row_height;

        let x = self.params.grid_bounds.left() + 2.0.mm();
        let text_height = f64::from(row_height) * 1.9;
        for row in 0..self.params.num_rows {
            let y = self.row_y(row + 1) + 1.5.mm();
            instructions.push_text(
                self.params.row_label(row).as_ref(),
                text_height,
                x,
                y,
                &self.params.font,
            );
        }
    }

    fn render_col_labels(&self, instructions: &mut Instructions) {
        if !self.params.has_col_labels {
            return;
        }

        let row_height = self.params.row_height;

        // (159, -21) after rotation.
        let text_height = f64::from(row_height) * 1.9;
        let y = self.params.grid_bounds.top() - self.params.col_label_height + 1.0.mm();
        for col in 0..self.params.num_cols {
            let x = self.col_x(col + 1) - 1.0.mm();

            instructions.push_state();
            instructions.rotate(90.0);
            instructions.translate(y, -x);

            // Text position is (0.0), so that we can rotate the text before translating it.
            instructions.push_text(
                self.params.col_label(col).as_ref(),
                text_height,
                Unit::zero(),
                Unit::zero(),
                &self.params.font,
            );
            instructions.pop_state();
        }
    }

    pub fn generate_instructions(&self) -> Instructions {
        let mut instructions = Instructions::default();

        self.render_horizontal_lines(&mut instructions);
        self.render_vertical_lines(&mut instructions);

        self.render_row_labels(&mut instructions);
        self.render_col_labels(&mut instructions);

        instructions
    }
}
