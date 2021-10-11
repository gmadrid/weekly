use argh::FromArgs;
use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use printpdf::{BuiltinFont, PdfDocument};
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use weekly::{Builder, NumericUnit, WRect};

const DEFAULT_NUM_COLS: u16 = 25;
const DEFAULT_TOP_LABEL_HEIGHT: f64 = 2.0;

#[derive(Debug, FromArgs)]
/// Create a monthly checklist.
///
/// Generates a daily checklist for every date supplied.
struct Args {
    /// month for which to generate the checklist
    #[argh(positional)]
    date: Vec<NaiveDate>,
}

fn days_in_month(date: &impl Datelike) -> i64 {
    let next_month = if date.month() == 12 {
        NaiveDate::from_ymd(date.year() + 1, 1, 1)
    } else {
        NaiveDate::from_ymd(date.year(), date.month() + 1, 1)
    };

    next_month
        .signed_duration_since(NaiveDate::from_ymd(date.year(), date.month(), 1))
        .num_days()
}

fn first_of_month(date: &impl Datelike) -> NaiveDate {
    NaiveDate::from_ymd(date.year(), date.month(), 1)
}

fn naive_today() -> NaiveDate {
    let today = Local::now().date();
    NaiveDate::from_ymd(today.year(), today.month(), today.day())
}

fn get_date_names(date: &impl Datelike) -> Vec<String> {
    let num_days = days_in_month(date);
    let first_of_month = first_of_month(date);

    (0..num_days)
        .map(|days| {
            (first_of_month + Duration::days(days))
                .format("%b %e")
                .to_string()
        })
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
        "Check calendar",
        "Inbox Zero",
        "Code reviews",
        "",
        "Brush teeth",
        "Floss",
        "",
        "Play chess",
        "",
        "Plank",
        "Stretch",
        "",
        "Check To Do list",
    ]
    .iter()
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

    let top_box_height = DEFAULT_TOP_LABEL_HEIGHT.inches();
    let cols = DEFAULT_NUM_COLS;

    let output_filename = default_output_filename(&date);
    let doc_title = default_doc_title(&date);

    let (doc, page, layer) = PdfDocument::new(
        &doc_title,
        page_rect.width().into(),
        page_rect.height().into(),
        "Layer 1",
    );
    let times_bold = doc.add_builtin_font(BuiltinFont::TimesBold).unwrap();

    let first = first_of_month(date);
    let width_func = move |row: usize| {
        let date = first + Duration::days(row as i64);
        if date.weekday() == Weekday::Sun {
            1.0
        } else {
            0.0
        }
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
        .width_func(width_func)
        .generate_instructions()
        .draw_to_layer(&doc.get_page(page).get_layer(layer));

    doc.save(&mut BufWriter::new(File::create(output_filename).unwrap()))
        .unwrap();

    Ok(())
}

fn main() {
    let args: Args = argh::from_env();

    if args.date.is_empty() {
        if let Err(err) = main_func(&naive_today()) {
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

// use argh::FromArgs;
// use chrono::Local;
// use printpdf::*;
// use std::fs::File;
// use std::io::BufWriter;
// use weekly::{inches_to_mm, PLine, PRect};
//
// #[derive(FromArgs)]
// /// Spew a calendar.
// struct Args {
//     #[argh(switch)]
//     /// prune any dates not in the requested month.
//     prune: bool,
// }
//
// fn foo() {
//     let page_size = PRect::new(0.0, 0.0, inches_to_mm(11.0).0, inches_to_mm(8.5).0);
//     let (doc, page, layer) =
//         PdfDocument::new("test", Mm(page_size.width), Mm(page_size.height), "Layer 1");
//     let current_layer = doc.get_page(page).get_layer(layer);
//
//     //    let printable_area = page_size.inset(6.0, 5.0);
//     // printable margins appear to be top/bottom: 5mm, left: 13mm, right: 20mm
//     let printable_area = page_size
//         .rmove(14.0, 5.0)
//         .with_width(page_size.width - 13.0 - 20.0)
//         .with_height(page_size.height - 2.0 * 5.0);
//
//     current_layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
//     current_layer.set_outline_thickness(0.0);
//
//     let col_rect = printable_area.with_width(printable_area.width / 8.0);
//
//     for mult in 0..8 {
//         current_layer.add_shape(col_rect.rmove(mult as f64 * col_rect.width, 0.0).shape());
//     }
//
//     let col_1 = col_rect.rmove(1.0 * col_rect.width, 0.0);
//     let col_2 = col_rect.rmove(2.0 * col_rect.width, 0.0);
//
//     let mut dividers = make_dividers(col_1, 2);
//     dividers.append(&mut make_dividers(col_2, 3));
//
//     for divider in dividers {
//         current_layer.add_shape(divider.shape());
//     }
//
//     doc.save(&mut BufWriter::new(File::create("foo.pdf").unwrap()))
//         .unwrap();
// }
//
// fn make_dividers(rect: PRect, num_dividers: u8) -> Vec<PLine> {
//     let delta = rect.height / (num_dividers + 1) as f64;
//
//     let mut result = vec![];
//     for index in 0..num_dividers {
//         result.push(PLine::horiz(
//             rect.x,
//             rect.y + delta * (index + 1) as f64,
//             rect.width,
//         ))
//     }
//
//     result
// }
//
// fn main() {
//     let args: Args = argh::from_env();
//
//     weekly::print_cal_for_month(Local::now().date(), args.prune);
//
//     foo();
// }
