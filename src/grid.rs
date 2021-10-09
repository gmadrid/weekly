use crate::shapes::line::WLine;
use crate::{Unit, WRect};

struct TableGrid {
    rows: u16,
    cols: u16,
    bounds: WRect,
    top_label_height: Unit,
    left_label_width: Unit,

    lines: Vec<WLine>,
}

impl TableGrid {
    fn new(
        rows: u16,
        cols: u16,
        bounds: &WRect,
        top_label_height: Unit,
        left_label_width: Unit,
    ) -> TableGrid {
        TableGrid {
            rows,
            cols,
            bounds: (*bounds).clone(),
            top_label_height,
            left_label_width,

            lines: Default::default(),
        }
    }

    fn render_vertical_bars(&mut self) {
        let col_width = (self.bounds.width() - self.left_label_width) / self.cols;

        for col in 0..self.cols {
            let x = self.bounds.left() + self.left_label_width + col_width * col;
            let line = WLine::line(x, self.bounds.top(), x, self.bounds.bottom());
            self.lines.push(line);
        }
    }

    fn render_horizontal_bars(&mut self) {
        let row_height = (self.bounds.height() - self.top_label_height) / self.rows;

        for row in 0..self.rows {
            let y = self.bounds.top() + self.top_label_height + row_height * row;
            let line = WLine::line(self.bounds.left(), y, self.bounds.right(), y);
            self.lines.push(line);
        }
    }

    fn generate_grid(&mut self) {
        self.render_vertical_bars();
        self.render_horizontal_bars();
    }
}

pub fn table_grid(
    rows: u16,
    cols: u16,
    bounds: &WRect,
    top_label_height: Unit,
    left_label_width: Unit,
) -> Vec<WLine> {
    let mut grid = TableGrid::new(rows, cols, bounds, top_label_height, left_label_width);
    grid.generate_grid();
    grid.lines
}
