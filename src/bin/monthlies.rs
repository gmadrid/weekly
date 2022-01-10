use chrono::NaiveDate;
use printpdf::*;
use std::path::PathBuf;
use weekly::{save_one_page_document, Builder, Datetools, Instructions, NumericUnit, WRect};

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

fn render_monthlies(
    date: &NaiveDate,
    doc: &PdfDocumentReference,
    page_rect: &WRect,
) -> weekly::Result<Instructions> {
    let num_rows = 35;
    let num_cols = 20;
    let col_labels = names_for_months(date, num_cols);
    let row_labels = vec![
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

    let table_bounds = page_rect.inset_all_q1(
        0.25.inches() + 0.125.inches(), // Extra 1/8" for the rings.
        0.25.inches(),
        0.25.inches(),
        0.25.inches(),
    );

    let times_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    let col_label_strs: Vec<&str> = col_labels.iter().map(|s| s.as_str()).collect();
    Ok(Builder::new()
        .row_labels(&row_labels)
        .col_labels(&col_label_strs)
        .num_rows(num_rows)
        .num_cols(num_cols)
        .bounds(table_bounds)
        .top_label_height(1.0.inches())
        .left_label_width(1.5.inches())
        .font(&times_bold)
        .generate_instructions())
}

fn main_func() -> weekly::Result<()> {
    let date = weekly::today();
    let title = default_doc_title(&date);
    let filename = default_output_filename(&date);

    let page_bounds =
        WRect::with_dimensions(5.5.inches(), 8.5.inches()).move_to(0.0.inches(), 8.5.inches());
    save_one_page_document(&title, &filename, &page_bounds, |d, r| {
        render_monthlies(&date, d, r)
    })
}

fn main() {
    if let Err(err) = main_func() {
        eprintln!("{:?}", err);
    }
}
