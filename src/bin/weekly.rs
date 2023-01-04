use argh::FromArgs;
use printpdf::{Line, PdfDocumentReference};
use weekly::{
    save_one_page_document, sizes, AsPdfLine, Attributes, Colors, FontProxy, GridDescription,
    Instructions, LineModifiers, NumericUnit, Result, TGrid, Unit, WLine, WRect,
};

const GOLDEN_RATIO: f64 = 1.618033988749894;

#[derive(Debug, FromArgs)]
/// Generates a weekly productivity tracker.
struct Args {
    /// name of the output file
    #[argh(positional, default = "String::from(\"weekly.pdf\")")]
    output_filename: String,
}

#[derive(Debug)]
struct SimpleDescription<F>
where
    F: Fn(&WRect, usize, &mut Instructions),
{
    rect: WRect,
    num_rows: usize,
    text: String,

    render_func: F,
    offset: Unit,
}

impl<F: Fn(&WRect, usize, &mut Instructions)> SimpleDescription<F> {
    fn new<T>(rect: &WRect, num_rows: usize, text: T, render_func: F) -> Self
    where
        T: Into<String>,
    {
        SimpleDescription {
            rect: rect.clone(),
            num_rows,
            text: text.into(),
            render_func,
            offset: 1.0.mm(),
        }
    }

    fn set_offset(mut self, offset: Unit) -> Self {
        self.offset = offset;
        self
    }
}

impl<F: Fn(&WRect, usize, &mut Instructions)> GridDescription for SimpleDescription<F> {
    fn bounds(&self) -> WRect {
        self.rect.clone()
    }

    fn num_rows(&self) -> Option<usize> {
        Some(self.num_rows)
    }

    fn num_cols(&self) -> Option<usize> {
        Some(1)
    }

    fn horiz_line_style(&self, index: usize, num_rows: usize) -> Option<Attributes> {
        let base = Attributes::default().with_stroke_color(&Colors::gray(0.75));
        if index <= 1 {
            None // Some(base.with_stroke_color(&Colors::black()))
        } else if index < num_rows {
            Some(base.with_dash(1, 1))
        } else {
            Some(base)
        }
    }

    fn render_cell_contents(
        &self,
        row: usize,
        _col: usize,
        cell_rect: &WRect,
        instructions: &mut Instructions,
    ) {
        instructions.push_state();

        if row == 0 {
            instructions.set_fill_color(Colors::black());
            instructions.push_shape(cell_rect.as_pdf_line());

            instructions.set_fill_color(Colors::white());
            instructions.push_text(
                &self.text,
                ((cell_rect.height() - 1.0.mm()) * 1.9).into(),
                cell_rect.left() + self.offset,
                cell_rect.bottom_q1() + 1.5.mm(),
                FontProxy::Helvetica(true, false),
            )
        }

        (self.render_func)(cell_rect, row, instructions);

        instructions.pop_state();
    }
}

// Number of lines in the top table + 1 to account for gutter.
const TOTAL_TOP_LINES: f64 = 9.0;

fn render_lines<T: Into<String>, F: Fn(&WRect, usize, &mut Instructions)>(
    rect: &WRect,
    text: T,
    num_rows: usize,
    offset: Unit,
    render_func: F,
) -> Result<Instructions> {
    let mut instructions = Instructions::default();
    let line_space = rect.height() / TOTAL_TOP_LINES;

    let table_rect = rect.resize(rect.width(), rect.height() - line_space);
    instructions.push_shape(table_rect.as_pdf_line().fill(false).stroke(true));

    let description =
        SimpleDescription::new(&table_rect, num_rows, text.into(), render_func).set_offset(offset);
    let tgrid = TGrid::with_description(description);

    tgrid.append_to_instructions(&mut instructions);

    Ok(instructions)
}

