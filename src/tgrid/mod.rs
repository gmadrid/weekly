use crate::{AsPdfLine, Instructions, NumericUnit, Unit, WLine, WRect};

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
    // Defaults to 1/4" high, 1/2" wide.
    fn row_height(&self) -> Option<Unit> {
        Some(0.25.inches())
    }
    fn col_width(&self) -> Option<Unit> {
        Some(0.5.inches())
    }
}

#[derive(Debug)]
struct Computed {
    row_height: Unit,
    col_width: Unit,
    num_rows: usize,
    num_cols: usize,
}

impl Computed {
    fn compute<D>(description: &D) -> Computed
    where
        D: GridDescription,
    {
        let row_height = 1.0.inches();
        let col_width = 2.0.inches();
        let num_rows = 5;
        let num_cols = 3;

        Computed {
            row_height,
            col_width,
            num_rows,
            num_cols,
        }
    }
}

pub struct TGrid<'a, D>
where
    D: GridDescription,
{
    description: &'a D,
}

impl<'a, D> TGrid<'a, D>
where
    D: GridDescription,
{
    pub fn with_description(description: &D) -> TGrid<D> {
        TGrid { description }
    }

    fn render_row_lines(&self, computed: &Computed, instructions: &mut Instructions) {
        for row in 0..computed.num_rows {
            instructions.push_shape(
                WLine::line(1.0.inches(), 1.0.inches() * row, 6.0.inches(), 1.0.inches() * row).as_pdf_line()
            )
        }
    }

    pub fn generate_instructions(&self) -> Instructions {
        let mut instructions = Instructions::default();

        let computed = Computed::compute(self.description);

        self.render_row_lines(&computed, &mut instructions);

        instructions.push_shape(
            WLine::line(0.25.inches(), 0.5.inches(), 3.0.inches(), 5.0.inches()).as_pdf_line(),
        );

        instructions
    }
}
