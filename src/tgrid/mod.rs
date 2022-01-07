use crate::{AsPdfLine, Instructions, NumericUnit, Unit, WLine, WRect};
use printpdf::IndirectFontRef;
use std::borrow::Cow;

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

    // Font to use for labels.
    // TODO: this is a leaky abstraction, since it relies on PDF print stuff.
    fn font(&self) -> &IndirectFontRef;
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
        if description.row_height().is_none() && description.num_rows().is_none() {
            // TODO: make this return a Result
            panic!("either row height or num rows must be set");
        }
        if description.col_width().is_none() && description.num_cols().is_none() {
            // TODO: make this return a Result
            panic!("either col width or num cols must be set");
        }

        // TODO: account for row labels and col labels here.
        let num_rows = description.num_rows().unwrap_or_else(|| {
            // unwrap: we check that both num_rows and row_height cannot be none.
            (description.bounds().height() - description.col_label_height().unwrap_or(Unit::zero()))
                / description.row_height().unwrap()
        });
        let num_cols = description.num_cols().unwrap_or_else(|| {
            // unwrap: we check that both num_cols and col_width cannot be none.
            (description.bounds().width() - description.row_label_width().unwrap_or(Unit::zero()))
                / description.col_width().unwrap()
        });

        let row_height = description.row_height().unwrap_or_else(|| {
            // unwrap: we check that both num_rows and row_height cannot be none.
            (description.bounds().height() - description.col_label_height().unwrap_or(Unit::zero()))
                / description.num_rows().unwrap()
        });
        let col_width = description.col_width().unwrap_or_else(|| {
            // unwrap: we check that both num_cols and col_width cannot be none.
            (description.bounds().width() - description.row_label_width().unwrap_or(Unit::zero()))
                / description.num_cols().unwrap()
        });

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

    fn render_horizontal_lines(&self, computed: &Computed, instructions: &mut Instructions) {
        let left = self.description.bounds().left();
        let right = left
            + self.description.row_label_width().unwrap_or(Unit::zero())
            + computed.col_width * computed.num_cols;
        let top = self.description.bounds().top()
            - self.description.col_label_height().unwrap_or(Unit::zero());
        for row in 0..(computed.num_rows + 1) {
            let y = top - computed.row_height * row;
            instructions.push_shape(WLine::line(left, y, right, y).as_pdf_line())
        }
    }

    fn render_vertical_lines(&self, computed: &Computed, instructions: &mut Instructions) {
        let top = self.description.bounds().top();
        let bottom = top
            - self.description.col_label_height().unwrap_or(Unit::zero())
            - computed.row_height * computed.num_rows;
        let left = self.description.bounds().left()
            + self.description.row_label_width().unwrap_or(Unit::zero());
        for col in 0..(computed.num_cols + 1) {
            let x = left + computed.col_width * col;
            instructions.push_shape(WLine::line(x, top, x, bottom).as_pdf_line())
        }
    }

    fn row_y(&self, row: usize, computed: &Computed) -> Unit {
        self.description.bounds().top()
            - self.description.col_label_height().unwrap_or(Unit::zero())
            - computed.row_height * row
    }

    fn col_x(&self, col: usize, computed: &Computed) -> Unit {
        self.description.bounds().left() + self.description.row_label_width().unwrap_or(Unit::zero()) + computed.col_width * col
    }

    fn render_row_labels(&self, computed: &Computed, instructions: &mut Instructions) {
        let row_height = computed.row_height;

        let x = self.description.bounds().left() + 2.0.mm();
        let text_height = f64::from(row_height) * 1.9;
        for row in 0..computed.num_rows {
            let y = self.row_y(row + 1, computed) + 1.5.mm();
            instructions.push_text(
                self.description.row_label(row).as_ref(),
                text_height,
                x,
                y,
                self.description.font(),
            );
        }
    }

    fn render_col_labels(&self, computed: &Computed, instructions: &mut Instructions) {
        // This is DRY
        let row_height = computed.row_height;

        // (159, -21) after rotation.
        let text_height = f64::from(row_height) * 1.9;
        let y = self.description.bounds().top() - self.description.col_label_height().unwrap_or(Unit::zero()) + 1.0.mm();
        for col in 0..computed.num_cols {
            let x = self.col_x(col + 1, computed) - 1.0.mm();

            instructions.push_state();
            instructions.rotate(90.0);
            instructions.translate(y, -x);

            // Text position is (0.0), so that we can rotate the text before translating it.
            instructions.push_text(
                self.description.col_label(col).as_ref(),
                text_height,
                Unit::zero(),
                Unit::zero(),
                self.description.font(),
            );
            instructions.pop_state();
        }
    }

    pub fn generate_instructions(&self) -> Instructions {
        let mut instructions = Instructions::default();
        let computed = Computed::compute(self.description);

        self.render_horizontal_lines(&computed, &mut instructions);
        self.render_vertical_lines(&computed, &mut instructions);

        self.render_row_labels(&computed, &mut instructions);
        self.render_col_labels(&computed, &mut instructions);

        instructions
    }
}
