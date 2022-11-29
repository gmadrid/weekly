use chrono::{Datelike, NaiveDate};
use printpdf::*;
use std::borrow::Cow;
use std::path::PathBuf;
use argh::FromArgs;
use weekly::{
    save_one_page_document, Colors, Datetools, FontProxy, GridDescription, Instructions,
    NumericUnit, TGrid, Unit, WRect,
};

#[derive(FromArgs)]
#[argh(description = "Creates a checklist of monthly tasks.")]
struct MonthlyArgs {
    // todo: allow use of yyyy-mm for start
    #[argh(option, long = "start", description = "the start month (yyyy-mm-dd)")]
    start_date: Option<NaiveDate>,
}

fn names_for_months(start_date: &NaiveDate, n: usize) -> Vec<String> {
    let mut month = start_date.first_of_month();
    let mut output = vec![];
    for _ in 0..n {
        output.push(month.format("%b %Y").to_string());
        month = month.next_month(); //  next_month(&curr_month);
    }
    output
}

fn default_output_filename(date: &NaiveDate) -> PathBuf {
    format!("monthlies-{}.pdf", date.format("%Y-%m")).into()
}

fn default_doc_title(date: &NaiveDate) -> String {
    format!("Monthly Checklist (starting {})", date.format("%B %Y"))
}

struct MonthlyDescription {
    bounds: WRect,
    month_names: Vec<String>,
}

impl MonthlyDescription {
    const NUM_ROWS: usize = 35;
    const NUM_COLS: usize = 20;

    const ROW_LABELS: [&'static str; 12] = [
        "Pay AmEx",
        "Pay Chase",
        "Pay Fidelity",
        "Pay Capital One",
        "Pay Apple",
        "Pay mortgage",
        "Pay Immersion",
        "Balance checkbook",
        "",
        "Check smoke alarms",
        "Change sleep equip.",
        "Run FI simulation",
    ];

    pub fn for_start_month<DL>(date: &DL, grid_rect: &WRect) -> MonthlyDescription
    where
        DL: Datelike,
    {
        MonthlyDescription {
            bounds: grid_rect.clone(),
            month_names: names_for_months(&date.first_of_month(), Self::NUM_ROWS),
        }
    }
}

impl GridDescription for MonthlyDescription {
    fn bounds(&self) -> WRect {
        self.bounds.clone()
    }

    fn num_rows(&self) -> Option<usize> {
        Some(Self::NUM_ROWS)
    }

    fn num_cols(&self) -> Option<usize> {
        Some(Self::NUM_COLS)
    }

    fn row_label_width(&self) -> Option<Unit> {
        Some(2.0.inches())
    }

    fn col_label_height(&self) -> Option<Unit> {
        Some(1.0.inches())
    }

    fn row_label(&self, index: usize) -> Cow<str> {
        if index < Self::ROW_LABELS.len() {
            Self::ROW_LABELS[index].into()
        } else {
            "".into()
        }
    }

    fn col_label(&self, index: usize) -> Cow<str> {
        self.month_names[index].as_str().into()
    }

    fn column_background(&self, index: usize) -> Option<Color> {
        if index % 2 == 0 {
            Some(Colors::gray(0.9))
        } else {
            None
        }
    }

    fn font(&self) -> weekly::FontProxy {
        FontProxy::Helvetica(true, false)
    }
}

fn render_monthlies(
    date: &NaiveDate,
    _: &PdfDocumentReference,
    page_rect: &WRect,
) -> weekly::Result<Instructions> {
    let table_bounds =
        page_rect.inset_all_q1(0.25.inches(), 0.25.inches(), 0.25.inches(), 0.25.inches());

    let description = MonthlyDescription::for_start_month(date, &table_bounds);
    let grid = TGrid::with_description(description);
    Ok(grid.generate_instructions())
}

fn main() -> weekly::Result<()> {
    let args: MonthlyArgs = argh::from_env();

    let date = args.start_date.unwrap_or(weekly::today());
    let title = default_doc_title(&date);
    let filename = default_output_filename(&date);

    let page_bounds = weekly::sizes::letter();
    save_one_page_document(&title, &filename, &page_bounds, |d, r| {
        render_monthlies(&date, d, r)
    })
}
