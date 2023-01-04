use argh::FromArgs;
use printpdf::PdfDocumentReference;
use weekly::{save_one_page_document, sizes, AsPdfLine, Colors, Instructions, LineModifiers, NumericUnit, Result, WRect, GridDescription, TGrid, FontProxy};

const GOLDEN_RATIO: f64 = 1.618033988749894;

#[derive(Debug, FromArgs)]
/// Generates a daily checklist for every date supplied.
struct Args {
    /// name of the output file
    #[argh(positional)]
    output_filename: String,
}

#[derive(Debug)]
struct SimpleDescription {
    rect: WRect,
    num_rows: usize,
    text: String,
}

impl SimpleDescription {
    fn new<T>(rect: &WRect, num_rows: usize, text: T) -> Self where T: Into<String> {
        SimpleDescription {
            rect: rect.clone(),
            num_rows,
            text: text.into(),
        }
    }
}

impl GridDescription for SimpleDescription {
    fn bounds(&self) -> WRect {
        self.rect.clone()
    }

    fn num_rows(&self) -> Option<usize> {
        Some(self.num_rows)
    }

    fn num_cols(&self) -> Option<usize> {
        Some(1)
    }

    fn render_cell_contents(&self, row: usize, _col: usize, cell_rect: &WRect, instructions: &mut Instructions) {
        instructions.push_state();

        if row == 0 {
            instructions.set_fill_color(Colors::black());
            instructions.push_shape(cell_rect.as_pdf_line());

            instructions.set_fill_color(Colors::white());
            instructions.push_text(&self.text, ((cell_rect.height() - 1.0.mm()) * 1.9).into(),
                                   cell_rect.left() + 1.0.mm(), cell_rect.bottom_q1() + 1.5.mm(), FontProxy::Helvetica(true, false))
        }

        instructions.pop_state();
    }
}

// Number of lines in the top table + 1 to account for gutter.
const TOTAL_TOP_LINES: f64 = 9.0;

fn render_lines<T: Into<String>>(rect: &WRect, text: T) -> Result<Instructions> {
    let mut instructions = Instructions::default();
    let line_space = rect.height() / TOTAL_TOP_LINES;

    let table_rect = rect.resize(rect.width(), rect.height() - line_space);
    instructions.push_shape(table_rect.as_pdf_line().fill(false).stroke(true));

    let description = SimpleDescription::new(&table_rect, 8, text.into());
    let tgrid = TGrid::with_description(description);

    tgrid.append_to_instructions(&mut instructions);

    Ok(instructions)
}

fn render_weekly(_: &PdfDocumentReference, page_rect: &WRect) -> Result<Instructions> {
    let mut instructions = Instructions::default();

    instructions.set_stroke_color(Colors::gray(0.66));
    instructions.set_stroke_width(1.0);
    instructions.clear_fill_color();

    let print_rect =
        page_rect.inset_all_q1(0.25.inches(), 0.25.inches(), 0.25.inches(), 0.25.inches());

    // Make the top and bottom halves in the golden ratio.
    // So, x + Phi.x = page height
    let top_height = print_rect.height() / (1.0 + GOLDEN_RATIO);
    let bottom_height = print_rect.height() - top_height;

    // Grid is based around a 5-column grid.
    let grid_x = print_rect.width() / 5.0;

    let priorities_rect = print_rect.resize(grid_x * 2.0, top_height);
    instructions.append(render_lines(&priorities_rect, "Weekly Priorities")?);

    let tracker_rect = priorities_rect.move_by(grid_x * 2.0, 0.0.into());
    instructions.append(render_lines(&tracker_rect, "Habit Tracker")?);

    let weekend_rect = tracker_rect
        .move_by(grid_x * 2.0, 0.0.into())
        .resize(grid_x, priorities_rect.height())
        ;
    instructions.append(render_lines(&weekend_rect, "Weekend Plans")?);

    let calendar_rect = print_rect.resize(print_rect.width(), bottom_height)
        .move_by(0.0.into(), -top_height);
    instructions.push_shape(calendar_rect.as_pdf_line().fill(false).stroke(true));

    Ok(instructions)
}

pub fn main() -> Result<()> {
    let args: Args = argh::from_env();

    // half letter = 215.9 x 139.7mm
    // Inset rect should be: w: 203.2 h: 127 l: 6.35 t: 133.35

    let page_rect = sizes::halfletter();

    save_one_page_document(
        &args.output_filename,
        &args.output_filename,
        &page_rect,
        |doc, page_rect| render_weekly(doc, page_rect),
    )
}
