use crate::grid::TableGrid;
use crate::units::Unit;
use crate::{Instructions, NumericUnit, WRect};
use printpdf::*;

#[derive(Default)]
pub struct Builder<'a> {
    doc_title: Option<String>,
    row_labels: Option<&'a [String]>,
    col_labels: Option<&'a [String]>,
    num_rows: Option<usize>,
    num_cols: Option<usize>,
    bounds: Option<WRect>,
    top_label_height: Option<Unit>,
    left_label_width: Option<Unit>,
    font: Option<&'a IndirectFontRef>,
    horiz_line_width_func: Option<&'a dyn Fn(usize) -> f64>,
    vert_line_width_func: Option<&'a dyn Fn(usize) -> f64>,
    cell_background_func: Option<&'a dyn Fn(usize, usize) -> Option<Color>>,
}

impl<'a> Builder<'a> {
    pub fn new<'b>() -> Builder<'b> {
        Builder::default()
    }

    pub fn generate_instructions(&self) -> Instructions {
        self.build().instructions()
    }

    fn build(&self) -> TableGrid<'a> {
        let row_labels = self.row_labels.unwrap_or_default();
        let col_labels = self.col_labels.unwrap_or_default();

        let rows = self.num_rows.unwrap_or_else(|| row_labels.len());
        let cols = self.num_cols.unwrap_or_else(|| col_labels.len());

        let bounds = self
            .bounds
            .clone()
            .unwrap_or_else(|| WRect::with_dimensions(1.0.inches(), 1.0.inches()));
        let top_label_height = self.top_label_height.unwrap_or_else(|| 0.5.inches());
        let left_label_width = self.left_label_width.unwrap_or_else(|| 0.5.inches());

        let font = if let Some(font) = self.font {
            font
        } else {
            panic!("A font must be set to create a grid.");
        };

        // TODO: do something with doc_title.
        TableGrid {
            row_labels,
            col_labels,
            rows,
            cols,
            bounds,
            top_label_height,
            left_label_width,
            font,
            horiz_line_width_func: self.horiz_line_width_func,
            vert_line_width_func: self.vert_line_width_func,
            cell_background_func: self.cell_background_func,
        }
    }

    pub fn horiz_line_width_func(mut self, f: &'a (dyn Fn(usize) -> f64)) -> Builder<'a> {
        self.horiz_line_width_func = Some(f);
        self
    }

    pub fn vert_line_width_func(mut self, f: &'a (dyn Fn(usize) -> f64)) -> Builder<'a> {
        self.vert_line_width_func = Some(f);
        self
    }

    pub fn cell_background_func(
        mut self,
        f: &'a (dyn Fn(usize, usize) -> Option<Color>),
    ) -> Builder<'a> {
        self.cell_background_func = Some(f);
        self
    }

    pub fn doc_title(mut self, title: impl Into<String>) -> Builder<'a> {
        self.doc_title = Some(title.into());
        self
    }

    pub fn row_labels(mut self, row_labels: &'a [String]) -> Builder<'a> {
        self.row_labels = Some(row_labels);
        self
    }

    pub fn col_labels(mut self, col_labels: &'a [String]) -> Builder<'a> {
        self.col_labels = Some(col_labels);
        self
    }

    pub fn num_rows(mut self, num_rows: usize) -> Builder<'a> {
        self.num_rows = Some(num_rows);
        self
    }

    pub fn num_cols(mut self, num_cols: usize) -> Builder<'a> {
        self.num_cols = Some(num_cols);
        self
    }

    pub fn bounds(mut self, bounds: WRect) -> Builder<'a> {
        self.bounds = Some(bounds);
        self
    }

    pub fn top_label_height(mut self, top_label_height: Unit) -> Builder<'a> {
        self.top_label_height = Some(top_label_height);
        self
    }

    pub fn left_label_width(mut self, left_label_width: Unit) -> Builder<'a> {
        self.left_label_width = Some(left_label_width);
        self
    }

    pub fn font(mut self, font: &'a IndirectFontRef) -> Builder<'a> {
        self.font = Some(font);
        self
    }
}
