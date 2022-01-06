use crate::pdfutils::{Colors, Instructions, LineModifiers};
use crate::shapes::line::WLine;
use crate::shapes::AsPdfLine;
use crate::units::Unit;
use crate::{NumericUnit, WRect};
use printpdf::*;
use std::cmp::min;

mod builder;
pub use builder::Builder;

// Maybe the builder should return instructions, and not the TableGrid.
pub struct TableGrid<'a> {
    row_labels: &'a [&'a str],
    col_labels: &'a [&'a str],
    rows: usize,
    cols: usize,
    bounds: WRect,
    top_label_height: Unit,
    left_label_width: Unit,
    box_width: Option<Unit>,

    font: &'a IndirectFontRef,
    horiz_line_width_func: Option<&'a dyn Fn(usize) -> f64>,
    vert_line_width_func: Option<&'a dyn Fn(usize) -> f64>,
    cell_background_func: Option<&'a dyn Fn(usize, usize) -> Option<Color>>,
}

impl<'a> TableGrid<'a> {
    pub fn instructions(&self) -> Instructions {
        let mut instructions = Instructions::default();
        self.generate_grid(&mut instructions);
        instructions
    }

    fn render_vertical_bars(&self, instructions: &mut Instructions) {
        for col in 0..self.cols {
            let stroke_width = self
                .vert_line_width_func
                .map(|f| f(col as usize))
                .unwrap_or(0.0);
            instructions.set_stroke_width(stroke_width);

            let x = self.col_x(col);
            let line = WLine::line(x, self.bounds.top(), x, self.bounds.bottom_q1());
            instructions.push_shape(line.as_pdf_line());
        }
    }

    fn col_x(&self, col: usize) -> Unit {
        self.bounds.left() + self.left_label_width + self.col_width() * col
    }

    fn render_horizontal_bars(&self, instructions: &mut Instructions) {
        instructions.set_stroke_color(&Colors::gray(0.25));

        for row in 0..self.rows {
            let stroke_width = self
                .horiz_line_width_func
                .map(|f| f(row as usize))
                .unwrap_or(0.0);
            instructions.set_stroke_width(stroke_width);

            let y = self.row_y(row);
            let line = WLine::line(self.bounds.left(), y, self.bounds.right(), y);
            instructions.push_shape(line.as_pdf_line());
        }
    }

    fn row_y(&self, row: usize) -> Unit {
        self.bounds.top() - self.top_label_height - self.row_height() * row
    }

    fn render_column_backgrounds(&self, instructions: &mut Instructions) {
        // This is DRY
        let base_col_rect = WRect::with_dimensions(self.col_width(), self.bounds.height());

        for col in 0..self.cols {
            if col % 2 == 0 {
                let x = self.col_x(col);
                let rect = base_col_rect.move_to(x, self.bounds.top());
                instructions.push_shape(rect.as_pdf_line());
            }
        }
    }

    fn render_cell_backgrounds(&self, instructions: &mut Instructions) {
        let cell_rect = WRect::with_dimensions(self.col_width(), self.row_height());

        for row in 0..self.rows {
            for col in 0..self.cols {
                if let Some(f) = self.cell_background_func {
                    if let Some(color) = f(row, col) {
                        let this_rect = cell_rect.move_to(self.col_x(col), self.row_y(row));
                        instructions.set_fill_color(&color);
                        instructions.push_shape(this_rect.as_pdf_line());
                    }
                }
            }
        }
    }

    fn render_row_labels(&self, instructions: &mut Instructions) {
        let row_height = self.row_height();

        let x = self.bounds.left() + 2.0.mm();
        let text_height = f64::from(row_height) * 1.9;
        for row in 0..min(self.rows, self.row_labels.len()) {
            let y = self.row_y(row + 1) + 1.5.mm();
            instructions.push_text(self.row_labels[row as usize], text_height, x, y, self.font);
        }
    }

    fn render_col_labels(&self, instructions: &mut Instructions) {
        // This is DRY
        let row_height = self.row_height();

        // (159, -21) after rotation.
        let text_height = f64::from(row_height) * 1.9;
        let y = self.bounds.top() - self.top_label_height + 1.0.mm();
        for col in 0..min(self.cols, self.col_labels.len()) {
            let x = self.col_x(col + 1) - 1.0.mm();

            instructions.push_state();
            instructions.rotate(90.0);
            instructions.translate(y, -x);

            // Text position is (0.0), so that we can rotate the text before translating it.
            instructions.push_text(
                self.col_labels[col as usize],
                text_height,
                Unit::zero(),
                Unit::zero(),
                self.font,
            );
            instructions.pop_state();
        }
    }

    fn render_check_boxes(&self, instructions: &mut Instructions) {
        instructions.clear_fill_color();
        instructions.set_stroke_color(&Colors::gray(0.25));
        instructions.set_stroke_width(0.0);
        if let Some(box_width) = self.box_width {
            let x_offset = (self.col_width() - box_width) / 2;
            let y_offset = (self.row_height() - box_width) / 2;
            for row in 0..self.rows {
                for col in 0..self.cols {
                    // TODO: get rid of this unwrap
                    if self.cell_background_func.unwrap()(row, col).is_none() {
                        instructions.push_shape(
                            WRect::with_dimensions(box_width, box_width)
                                .move_to(self.col_x(col) + x_offset, self.row_y(row) - y_offset)
                                .as_pdf_line()
                                .fill(false)
                                .stroke(true),
                        );
                    }
                }
            }
        }
    }

    fn row_height(&self) -> Unit {
        (self.bounds.height() - self.top_label_height) / self.rows
    }

    fn col_width(&self) -> Unit {
        (self.bounds.width() - self.left_label_width) / self.cols
    }

    fn generate_grid(&self, instructions: &mut Instructions) {
        instructions.set_fill_color(&Colors::gray(0.9));
        self.render_column_backgrounds(instructions);
        self.render_cell_backgrounds(instructions);

        self.render_vertical_bars(instructions);
        self.render_horizontal_bars(instructions);

        self.render_check_boxes(instructions);

        instructions.set_fill_color(&Colors::black());
        self.render_row_labels(instructions);
        self.render_col_labels(instructions);
    }
}
