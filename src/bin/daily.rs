use argh::FromArgs;
use chrono::{Datelike, Duration, NaiveDate, Weekday};
use printpdf::{BuiltinFont, PdfDocument};
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

    let col_labels: Vec<String> = vec![
        "Plank",
        "Journal",
        "Virtuemap",
        "",
        "",
        "Check calendar",
        "Check ToDo list",
        "",
        "Brush teeth",
        "Floss",
        "Knit",
        "Magic",
        "Chess",
        "",
        "",
        "Code reviews",
        "Inbox Zero",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

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
    let horiz_line_width_func = move |row: usize| {
        let date = first + Duration::days(row as i64);
        if date.weekday() == Weekday::Sun {
            1.0
        } else {
            0.0
        }
    };
    let vert_line_width_func = |col: usize| {
        if col > 0 && col % 5 == 0 {
            1.5
        } else {
            0.0
        }
    };
    let col_labels_ref = &col_labels;
    let first = date.first_of_month();
    let cell_background_func = |row: usize, col: usize| {
        let date = first + Duration::days(row as i64);
        if row < 26 {
            return Some(Colors::gray(0.4))
        }
        if col < col_labels_ref.len() {
            let label = col_labels_ref[col].as_str();
            if label == "Code reviews" || label == "Inbox Zero" {
                if date.weekday() == Weekday::Sun || date.weekday() == Weekday::Sat {
                    return Some(Colors::gray(0.4))
                }
            }
        }
        None
    };

    Builder::new()
        .doc_title(doc_title)
        .row_labels(&date_names)
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
