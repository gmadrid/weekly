use crate::units::Unit;
use crate::{Result, WRect};
use printpdf::*;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::BufWriter;
use std::path::Path;

pub fn point_pair(x: Unit, y: Unit, next: bool) -> (Point, bool) {
    (Point::new(x.into(), y.into()), next)
}

#[derive(Debug, Default)]
struct FontMap(HashMap<FontProxy, IndirectFontRef>);

impl FontMap {
    fn resolve_fonts(
        mut self,
        doc: &PdfDocumentReference,
        instructions: &Instructions,
    ) -> Result<FontMap> {
        // Look for all of the fonts referenced in the Instructions,
        // add them to the PdfDocument, adding the fonts to map.
        instructions
            .instructions
            .iter()
            .filter_map(|i| match i {
                Instruction::Text(tv) => Some(tv.font),
                _ => None,
            })
            .try_for_each::<_, Result<()>>(|font| {
                let entry = self.0.entry(font);

                // Basically doing or_insert_with(), but I need to propagate an error.
                if let std::collections::hash_map::Entry::Vacant(ve) = entry {
                    let indirect_font = doc.add_builtin_font(font.into())?;
                    ve.insert(indirect_font);
                }
                Ok(())
            })?;
        Ok(self)
    }

    fn lookup(&self, font_proxy: FontProxy) -> &IndirectFontRef {
        // unwrap: can we get rid of this?
        self.0.get(&font_proxy).unwrap()
    }
}

impl From<FontProxy> for BuiltinFont {
    fn from(font_proxy: FontProxy) -> Self {
        match font_proxy {
            FontProxy::Helvetica(bold, italic) => {
                if bold && italic {
                    BuiltinFont::HelveticaBoldOblique
                } else if bold {
                    BuiltinFont::HelveticaBold
                } else if italic {
                    BuiltinFont::HelveticaOblique
                } else {
                    BuiltinFont::Helvetica
                }
            }
            FontProxy::Times(bold, italic) => {
                if bold && italic {
                    BuiltinFont::TimesBoldItalic
                } else if bold {
                    BuiltinFont::TimesBold
                } else if italic {
                    BuiltinFont::TimesItalic
                } else {
                    BuiltinFont::TimesRoman
                }
            }
        }
    }
}

pub struct Colors {}

impl Colors {

}

pub trait LineModifiers {
    fn stroke(self, value: bool) -> Self;
    fn fill(self, value: bool) -> Self;
}

impl LineModifiers for Line {
    fn stroke(mut self, value: bool) -> Self {
        self.has_stroke = value;
        self
    }

    fn fill(mut self, value: bool) -> Self {
        self.has_fill = value;
        self
    }
}

pub mod sizes {
    use crate::{NumericUnit, Unit, WRect};

    pub fn cornell_rule_height() -> Unit {
        (9.0 / 32.0).inches()
    }

    pub fn wide_rule_height() -> Unit {
        (11.0 / 32.0).inches()
    }

    pub fn letter() -> WRect {
        quadrant1(8.5.inches(), 11.0.inches())
    }

    pub fn legal() -> WRect {
        quadrant1(8.5.inches(), 14.0.inches())
    }

    pub fn tableau() -> WRect {
        quadrant1(11.0.inches(), 17.0.inches())
    }

    pub fn a4() -> WRect {
        quadrant1(210.0.mm(), 297.0.mm())
    }

    // Remarkable claims to want 1404×1872 pixel images. (4/3 aspect ratio)
    // These dimensions below are producing a 928x1237 pixel image.
    const REMARKABLE_WIDTH_MM: f64 = 157.2;
    const REMARKABLE_HEIGHT_MM: f64 = 209.6;

    pub fn remarkable2() -> WRect {
        quadrant1(REMARKABLE_WIDTH_MM.mm(), REMARKABLE_HEIGHT_MM.mm())
    }

    const fn quadrant1(width: Unit, height: Unit) -> WRect {
        WRect::with_dimensions(width, height).move_to(Unit::zero(), height)
    }
}

pub fn save_one_page_document<F>(
    title: &str,
    filename: impl AsRef<Path>,
    page_bounds: &WRect,
    callback: F,
) -> Result<()>
where
    F: FnOnce(&PdfDocumentReference, &WRect) -> Result<Instructions>,
{
    let (doc, page, layer) = PdfDocument::new(
        title,
        page_bounds.width().into(),
        page_bounds.height().into(),
        "Layer 1",
    );

    callback(&doc, page_bounds)?.draw_to_layer(&doc, &doc.get_page(page).get_layer(layer))?;

    doc.save(&mut BufWriter::new(File::create(filename)?))?;
    Ok(())
}
