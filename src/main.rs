use argh::FromArgs;
use chrono::{Datelike, Duration, Local, NaiveDate};
use printpdf::{BuiltinFont, PdfDocument};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use weekly::{table_grid, NumericUnit, WRect};

const DEFAULT_NUM_COLS: u16 = 25;
const DEFAULT_TOP_LABEL_HEIGHT: f64 = 2.0;
const DEFAULT_OUTPUT_FILE: &str = "table_grid.pdf";

#[derive(Debug, FromArgs)]
/// Create a monthly checklist.
struct Args {
    #[argh(option, long = "cols", default = "DEFAULT_NUM_COLS")]
    /// number of columns in the grid
    num_cols: u16,

    #[argh(positional, default = "DEFAULT_OUTPUT_FILE.to_string()")]
    output_filename: String,

    #[argh(option, default = "DEFAULT_TOP_LABEL_HEIGHT")]
    /// height (in inches) of the top label area.
    top_label_height: f64,
}

fn days_in_month(date: &NaiveDate) -> i64 {
    let next_month = if date.month() == 12 {
        NaiveDate::from_ymd(date.year() + 1, 1, 1)
    } else {
        NaiveDate::from_ymd(date.year(), date.month() + 1, 1)
    };

    next_month
        .signed_duration_since(NaiveDate::from_ymd(date.year(), date.month(), 1))
        .num_days()
}

fn get_date_names() -> Vec<String> {
    let today = Local::now().date();
    let num_days = days_in_month(&NaiveDate::from_ymd(
        today.year(),
        today.month(),
        today.day(),
    ));
    let first_of_month = NaiveDate::from_ymd(today.year(), today.month(), 1);

    let mut labels = vec![];
    for days in 0..num_days {
        labels.push(
            (first_of_month + Duration::days(days))
                .format("%b %e")
                .to_string(),
        );
    }
    labels
}

fn main_func(args: Args) -> weekly::Result<()> {
    let date_names = get_date_names();

    let page_rect = WRect::with_dimensions(5.5.inches(), 8.5.inches());
    let table_bounds = page_rect.inset_all(
        0.25.inches() + 0.125.inches(),
        0.25.inches(),
        0.25.inches(),
        0.25.inches(),
    );
    let top_box_height = args.top_label_height.inches();
    let cols = args.num_cols;

    let (doc, page, layer) = PdfDocument::new(
        Path::new(&args.output_filename)
            .file_stem()
            .map(|o| o.to_string_lossy())
            .unwrap_or_else(|| "Grid".into()),
        page_rect.width().into(),
        page_rect.height().into(),
        "Layer 1",
    );
    let times_bold = doc.add_builtin_font(BuiltinFont::TimesBold).unwrap();

    let grid = table_grid(
        &date_names,
        cols,
        &table_bounds,
        top_box_height,
        15.0.mm(),
        page_rect.height(),
        &times_bold,
    );
    grid.draw_to_layer(&doc.get_page(page).get_layer(layer), page_rect.height());

    doc.save(&mut BufWriter::new(
        File::create(args.output_filename).unwrap(),
    ))
    .unwrap();

    Ok(())
}

fn main() {
    let args: Args = argh::from_env();

    if let Err(err) = main_func(args) {
        eprintln!("Error: {:?}", err);
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
