use crate::pdfutils::Instructions;
use crate::shapes::line::WLine;
use crate::units::Unit;
use crate::WRect;
use printpdf::*;

struct TableGrid {
    rows: u16,
    cols: u16,
    bounds: WRect,
    top_label_height: Unit,
    left_label_width: Unit,
    page_height: Unit,

    instructions: Instructions,
}

impl TableGrid {
    fn new(
        rows: u16,
        cols: u16,
        bounds: &WRect,
        top_label_height: Unit,
        left_label_width: Unit,
        page_height: Unit,
    ) -> TableGrid {
        TableGrid {
            rows,
            cols,
            bounds: (*bounds).clone(),
            top_label_height,
            left_label_width,
            page_height,

            instructions: Default::default(),
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
        let row_height = (self.bounds.height() - self.top_label_height) / self.rows;

        for row in 0..self.rows {
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
                let rect =
                    WRect::new(x, self.bounds.top(), x + col_width, self.bounds.bottom());
                self.instructions
                    .push_shape(rect.as_shape(self.page_height));
            }
        }
    }

    fn generate_grid(&mut self) {
        self.instructions
            .set_fill_color(&Color::Rgb(Rgb::new(0.9, 0.9, 0.9, None)));
        self.render_column_backgrounds();

        self.instructions.set_stroke_width(0.0);

        self.instructions
            .set_stroke_color(&Color::Rgb(Rgb::new(0.75, 0.75, 0.75, None)));
        self.render_vertical_bars();

        self.instructions
            .set_stroke_color(&Color::Rgb(Rgb::new(0.25, 0.25, 0.25, None)));
        self.render_horizontal_bars();
    }
}

pub fn table_grid(
    rows: u16,
    cols: u16,
    bounds: &WRect,
    top_label_height: Unit,
    left_label_width: Unit,
    page_height: Unit,
) -> Instructions {
    let mut grid = TableGrid::new(
        rows,
        cols,
        bounds,
        top_label_height,
        left_label_width,
        page_height,
    );
    grid.generate_grid();
    grid.instructions
}
