// use std::fs::File;
// use std::io::BufWriter;
// use printpdf::PdfDocument;
// use weekly::{Colors, Instructions, LineModifiers, NumericUnit, Unit, WRect};
//
// const REMARKABLE_WIDTH: f64 = 157.2;
// const REMARKABLE_HEIGHT: f64 =209.6;

use image::Rgb;
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;
use printpdf::image::RgbImage;

const REMARKABLE_WIDTH: u32 = 1404;
const REMARKABLE_HEIGHT: u32 = 1872;

fn remarkable_bounds() -> Rect {
    Rect::at(0, 0).of_size(REMARKABLE_WIDTH, REMARKABLE_HEIGHT)
}

trait RectOps {
    fn inset(&self, inset_x: i32, inset_y: i32) -> Self;
    fn top_left(&self) -> (f32, f32);
    fn bottom_right(&self) -> (f32, f32);
}

impl RectOps for Rect {
    fn top_left(&self) -> (f32, f32) {
        (self.left() as f32, self.top() as f32)
    }

    fn bottom_right(&self) -> (f32, f32) {
        (self.right() as f32, self.bottom() as f32)
    }

    fn inset(&self, inset_x: i32, inset_y: i32) -> Rect {
        Rect::at(self.left() + inset_x, self.top() + inset_y)
            .of_size((self.width() as i32  - 2 * inset_x) as u32, (self.height() as i32 - 2 * inset_y) as u32)
    }
}

fn main() {
    let output_filename = "remtest.png";

    let page_bounds = remarkable_bounds();

    let content_bounds = page_bounds.inset(25, 25);

    let mut image_buffer = RgbImage::new(content_bounds.width(), content_bounds.height());
    draw_filled_rect_mut(&mut image_buffer, content_bounds, Rgb([255, 255, 255]));

    draw_line_segment_mut(&mut image_buffer, content_bounds.top_left(), content_bounds.bottom_right(), Rgb([248, 64, 64]));

    image_buffer.save(output_filename).unwrap();

    //image_buffer.


    // let doc_title = "Remarkable test";
    // let output_filename = "remtest.pdf";
    //
    // let page_bounds =
    // WRect::with_dimensions(REMARKABLE_WIDTH.mm(), REMARKABLE_HEIGHT.mm())
    //     .move_to(0.0.mm(), REMARKABLE_HEIGHT.mm());
    //
    // let mut instructions = Instructions::default();
    // instructions.set_fill_color(&Colors::red());
    // instructions.set_stroke_width(1.0);
    // instructions.set_stroke_color(&Colors::black());
    //
    // let black_rect = dbg!(page_bounds.inset_q1(2.0.mm(), 2.0.mm()));
    // let rect_shape = black_rect.as_shape().fill(false).stroke(true);
    // instructions.push_shape(rect_shape);
    //
    // let (doc, page, layer) = PdfDocument::new(
    //     doc_title,
    //     page_bounds.width().into(),
    //     page_bounds.height().into(),
    //     "Layer 1",
    // );
    //
    // let layer = doc.get_page(page).get_layer(layer);
    // instructions.draw_to_layer(&layer);
    //
    // doc.save(&mut BufWriter::new(File::create(output_filename).unwrap()))
    //     .unwrap();
}