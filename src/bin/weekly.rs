use argh::FromArgs;
use printpdf::PdfDocumentReference;
use weekly::{
    save_one_page_document, sizes, Attributes, Circle, Colors, GridDescription, Instructions,
    LineModifiers, NumericUnit, Result, TGrid, TextContext, ToPdfLine, Unit, WLine, WRect,
};

const GOLDEN_RATIO: f64 = 1.618033988749894;

const DAY_ABBREVS: [&str; 5] = ["Mon", "Tue", "Wed", "Thu", "Fri"];

const DAY_LETTERS: [&str; 7] = ["M", "T", "W", "T", "F", "S", "S"];

const HABITS: [&str; 7] = [
    "Check calendar",
    "Inbox Zero",
    "Code reviews",
    "Bug sweep",
    "GTD",
    "",
    "Release tasks",
];

// Number of lines in the top table + 1 to account for gutter.
const TOTAL_TOP_LINES: f64 = 9.0;

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
    text_context: TextContext,

    render_func: F,
    offset: Unit,
}

impl<F: Fn(&WRect, usize, &mut Instructions)> SimpleDescription<F> {
    fn new<T>(
        rect: &WRect,
        num_rows: usize,
        text: T,
        text_context: &TextContext,
        render_func: F,
    ) -> Self
    where
        T: Into<String>,
    {
        SimpleDescription {
            rect: rect.clone(),
            num_rows,
            text: text.into(),
            text_context: text_context.clone(),
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
            instructions.push_shape(cell_rect.to_filled_line());

            instructions.set_fill_color(Colors::white());
            self.text_context
                .bold(true)
                .with_text_height((cell_rect.height() - 1.0.mm()) * 1.9)
                .render(
                    &self.text,
                    cell_rect.left() + self.offset,
                    cell_rect.bottom_q1() + 1.5.mm(),
                    instructions,
                );
        }

        (self.render_func)(cell_rect, row, instructions);

        instructions.pop_state();
    }
}

fn render_lines<T: AsRef<str>, F: Fn(&WRect, usize, &mut Instructions)>(
    rect: &WRect,
    text: T,
    text_context: &TextContext,
    num_rows: usize,
    offset: Unit,
    render_func: F,
    instructions: &mut Instructions,
) {
    let line_space = rect.height() / TOTAL_TOP_LINES;

    let table_rect = rect.resize(rect.width(), rect.height() - line_space);

    let description = SimpleDescription::new(
        &table_rect,
        num_rows,
        text.as_ref(),
        text_context,
        render_func,
    )
    .set_offset(offset);
    let tgrid = TGrid::with_description(description);

    instructions.push_shape(table_rect.to_stroked_line());

    tgrid.append_to_instructions(instructions);
}

fn render_left_circle(rect: &WRect, instructions: &mut Instructions) {
    let radius = rect.height() / 2.0;
    let x = rect.left() + radius;
    let y = rect.bottom_q1() + radius;

    let circle = Circle::at_zero(radius - 1.15.mm()).move_to(x, y);
    instructions.push_shape(circle);
}

fn render_days(rect: &WRect, text_context: &TextContext, instructions: &mut Instructions) {
    let day_width = rect.width() / DAY_ABBREVS.len() as f64;

    let day_rect = rect.resize(day_width, rect.height());
    for (i, abbrev) in DAY_ABBREVS.iter().enumerate() {
        render_lines(
            &day_rect.move_by(day_width * i as f64, 0.0.mm()),
            abbrev,
            text_context,
            14,
            12.0.mm(),
            |rect, idx, instructions| {
                if idx == 0 {
                    let radius = rect.height() / 2.0 + 1.5.mm();
                    instructions.push_state();
                    instructions.set_fill_color(Colors::white());
                    instructions.set_stroke_color(Colors::gray(0.6));
                    instructions.set_stroke_width(1.0);

                    instructions.push_shape(
                        Circle::at_zero(radius)
                            .move_to(
                                rect.left() + radius + 2.0.mm(),
                                rect.bottom_q1() + radius / 2.0 + 0.8.mm(),
                            )
                            .to_filled_line()
                            .stroke(true),
                    );
                    instructions.pop_state();
                }
            },
            instructions,
        );
    }
}

fn render_weekly(
    _: &PdfDocumentReference,
    page_rect: &WRect,
    text_context: &TextContext,
    instructions: &mut Instructions,
) {
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
    render_priorities(
        &priorities_rect,
        top_text_offset,
        text_context,
        instructions,
    );

    let tracker_rect = priorities_rect.move_by(grid_x * 2.0, Unit::zero());
    render_tracker(&tracker_rect, top_text_offset, text_context, instructions);

    let weekend_rect = tracker_rect
        .move_by(grid_x * 2.0, Unit::zero())
        .resize(grid_x, priorities_rect.height());
    render_weekend(&weekend_rect, top_text_offset, text_context, instructions);

    let calendar_rect = print_rect
        .resize(print_rect.width(), bottom_height)
        .move_by(Unit::zero(), -top_height);
    render_days(&calendar_rect, text_context, instructions);
}

