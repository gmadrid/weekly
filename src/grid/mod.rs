use crate::pdfutils::{Colors, Instructions};
use crate::shapes::line::WLine;
use crate::units::Unit;
use crate::{NumericUnit, WRect};
use printpdf::*;
use std::cmp::min;

mod builder;
pub use builder::Builder;

// Maybe the builder should return instructsions, and not the TableGrid.
pub struct TableGrid<'a> {
    row_labels: &'a [String],
    col_labels: &'a [String],
    rows: usize,
    cols: usize,
    bounds: WRect,
    top_label_height: Unit,
    left_label_width: Unit,

    instructions: Instructions,
    font: &'a IndirectFontRef,
    width_func: Box<dyn Fn(usize) -> f64>,
}

impl<'a> TableGrid<'a> {
    pub fn instructions(mut self) -> Instructions {
        self.generate_grid();
        self.instructions
    }

    fn new<'f>(
        _doc_title: &str, // unused for now.
        row_labels: &'f [String],
        col_labels: &'f [String],
        rows: usize,
        cols: usize,
        bounds: WRect,
        top_label_height: Unit,
        left_label_width: Unit,
        font: &'f IndirectFontRef,
        width_func: Box<dyn Fn(usize) -> f64>,
    ) -> TableGrid<'f> {
        TableGrid {
            row_labels,
            col_labels,
            rows,
            cols,
            bounds,
            top_label_height,
            left_label_width,

            instructions: Default::default(),
            font,
            width_func,
        }
    }

    fn render_vertical_bars(&mut self) {
        let col_width = (self.bounds.width() - self.left_label_width) / self.cols;

        for col in 0..self.cols {
            let x = self.bounds.left() + self.left_label_width + col_width * col;
            let line = WLine::line(x, self.bounds.top(), x, self.bounds.bottom_q1());
            self.instructions.push_shape(line.as_shape());
        }
    }

    fn render_horizontal_bars(&mut self) {
        let row_height = (self.bounds.height() - self.top_label_height) / self.rows;
        let wf = &self.width_func;
        self.instructions.set_stroke_color(&Colors::gray(0.25));

        for row in 0..self.rows {
            self.instructions.set_stroke_width(wf(row as usize));
            let y = self.bounds.top() - self.top_label_height - row_height * row;
            let line = WLine::line(self.bounds.left(), y, self.bounds.right(), y);
            self.instructions.push_shape(line.as_shape());
        }
    }

    fn render_column_backgrounds(&mut self) {
        // This is DRY
        let col_width = (self.bounds.width() - self.left_label_width) / self.cols;
        let base_col_rect = WRect::with_dimensions(col_width, self.bounds.height());

        for col in 0..self.cols {
            if col % 2 == 0 {
                let x = self.bounds.left() + self.left_label_width + col_width * col;
                // let rect = WRect::new(x, self.bounds.top(), x + col_width, self.bounds.bottom_q1());
                let rect = base_col_rect.move_to(x, self.bounds.top());
                self.instructions.push_shape(rect.as_shape());
            }
        }
    }

    fn render_row_labels(&mut self) {
        let row_height = (self.bounds.height() - self.top_label_height) / self.rows;

        let x = self.bounds.left() + 2.0.mm();
        let text_height = f64::from(row_height) * 1.9;
        for row in 0..min(self.rows, self.row_labels.len()) {
            let y = self.bounds.top() - self.top_label_height - row_height * (row + 1) + 1.5.mm();
            self.instructions.push_text(
                &self.row_labels[row as usize],
                text_height,
                x,
                y,
                self.font,
            );
        }
    }

    fn render_col_labels(&mut self) {
        // This is DRY
        let row_height = (self.bounds.height() - self.top_label_height) / self.rows;
        let col_width = (self.bounds.width() - self.left_label_width) / self.cols;

        // (159, -21) after rotation.
        let text_height = f64::from(row_height) * 1.9;
        for col in 0..min(self.cols, self.col_labels.len()) {
            let x = self.bounds.left() + self.left_label_width + col_width * (col + 1) - 1.0.mm();
            let y = self.bounds.top() - self.top_label_height + 1.0.mm();

            self.instructions.push_state();
            self.instructions.rotate(90.0);
            self.instructions.translate(y, -x);

            // Text position is (0.0), so that we can rotate the text before translating it.
            self.instructions.push_text(
                &self.col_labels[col as usize],
                text_height,
                Unit::zero(),
                Unit::zero(),
                self.font,
            );
            self.instructions.pop_state();
        }
    }

    fn generate_grid(&mut self) {
        self.instructions.set_fill_color(&Colors::gray(0.9));
        self.render_column_backgrounds();

        self.instructions.set_stroke_width(0.0);
        self.instructions.set_stroke_color(&Colors::gray(0.75));
        self.render_vertical_bars();

        self.render_horizontal_bars();

        self.instructions.set_fill_color(&Colors::black());
        self.render_row_labels();
        self.render_col_labels();
    }
}
