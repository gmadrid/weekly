use argh::FromArgs;
use chrono::{Datelike, NaiveDate, Weekday};
use printpdf::{Color, PdfDocumentReference};
use std::borrow::Cow;
use std::path::PathBuf;
use weekly::{
    save_one_page_document, sizes, Attributes, Colors, Datetools, HasRenderAttrs, NumericUnit,
    Result, TGrid, Unit, WRect,
};
use weekly::{GridDescription, Instructions};

#[derive(Debug, FromArgs)]
/// Generates a daily checklist for every date supplied.
struct Args {
    /// month for which to generate the checklist
    #[argh(positional)]
    dates: Vec<NaiveDate>,

    // TODO: try to unify args across apps.
    // TODO: these arguments are kind of ugly. Fix them.
    /// optional start date
    #[argh(option, short = 's')]
    start_date: Option<NaiveDate>,

    /// optional end date
    #[argh(option, short = 'e')]
    end_date: Option<NaiveDate>,
}

mod data {
    use chrono::Weekday;
    use chrono::Weekday::{Fri, Mon, Thu, Tue, Wed};
    use lazy_static::lazy_static;
    use std::collections::HashSet;

    #[derive(Default, Debug)]
    pub struct DailyTask<'a> {
        pub name: &'a str,
        // No set means ALL days. Empty set means NO days.
        pub days: Option<HashSet<Weekday>>,
    }

    fn some_days<const N: usize>(days: [Weekday; N]) -> Option<HashSet<Weekday>> {
        Some(days.iter().copied().collect())
    }
    fn one_day(day: Weekday) -> Option<HashSet<Weekday>> {
        some_days([day])
    }
    fn weekdays_only() -> Option<HashSet<Weekday>> {
        some_days([Mon, Tue, Wed, Thu, Fri])
    }

    lazy_static! {
        pub static ref TASKS: Vec<DailyTask<'static>> = {
            vec![
                DailyTask {
                    name: "Plank",
                    days: None,
                },
                DailyTask {
                    name: "Door stretch",
                    days: None,
                },
                DailyTask {
                    name: "Stretch",
                    days: None,
                },
                DailyTask {
                    name: "Workout",
                    days: some_days([Weekday::Mon, Weekday::Wed, Weekday::Fri]),
                },
                DailyTask {
                    name: "Weekly review",
                    days: one_day(Weekday::Sun),
                },
                DailyTask {
                    name: "",
                    days: None,
                },
                DailyTask {
                    name: "Brush teeth",
                    days: None,
                },
                DailyTask {
                    name: "Floss",
                    days: None,
                },
                DailyTask {
                    name: "Mouthwash",
                    days: None,
                },
                DailyTask {
                    name: "Feet and nails",
                    days: None,
                },
                DailyTask {
                    name: "Drugs",
                    days: None,
                },
                DailyTask {
                    name: "Face (am)",
                    days: None,
                },
                DailyTask {
                    name: "Face (pm)",
                    days: None,
                },
                DailyTask {
                    name: "Clean food",
                    days: None,
                },
                DailyTask {
                    name: "",
                    days: None,
                },
                DailyTask {
                    name: "Journal",
                    days: None,
                },
                DailyTask {
                    name: "Knit",
                    days: None,
                },
                DailyTask {
                    name: "Magic",
                    days: None,
                },
                DailyTask {
                    name: "Lone Wolf & Cub",
                    days: None,
                },
                DailyTask {
                    name: "Read",
                    days: None,
                },
                DailyTask {
                    name: "Bug sweep",
                    days: weekdays_only(),
                },
                DailyTask {
                    name: "Code reviews",
                    days: weekdays_only(),
                },
                DailyTask {
                    name: "Inbox Zero",
                    days: weekdays_only(),
                },
                DailyTask {
                    name: "Check calendar",
                    days: None,
                },
                DailyTask {
                    name: "GTD",
                    days: None,
                },
            ]
        };
    }
}

struct DailyDescription {
    bounds: WRect,
    dates_in_month: Vec<NaiveDate>,
}

impl DailyDescription {
    const NUM_COLS: usize = 25;

    pub fn for_month<DL>(date: &DL, bounds: WRect) -> DailyDescription
    where
        DL: Datelike,
    {
        DailyDescription {
            bounds,
            dates_in_month: date.dates_in_month(),
        }
    }
}

impl GridDescription for DailyDescription {
    fn bounds(&self) -> WRect {
        self.bounds.clone()
    }

    fn num_rows(&self) -> Option<usize> {
        Some(self.dates_in_month.len())
    }

    fn num_cols(&self) -> Option<usize> {
        Some(Self::NUM_COLS)
    }

