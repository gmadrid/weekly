use argh::FromArgs;
use chrono::{Datelike, NaiveDate, Weekday};
use printpdf::{PdfDocumentReference};
use std::borrow::Cow;
use std::path::PathBuf;
use weekly::{Attributes, ColorProxy};
use weekly::{
    save_one_page_document, sizes, AsPdfLine, Datetools, LineModifiers, NumericUnit,
    Result, TGrid, Unit, WRect,
};
use weekly::{GridDescription, Instructions};

#[derive(Debug, FromArgs)]
/// Generates a daily checklist for every date supplied.
struct Args {
    /// month for which to generate the checklist
    #[argh(positional)]
    dates: Vec<NaiveDate>,
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

    fn weekdays_only() -> HashSet<Weekday> {
        vec![Mon, Tue, Wed, Thu, Fri].into_iter().collect()
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
                    name: "Walk",
                    days: None,
                },
                DailyTask {
                    name: "",
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
                    name: "Virtuemap",
                    days: None,
                },
                DailyTask {
                    name: "Add item to bucket list",
                    days: None,
                },
                DailyTask {
                    name: "",
                    days: None,
                },
                DailyTask {
                    name: "",
                    days: None,
                },
                DailyTask {
                    name: "Check calendar",
                    days: None,
                },
                DailyTask {
                    name: "Check ToDo list",
                    days: None,
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
                    name: "",
                    days: None,
                },
                DailyTask {
                    name: "",
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
                    name: "Chess",
                    days: None,
                },
                DailyTask {
                    name: "",
                    days: None,
                },
                DailyTask {
                    name: "",
                    days: None,
                },
                DailyTask {
                    name: "Bug sweep",
                    days: Some(weekdays_only()),
                },
                DailyTask {
                    name: "Code reviews",
                    days: Some(weekdays_only()),
                },
                DailyTask {
                    name: "Inbox Zero",
                    days: Some(weekdays_only()),
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

    fn column_background(&self, index: usize) -> Option<ColorProxy> {
        if index % 2 == 0 {
            Some(ColorProxy::gray(0.9))
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
                    instructions.set_fill_color(ColorProxy::gray(0.7));
                    instructions.push_shape(cell_rect.as_pdf_line());
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
    instructions.set_stroke_color(ColorProxy::gray(0.25));
    instructions.set_stroke_width(0.0);

    instructions.push_shape(checkbox_rect.as_pdf_line().fill(false).stroke(true));
}

fn render_dailies(
    date: &NaiveDate,
    _: &PdfDocumentReference,
    page_rect: &WRect,
) -> weekly::Result<Instructions> {
    let grid_rect =
        page_rect.inset_all_q1(0.5.inches(), 0.25.inches(), 0.25.inches(), 0.25.inches());
    let description = DailyDescription::for_month(date, grid_rect);
    let grid = TGrid::with_description(description);
    Ok(grid.generate_instructions())
}

fn default_output_filename(date: &NaiveDate) -> PathBuf {
    format!("daily_checklist_{}.pdf", date.format("%Y-%m")).into()
}

fn default_doc_title(date: &NaiveDate) -> String {
    format!("Daily Checklist - {}", date.format("%B %Y"))
}

fn main_func(date: &NaiveDate) -> Result<()> {
    let output_filename = default_output_filename(date);
    let doc_title = default_doc_title(date);

    save_one_page_document(&doc_title, &output_filename, &sizes::letter(), |d, p| {
        render_dailies(date, d, p)
    })
}

fn main() {
    let args: Args = argh::from_env();

    if args.dates.is_empty() {
        if let Err(err) = main_func(&weekly::today()) {
            eprintln!("Error: {:?}", err);
        }
    } else {
        for date in &args.dates {
            if let Err(err) = main_func(date) {
                eprintln!("Error: {} : {:?}", date.format("%Y-%m"), err);
            }
        }
    }
}
