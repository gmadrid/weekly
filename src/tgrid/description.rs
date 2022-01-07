use printpdf::IndirectFontRef;
use std::borrow::Cow;

use crate::{NumericUnit, Unit, WRect};

pub trait GridDescription {
    // Returns the page bounds of the table.
    // Everything rendered for the table should fit into this rectangle.
    //
    // Defaults to 8.5x11 inches (one sheet of copy paper) shifted to account for Q1 math.
    fn bounds(&self) -> WRect {
        WRect::with_dimensions(8.5.inches(), 11.0.inches()).move_to(Unit::zero(), 11.0.inches())
    }

    // Returns the number of rows/cols in the final table. Returning None will auto-compute these
    // values based on the bounds and specified row height/col width.
    //
    // It is an error for both num_rows() and row_height() to return None.
    // Likewise for num_cols() and col_width().
    //
    // Defaults to None.
    fn num_rows(&self) -> Option<usize> {
        None
    }
    fn num_cols(&self) -> Option<usize> {
        None
    }

    // Returns the height/width for rows/cols.
    // See num_rows() for discussion of returning None.
    //
    // Defaults to None.
    fn row_height(&self) -> Option<Unit> {
        None
    }
    fn col_width(&self) -> Option<Unit> {
        None
    }

    // Width(height) of the row(column) label. If None, labels are not rendered.
    // Defaults to None.
    fn row_label_width(&self) -> Option<Unit> {
        None
    }
    fn col_label_height(&self) -> Option<Unit> {
        None
    }

    // Text for the row(col) label.
    // index will always be < num_rows(num_cols)
    fn row_label(&self, _index: usize) -> Cow<'static, str> {
        "".into()
    }
    fn col_label(&self, _index: usize) -> Cow<'static, str> {
        "".into()
    }

    // Line width of the grid like _before_ the indexed row(col).
    // index will be 0..=num_rows(num_cols). If index == num_rows(num_cols),
    // it is the final line _after_ the last row(col).
    fn horiz_line_width(&self, _index: usize) -> f64{ 1.0 }
    fn vert_line_width(&self, _index: usize) -> f64 { 1.0 }

    // Font to use for labels.
    // TODO: this is a leaky abstraction, since it relies on PDF print stuff.
    // TODO: allow returning a font size
    // TODO: allow returning a different font in different parts of the grid.
    fn font(&self) -> &IndirectFontRef;
}