fn render_weekend(
    weekend_rect: &WRect,
    top_text_offset: Unit,
    text_context: &TextContext,
    instructions: &mut Instructions,
) {
    render_lines(
        weekend_rect,
        "Weekend Plans",
        text_context,
        8,
        top_text_offset,
        |_, _, _| {},
        instructions,
    );
}

fn render_tracker(
    tracker_rect: &WRect,
    top_text_offset: Unit,
    text_context: &TextContext,
    instructions: &mut Instructions,
) {
    render_lines(
        tracker_rect,
        "Habit Tracker",
        text_context,
        8,
        top_text_offset,
        |rect, row, instructions| {
            let text_context = text_context.with_text_height((rect.height() - 1.0.mm()) * 1.9);

            let small_grid_left = rect.right() - rect.height() * 7.0;
            if row > 0 {
                instructions.push_state();
                instructions.set_stroke_color(Colors::gray(0.75));
                instructions.set_dash(1, 1);
                for i in 0..7 {
                    let l = small_grid_left + rect.height() * i;
                    let wline = WLine::line(l, rect.bottom_q1(), l, rect.top());
                    instructions.push_shape(wline)
                }
                instructions.pop_state();
            } else {
                // Top row labels
                instructions.push_state();
                instructions.set_fill_color(Colors::white());
                let bold_context = text_context.bold(true);
                for (i, letter) in DAY_LETTERS.iter().enumerate() {
                    let l = small_grid_left + rect.height() * i as f64;
                    bold_context.render(
                        letter,
                        l + 1.4.mm(),
                        rect.bottom_q1() + 1.5.mm(),
                        instructions,
                    );
                }

                instructions.pop_state();
            }

            if row > 0 && row < HABITS.len() + 1 {
                instructions.push_state();
                instructions.set_fill_color(Colors::black());
                text_context.render(
                    HABITS[row - 1],
                    rect.left() + 1.5.mm(),
                    rect.bottom_q1() + 1.5.mm(),
                    instructions,
                );
                instructions.pop_state();
            }
        },
        instructions,
    );
}

fn render_priorities(
    priorities_rect: &WRect,
    top_text_offset: Unit,
    text_context: &TextContext,
    instructions: &mut Instructions,
) {
    render_lines(
        priorities_rect,
        "Weekly Priorities",
        text_context,
        8,
        top_text_offset,
        |rect, row, instructions| {
            if row > 0 {
                render_left_circle(rect, instructions)
            }
        },
        instructions,
    );
}

fn render_dotted(_: &PdfDocumentReference, dotted_rect: &WRect, instructions: &mut Instructions) {
    instructions.push_state();

    instructions.clear_fill_color();
    instructions.set_stroke_color(Colors::gray(0.7));
    instructions.set_stroke_width(0.5);
    instructions.push_shape(
        dotted_rect
            .as_rounded_rect_shape(2.0.mm())
            .fill(false)
            .stroke(true),
    );

    instructions.set_fill_color(Colors::gray(0.7));
    let grid_spacing = 0.25.inches();

    let base_circle = Circle::at_zero(0.25.mm());
    let mut x = dotted_rect.left() + grid_spacing;
    while x <= dotted_rect.right() - grid_spacing {
        let mut y = dotted_rect.top() - grid_spacing;

        while y >= dotted_rect.bottom_q1() + grid_spacing {
            instructions.push_shape(base_circle.move_to(x, y).to_filled_line());

            y = y - grid_spacing;
        }
        x = x + grid_spacing;
    }
    instructions.pop_state();
}

fn render_weekly_page(doc: &PdfDocumentReference, page_rect: &WRect) -> Result<Instructions> {
    let mut instructions = Instructions::default();
    let top_half = page_rect.resize(page_rect.width(), page_rect.height() / 2.0);
    let text_context = TextContext::helvetica();
    render_weekly(doc, &top_half, &text_context, &mut instructions);

    let bottom_half = top_half
        .move_by(Unit::zero(), -top_half.height())
        .inset_all_q1(0.25.inches(), 0.25.inches(), 0.25.inches(), 0.25.inches());
    render_dotted(doc, &bottom_half, &mut instructions);

    Ok(instructions)
}

pub fn main() -> Result<()> {
    let args: Args = argh::from_env();

    let page_rect = sizes::letter();

    save_one_page_document(
        "Productivity Tracker",
        args.output_filename,
        &page_rect,
        render_weekly_page,
    )
}
