use chrono::{Datelike, NaiveDate};
use printpdf::{BuiltinFont, IndirectFontRef, PdfDocument};
use std::borrow::Cow;
use std::fs::File;
use std::io::BufWriter;
use weekly::{today, Datetools, GridDescription, NumericUnit, TGrid, Unit, WRect};

struct DailyDescription {
    bounds: WRect,
    dates_in_month: Vec<NaiveDate>,
    font: IndirectFontRef,
}

impl DailyDescription {
    pub fn for_month<DL>(date: &DL, bounds: WRect, font: IndirectFontRef) -> DailyDescription
    where
        DL: Datelike,
    {
        DailyDescription {
            bounds,
            dates_in_month: date.dates_in_month(),
            font,
        }
    }
}

impl GridDescription for DailyDescription {
    fn bounds(&self) -> WRect {
        self.bounds.clone()
    }

    fn num_rows(&self) -> Option<usize> {
        Some(self.dates_in_month.len())
    }

    fn num_cols(&self) -> Option<usize> {
        Some(5)
    }

    fn row_label_width(&self) -> Option<Unit> {
        Some(1.0.inches())
    }

    fn col_label_height(&self) -> Option<Unit> {
        Some(2.0.inches())
    }

    fn row_label(&self, index: usize) -> Cow<'static, str> {
        self.dates_in_month[index]
            .format("%b %e")
            .to_string()
            .into()
    }

    fn col_label(&self, index: usize) -> Cow<'static, str> {
        format!("Column {}", index).into()
    }

    fn font(&self) -> &IndirectFontRef {
        &self.font
    }
}

fn main() {
    let doc_title = "Foo";
    let output_filename = "foo.pdf";

    let page_rect =
        WRect::with_dimensions(8.5.inches(), 11.0.inches()).move_to(0.0.inches(), 11.0.inches());

    let (doc, page, layer) = PdfDocument::new(
        doc_title,
        page_rect.width().into(),
        page_rect.height().into(),
        "Layer 1",
    );

    let description = DailyDescription::for_month(
        &today(),
        page_rect.inset_q1(1.0.inches(), 1.0.inches()),
        doc.add_builtin_font(BuiltinFont::TimesBold).unwrap(),
    );
    let grid = TGrid::with_description(&description);

    grid.generate_instructions()
        .draw_to_layer(&doc.get_page(page).get_layer(layer));

    doc.save(&mut BufWriter::new(File::create(output_filename).unwrap()))
        .unwrap();
}
