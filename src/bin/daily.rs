use argh::FromArgs;
use chrono::Weekday::{Fri, Mon, Thu, Tue, Wed};
use chrono::{Datelike, Duration, NaiveDate, Weekday};
use lazy_static::lazy_static;
use printpdf::{BuiltinFont, PdfDocument};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use weekly::{Builder, Colors, Datetools, NumericUnit, WRect};

const DEFAULT_NUM_COLS: u16 = 25;
const DEFAULT_TOP_LABEL_HEIGHT: f64 = 2.0;

#[derive(Debug, FromArgs)]
/// Generates a daily checklist for every date supplied.
struct Args {
    /// month for which to generate the checklist
    #[argh(positional)]
    date: Vec<NaiveDate>,
}

#[derive(Default, Debug)]
struct DailyTask<'a> {
    name: &'a str,
    // No set means ALL days. Empty set means NO days.
    days: Option<HashSet<Weekday>>,
}

lazy_static! {
    static ref TASKS: Vec<DailyTask<'static>> = {
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

fn weekdays_only() -> HashSet<Weekday> {
    vec![Mon, Tue, Wed, Thu, Fri].into_iter().collect()
}

fn get_date_names(date: &impl Datelike) -> Vec<String> {
    date.dates_in_month()
        .into_iter()
        .map(|d| d.format("%b %e").to_string())
        .collect()
}

fn default_output_filename(date: &NaiveDate) -> PathBuf {
    format!("daily_checklist_{}.pdf", date.format("%Y-%m")).into()
}

fn default_doc_title(date: &NaiveDate) -> String {
    format!("Daily Checklist - {}", date.format("%B %Y"))
}

fn main_func(date: &NaiveDate) -> weekly::Result<()> {
    let date_names = get_date_names(date);

    let col_labels: Vec<&str> = TASKS.iter().map(|t| t.name).collect();

    let page_rect =
        WRect::with_dimensions(5.5.inches(), 8.5.inches()).move_to(0.0.inches(), 8.5.inches());
    let table_bounds = page_rect.inset_all_q1(
        0.5.inches() + 0.125.inches(), // Extra 1/8" for the rings.
        0.25.inches(),
        0.25.inches(),
        0.25.inches(),
    );

    let top_box_height = DEFAULT_TOP_LABEL_HEIGHT.inches();
    let cols = DEFAULT_NUM_COLS;

    let output_filename = default_output_filename(date);
    let doc_title = default_doc_title(date);

    let (doc, page, layer) = PdfDocument::new(
        &doc_title,
        page_rect.width().into(),
        page_rect.height().into(),
        "Layer 1",
    );
    let times_bold = doc.add_builtin_font(BuiltinFont::TimesBold).unwrap();

    let first = date.first_of_month();
    let horiz_line_width_func = |row: usize| {
        let date = first + Duration::days(row as i64);
        if date.weekday() == Weekday::Sun {
            1.0
        } else {
            0.0
        }
    };
    let vert_line_width_func = |col: usize| {
        if col > 0 && col % 5 == 0 {
            1.0
        } else {
            0.0
        }
    };
    let cell_background_func = |row: usize, col: usize| {
        if col < TASKS.len() {
            if let Some(day_set) = &TASKS[col].days {
                let date = first + Duration::days(row as i64);
                if !day_set.contains(&date.weekday()) {
                    return Some(Colors::gray(0.7));
                }
            }
        }
        None
    };

    let date_names_str: Vec<&str> = date_names.iter().map(|s| s.as_str()).collect();
    Builder::new()
        .doc_title(doc_title)
        // TODO: Make this dependent on the cell size?
        .box_width(2.0.mm())
        .row_labels(&date_names_str)
        .col_labels(&col_labels)
        .num_cols(cols as usize)
        .bounds(table_bounds)
        .top_label_height(top_box_height)
        .left_label_width(15.0.mm())
        .font(&times_bold)
        .horiz_line_width_func(&horiz_line_width_func)
        .vert_line_width_func(&vert_line_width_func)
        .cell_background_func(&cell_background_func)
        .generate_instructions()
        .draw_to_layer(&doc.get_page(page).get_layer(layer));

    doc.save(&mut BufWriter::new(File::create(output_filename).unwrap()))
        .unwrap();

    Ok(())
}

fn main() {
    let args: Args = argh::from_env();

    if args.date.is_empty() {
        if let Err(err) = main_func(&weekly::today()) {
            eprintln!("Error: {:?}", err);
        }
    } else {
        for date in &args.date {
            if let Err(err) = main_func(date) {
                eprintln!("Error: {} : {:?}", date.format("%Y-%m"), err);
            }
        }
    }
}