fn render_left_circle(rect: &WRect, instructions: &mut Instructions) {
    let radius = rect.height() / 2.0;

    let points = printpdf::utils::calculate_points_for_circle(
        radius - 1.15,
        rect.left() + radius,
        rect.bottom_q1() + radius,
    );
    let line = Line {
        points,
        is_closed: true,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    instructions.push_shape(line);
}

const DAY_ABBREVS: [&str; 5] = ["Mon", "Tue", "Wed", "Thu", "Fri"];
const DAY_LETTERS: [&str; 7] = ["M", "T", "W", "T", "F", "S", "S"];

fn render_days(rect: &WRect) -> Instructions {
    let day_width = rect.width() / DAY_ABBREVS.len() as f64;
    let mut instructions = Instructions::default();

    let day_rect = rect.resize(day_width, rect.height());
    for i in 0..DAY_ABBREVS.len() {
        instructions.append(render_lines(
            &day_rect.move_by(day_width * i as f64, 0.0.mm()),
            DAY_ABBREVS[i],
            14,
            12.0.mm(),
            |rect, idx, instructions| {
                if idx == 0 {
                    let radius = rect.height() / 2.0 + 1.5.mm();
                    instructions.push_state();
                    instructions.set_fill_color(Colors::white());
                    instructions.set_stroke_color(Colors::gray(0.6));
                    instructions.set_stroke_width(1.0);

                    let points = printpdf::utils::calculate_points_for_circle(
                        radius,
                        rect.left() + radius + 2.0.mm(),
                        rect.bottom_q1() + radius / 2.0 + 0.8.mm(),
                    );
                    let circle = Line {
                        points,
                        is_closed: true,
                        has_fill: true,
                        has_stroke: true,
                        is_clipping_path: false,
                    };
                    instructions.push_shape(circle);

                    instructions.pop_state();
                }
            },
        ).unwrap());
    }

    instructions
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

    let top_text_offset = 5.0.mm();

    let priorities_rect = print_rect.resize(grid_x * 2.0, top_height);
    instructions.append(render_lines(
        &priorities_rect,
        "Weekly Priorities",
        8,
        top_text_offset,
        |rect, row, instructions| {
            if row > 0 {
                render_left_circle(rect, instructions)
            }
        },
    )?);

    let tracker_rect = priorities_rect.move_by(grid_x * 2.0, 0.0.into());
    instructions.append(render_lines(
        &tracker_rect,
        "Habit Tracker",
        8,
        top_text_offset,
        |rect, row, instructions| {
            let small_grid_left = rect.right() - rect.height() * 7.0;
            if row > 0 {
                instructions.push_state();
                instructions.set_dash(1, 1);
                for i in 0..7 {
                    let l = small_grid_left + rect.height() * i;
                    let wline = WLine::line(l, rect.bottom_q1(), l, rect.top());
                    instructions.push_shape(wline.as_pdf_line())
                }
                instructions.pop_state();
            } else {
                // Top row labels
                instructions.push_state();
                instructions.set_fill_color(Colors::white());
                for i in 0..7 {
                    let l = small_grid_left + rect.height() * i;
                    instructions.push_text(
                        DAY_LETTERS[i],
                        ((rect.height() - 1.0.mm()) * 1.9).into(),
                        l + 1.4.mm(),
                        rect.bottom_q1() + 1.5.mm(),
                        FontProxy::Helvetica(true, false),
                    );
                }

                instructions.pop_state();
            }
        },
    )?);

    let weekend_rect = tracker_rect
        .move_by(grid_x * 2.0, 0.0.into())
        .resize(grid_x, priorities_rect.height());
    instructions.append(render_lines(
        &weekend_rect,
        "Weekend Plans",
        8,
        top_text_offset,
        |_, _, _| {},
    )?);

    let calendar_rect = print_rect
        .resize(print_rect.width(), bottom_height)
        .move_by(0.0.into(), -top_height);
    instructions.append(render_days(&calendar_rect));

    Ok(instructions)
}

pub fn main() -> Result<()> {
    let args: Args = argh::from_env();

    let page_rect = sizes::halfletter();

    save_one_page_document(
        "Productivity Tracker",
        &args.output_filename,
        &page_rect,
        |doc, page_rect| render_weekly(doc, page_rect),
    )
}
