use argh::FromArgs;
use chrono::{Datelike, Local, NaiveDate};
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use weekly::{Builder, NumericUnit, WRect};

#[derive(FromArgs)]
/// Generates a checklist of monthly tasks.
struct Args {}

fn next_month(date: &NaiveDate) -> NaiveDate {
    if date.month() == 12 {
        NaiveDate::from_ymd(date.year() + 1, 1, 1)
    } else {
        NaiveDate::from_ymd(date.year(), date.month() + 1, 1)
    }
}

fn months_from_date(date: &NaiveDate, n: usize) -> Vec<String> {
    // unwrap: first of month is always valid.
    let first_of_month: NaiveDate = date.with_day(1).unwrap();

    let mut curr_month = first_of_month;
    let mut output = vec![];
    for _ in 0..n {
        output.push(curr_month.format("%b %Y").to_string());
        curr_month = next_month(&curr_month);
    }
    output
}

fn default_output_filename(date: &NaiveDate) -> PathBuf {
    format!("monthlies-{}.pdf", date.format("%Y-%m")).into()
}

fn default_doc_title(date: &NaiveDate) -> String {
    format!("Monthly Checklist (starting {})", date.format("%B %Y"))
}

fn main_func() -> weekly::Result<()> {
    let num_rows = 35;
    let num_cols = 20;
    let date = Local::now().date().naive_local();
    let col_labels = months_from_date(&date, num_cols);
    let row_labels: Vec<String> = vec![
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
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    let page_rect =
        WRect::with_dimensions(5.5.inches(), 8.5.inches()).move_to(0.0.inches(), 8.5.inches());
    let table_bounds = page_rect.inset_all_q1(
        0.25.inches() + 0.125.inches(), // Extra 1/8" for the rings.
        0.25.inches(),
        0.25.inches(),
        0.25.inches(),
    );

    let doc_title = default_doc_title(&date);
    let output_filename = default_output_filename(&date);

    let (doc, page, layer) = PdfDocument::new(
        &doc_title,
        page_rect.width().into(),
        page_rect.height().into(),
        "Layer 1",
    );
    let times_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();

    Builder::new()
        .doc_title(doc_title)
        .row_labels(&row_labels)
        .col_labels(&col_labels)
        .num_rows(num_rows)
        .num_cols(num_cols)
        .bounds(table_bounds)
        .top_label_height(1.0.inches())
        .left_label_width(1.5.inches())
        .font(&times_bold)
        .generate_instructions()
        .draw_to_layer(&doc.get_page(page).get_layer(layer));

    doc.save(&mut BufWriter::new(File::create(output_filename).unwrap()))
        .unwrap();

    Ok(())
}

fn main() {
    if let Err(err) = main_func() {
        eprintln!("{:?}", err);
    }
}
