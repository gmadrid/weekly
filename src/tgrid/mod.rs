use crate::proxies::ColorProxy;
use crate::tgrid::renderparams::RenderParams;
use crate::{Instructions, NumericUnit, Unit, WLine, WRect};
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
        let num_rows = self.params.num_rows;
        for row in 0..=num_rows {
            if let Some(attrs) = self.params.horiz_line_style(row, num_rows) {
                attrs.render(instructions, |instructions| {
                    let y = top - self.params.row_height * row;
                    instructions.push_line(WLine::line(left, y, right, y));
                });
            }
        }
    }

    fn render_vertical_lines(&self, instructions: &mut Instructions) {
        let top = self.params.grid_bounds.top();
        let bottom =
            top - self.params.col_label_height - self.params.row_height * self.params.num_rows;
        let left = self.params.grid_bounds.left() + self.params.row_label_width;
        let num_cols = self.params.num_cols;
        for col in 0..=num_cols {
            if let Some(attrs) = self.params.vert_line_style(col, num_cols) {
                attrs.render(instructions, |instructions| {
                    let x = left + self.params.col_width * col;
                    instructions.push_line(WLine::line(x, top, x, bottom))
                });
            }
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
                self.params.font,
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
                self.params.font,
            );
            instructions.pop_state();
        }
    }

    fn render_column_backgrounds(&self, instructions: &mut Instructions) {
        let base_col_rect = WRect::with_dimensions(
            self.params.col_width,
            self.params.col_label_height + self.params.row_height * self.params.num_rows,
        );

        for col in 0..self.params.num_cols {
            if let Some(color) = self.params.column_background(col) {
                let x = self.col_x(col);
                let rect = base_col_rect.move_to(x, self.params.grid_bounds.top());
                instructions.set_fill_color(color);
                instructions.push_rect(rect);
            }
        }
    }

    fn render_cell_contents(&self, instructions: &mut Instructions) {
        let cell_rect = WRect::with_dimensions(self.params.col_width, self.params.row_height);

        for row in 0..self.params.num_rows {
            for col in 0..self.params.num_cols {
                let this_rect = cell_rect.move_to(self.col_x(col), self.row_y(row));
                self.params
                    .render_cell_contents(row, col, &this_rect, instructions);
            }
        }
    }

    pub fn append_to_instructions(&self, instructions: &mut Instructions) {
        self.render_column_backgrounds(instructions);
        self.render_cell_contents(instructions);

        // These are the default values for the horiz/vert lines.
        // Ideally, we will set this at the very start and push/pop state as we go.
        // TODO: push/pop state and set this at the very start.
        instructions.set_stroke_width(1.0);
        instructions.set_stroke_color(ColorProxy::black());
        instructions.clear_dash();
        self.render_horizontal_lines(instructions);
        self.render_vertical_lines(instructions);

        // TODO: allow changing text colors.
        instructions.set_fill_color(ColorProxy::black());
        self.render_row_labels(instructions);
        self.render_col_labels(instructions);
    }

    pub fn generate_instructions(&self) -> Instructions {
        let mut instructions = Instructions::default();
        self.append_to_instructions(&mut instructions);
        instructions
    }
}
