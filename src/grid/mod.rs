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
    rows: u16,
    cols: u16,
    bounds: WRect,
    top_label_height: Unit,
    left_label_width: Unit,
    page_height: Unit,

    instructions: Instructions,
    font: &'a IndirectFontRef,
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
        rows: u16,
        cols: u16,
        bounds: WRect,
        top_label_height: Unit,
        left_label_width: Unit,
        page_height: Unit,
        font: &'f IndirectFontRef,
    ) -> TableGrid<'f> {
        TableGrid {
            row_labels,
            col_labels,
            rows,
            cols,
            bounds: bounds,
            top_label_height,
            left_label_width,
            page_height,

            instructions: Default::default(),
            font,
        }
    }

    fn render_vertical_bars(&mut self) {
        let col_width = (self.bounds.width() - self.left_label_width) / self.cols;

        for col in 0..self.cols {
            let x = self.bounds.left() + self.left_label_width + col_width * col;
            let line = WLine::line(x, self.bounds.top(), x, self.bounds.bottom());
            self.instructions
                .push_shape(line.as_shape(self.page_height));
        }
    }

    fn render_horizontal_bars(&mut self) {
        let row_height =
            (self.bounds.height() - self.top_label_height) / self.rows;

        for row in 0..self.rows as u16 {
            let y = self.bounds.top() + self.top_label_height + row_height * row;
            let line = WLine::line(self.bounds.left(), y, self.bounds.right(), y);
            self.instructions
                .push_shape(line.as_shape(self.page_height));
        }
    }

    fn render_column_backgrounds(&mut self) {
        // This is DRY
        let col_width = (self.bounds.width() - self.left_label_width) / self.cols;

        for col in 0..self.cols {
            if col % 2 == 0 {
                let x = self.bounds.left() + self.left_label_width + col_width * col;
                let rect = WRect::new(x, self.bounds.top(), x + col_width, self.bounds.bottom());
                self.instructions
                    .push_shape(rect.as_shape(self.page_height));
            }
        }
    }

    fn render_row_labels(&mut self) {
        let row_height =
            (self.bounds.height() - self.top_label_height) / self.rows;

        let x = self.bounds.left() + 2.0.mm();
        let text_height = f64::from(row_height) * 1.9;
        for row in 0..min(self.rows, self.row_labels.len() as u16) {
            let y = self.bounds.top() + self.top_label_height + row_height * (row + 1) - 1.5.mm();
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
        let row_height =
            (self.bounds.height() - self.top_label_height) / self.rows as u16;
        let col_width = (self.bounds.width() - self.left_label_width) / self.cols;

        // (159, -21) after rotation.
        let text_height = f64::from(row_height) * 1.9;
        for col in 0..min(self.cols, self.col_labels.len() as u16) {
            let x = self.bounds.left() + self.left_label_width + col_width * (col + 1) - 1.0.mm();
            let y = self.page_height - (self.bounds.top() + self.top_label_height - 1.0.mm());
            self.instructions.push_state();
            self.instructions.rotate(90.0);
            self.instructions.translate(y, -x);
            // Text position is (0,page_height) because we're placing the text by translating (and rotating)
            // the translation matrix. Using the two coord systems together is really problematic,
            // so I'm adjusting for it here, and I should switch to Q1 coords in the near future.
            self.instructions.push_text(
                &self.col_labels[col as usize],
                text_height,
                Unit::zero(),
                self.page_height,
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

        self.instructions.set_stroke_color(&Colors::gray(0.25));
        //.set_stroke_color(&Color::Rgb(Rgb::new(0.25, 0.25, 0.25, None)));
        self.render_horizontal_bars();

        self.instructions.set_fill_color(&Colors::black());
        self.render_row_labels();
        self.render_col_labels();
    }
}