    fn row_label_width(&self) -> Option<Unit> {
        Some(1.0.inches())
    }

    fn col_label_height(&self) -> Option<Unit> {
        Some(2.0.inches())
    }

    fn row_label(&self, index: usize) -> Cow<str> {
        self.dates_in_month[index]
            .format("%b %e")
            .to_string()
            .into()
    }

    fn col_label(&self, index: usize) -> Cow<str> {
        if index < data::TASKS.len() {
            data::TASKS[index].name.into()
        } else {
            "".into()
        }
    }

    fn horiz_line_style(&self, row: usize, _num_rows: usize) -> Option<Attributes> {
        let attrs = Attributes::default();
        if row < self.dates_in_month.len() && self.dates_in_month[row].weekday() == Weekday::Sun {
            Some(attrs)
        } else {
            Some(attrs.with_stroke_width(0.0))
        }
    }

    fn vert_line_style(&self, col: usize, _num_cols: usize) -> Option<Attributes> {
        let attrs = Attributes::default();
        if col > 0 && col < Self::NUM_COLS && col % 5 == 0 {
            Some(attrs)
        } else {
            Some(attrs.with_stroke_width(0.0))
        }
    }

    fn column_background(&self, index: usize) -> Option<Color> {
        if index % 2 == 0 {
            Some(Colors::gray(0.9))
        } else {
            None
        }
    }

    fn render_cell_contents(
        &self,
        row: usize,
        col: usize,
        cell_rect: &WRect,
        instructions: &mut Instructions,
    ) {
        let mut should_draw_checkbox = true;
        if col < data::TASKS.len() {
            if let Some(day_set) = &data::TASKS[col].days {
                let date = &self.dates_in_month[row];
                if !day_set.contains(&date.weekday()) {
                    instructions.set_fill_color(Colors::gray(0.7));
                    // TODO: can we get rid of this clone()?
                    instructions.push_shape(cell_rect.clone().fill());
                    should_draw_checkbox = false;
                }
            }
        }

        if should_draw_checkbox {
            render_checkbox(cell_rect, instructions);
        }
    }
}

fn render_checkbox(cell_rect: &WRect, instructions: &mut Instructions) {
    let box_width = 3.0.mm();

    let x_offset = (cell_rect.width() - box_width) / 2;
    let y_offset = (cell_rect.height() - box_width) / 2;

    let checkbox_rect = WRect::with_dimensions(box_width, box_width)
        .move_to(cell_rect.left() + x_offset, cell_rect.top() - y_offset);

    instructions.clear_fill_color();
    instructions.set_stroke_color(Colors::gray(0.25));
    instructions.set_stroke_width(0.0);

    instructions.push_shape(checkbox_rect.stroke());
}

fn render_dailies(
    date: &NaiveDate,
    end_date: &Option<NaiveDate>,
    _: &PdfDocumentReference,
    page_rect: &WRect,
) -> weekly::Result<Instructions> {
    let grid_rect =
        page_rect.inset_all_q1(0.25.inches(), 0.25.inches(), 0.6.inches(), 0.25.inches());
    let description = if let Some(end) = end_date {
        DailyDescription {
            bounds: grid_rect,
            dates_in_month: date.date_range((*end - *date).num_days()),
        }
    } else {
        DailyDescription::for_month(date, grid_rect)
    };
    let grid = TGrid::with_description(description);
    Ok(grid.generate_instructions())
}

fn default_output_filename(date: &NaiveDate) -> PathBuf {
    format!("daily_checklist_{}.pdf", date.format("%Y-%m")).into()
}

fn default_doc_title(date: &NaiveDate) -> String {
    format!("Daily Checklist - {}", date.format("%B %Y"))
}

fn main_func(date: &NaiveDate, end: &Option<NaiveDate>) -> Result<()> {
    let output_filename = default_output_filename(date);
    let doc_title = default_doc_title(date);

    save_one_page_document(&doc_title, output_filename, &sizes::letter(), |d, p| {
        render_dailies(date, end, d, p)
    })
}

fn main() {
    let args: Args = argh::from_env();

    if args.start_date.is_some() && args.end_date.is_some() {
        if let Err(err) = main_func(&args.start_date.unwrap(), &args.end_date) {
            eprintln!("Error: {:?}", err);
        }
    } else if args.dates.is_empty() {
        if let Err(err) = main_func(&weekly::today(), &None) {
            eprintln!("Error: {:?}", err);
        }
    } else {
        for date in &args.dates {
            if let Err(err) = main_func(date, &None) {
                eprintln!("Error: {} : {:?}", date.format("%Y-%m"), err);
            }
        }
    }
}
