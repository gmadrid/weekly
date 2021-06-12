use argh::FromArgs;
use chrono::Local;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use weekly::PRect;

#[derive(FromArgs)]
/// Spew a calendar.
struct Args {
    #[argh(switch)]
    /// prune any dates not in the requested month.
    prune: bool,
}

fn inches_to_mm(inches: f64) -> Mm {
    Mm(inches * 25.4)
}

fn foo() {
    let page_size = PRect::new(0.0, 0.0, inches_to_mm(11.0).0, inches_to_mm(8.5).0);
    let (doc, page, layer) =
        PdfDocument::new("test", Mm(page_size.width), Mm(page_size.height), "Layer 1");
    let current_layer = doc.get_page(page).get_layer(layer);

    let printable_area = page_size.inset(6.0, 5.0);

    current_layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
    current_layer.set_outline_thickness(0.0);

    let col_rect = printable_area.with_width(printable_area.width / 8.0);

    for mult in 0..8 {
        current_layer.add_shape(col_rect.rmove(mult as f64 * col_rect.width, 0.0).shape());
    }

    //    current_layer.add_shape(col_rect.shape());

    /*    let points = vec![
        (Point::new(Mm(0.0), Mm(0.0)), false),
         (Point::new(Mm(100.0), Mm(0.0)), false),
          (Point::new(Mm(100.0), Mm(200.0)), false),
        (Point::new(Mm(0.0), Mm(200.0)), false),
    ];
    let line = Line {
    points: points,
    is_closed: true,
    has_fill: false,
    has_stroke: true,
    is_clipping_path: false,
    };
     */

    //    current_layer.add_shape(line);

    doc.save(&mut BufWriter::new(
        File::create("/mnt/c/Users/gmadr/Desktop/foo.pdf").unwrap(),
    ))
    .unwrap();
}

fn main() {
    let args: Args = argh::from_env();

    weekly::print_cal_for_month(Local::now().date(), args.prune);

    foo();
}
