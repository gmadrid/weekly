use chrono::{Datelike, NaiveDate};
use printpdf::*;
use std::borrow::Cow;
use std::path::PathBuf;
use weekly::{
    save_one_page_document, Colors, Datetools, GridDescription, Instructions, NumericUnit, TGrid,
    Unit, WRect,
};

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
    font: IndirectFontRef,
}

impl MonthlyDescription {
    const NUM_ROWS: usize = 35;
    const NUM_COLS: usize = 20;

    const ROW_LABELS: [&'static str; 10] = [
        "Pay AmEx",
        "Pay Chase",
        "Pay Fidelity",
        "Pay Capital One",
        "Pay mortgage",
        "Balance checkbook",
        "",
        "Check smoke alarms",
        "Change sleep equip.",
        "Run FI simulation",
    ];

    pub fn for_start_month<DL>(
        date: &DL,
        grid_rect: &WRect,
        font: IndirectFontRef,
    ) -> MonthlyDescription
    where
        DL: Datelike,
    {
        MonthlyDescription {
            bounds: grid_rect.clone(),
            month_names: names_for_months(&date.first_of_month(), Self::NUM_ROWS),
            font,
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

    fn row_label(&self, index: usize) -> Cow<'static, str> {
        if index < Self::ROW_LABELS.len() {
            Self::ROW_LABELS[index].into()
        } else {
            "".into()
        }
    }

    fn col_label(&self, index: usize) -> Cow<'static, str> {
        // TODO: can we get rid of this clone()?
        self.month_names[index].clone().into()
    }

    fn column_background(&self, index: usize) -> Option<Color> {
        if index % 2 == 0 {
            Some(Colors::gray(0.9))
        } else {
            None
        }
    }

    fn font(&self) -> &IndirectFontRef {
        &self.font
    }
}

fn render_monthlies(
    date: &NaiveDate,
    doc: &PdfDocumentReference,
    page_rect: &WRect,
) -> weekly::Result<Instructions> {
    let table_bounds =
        page_rect.inset_all_q1(0.25.inches(), 0.25.inches(), 0.25.inches(), 0.25.inches());

    let font = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;
    let description = MonthlyDescription::for_start_month(date, &table_bounds, font);
    let grid = TGrid::with_description(description);
    Ok(grid.generate_instructions())
}

fn main_func() -> weekly::Result<()> {
    let date = weekly::today();
    let title = default_doc_title(&date);
    let filename = default_output_filename(&date);

    let page_bounds = weekly::sizes::letter();
    save_one_page_document(&title, &filename, &page_bounds, |d, r| {
        render_monthlies(&date, d, r)
    })
}

fn main() {
    if let Err(err) = main_func() {
        eprintln!("{:?}", err);
    }
}
