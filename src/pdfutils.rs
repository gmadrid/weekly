use crate::instructions::{Instruction, Instructions};
use crate::proxies::FontProxy;
use crate::units::Unit;
use crate::{Attributes, ColorProxy, Result, WLine, WRect};
use printpdf::*;
use std::collections::HashMap;
use std::fs::File;
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

fn draw_instructions_to_layer(
    document: &PdfDocumentReference,
    layer: &PdfLayerReference,
    instructions: &Instructions,
) -> Result<()> {
    // TODO: should we ensure that the document and layer are consistent?
    let font_map = FontMap::default().resolve_fonts(document, instructions)?;

    for instruction in &instructions.instructions {
        draw_instruction_to_layer(layer, instruction, &font_map);
    }
    Ok(())
}

fn execute_attrs(layer: &PdfLayerReference, attrs: &Attributes) {
    if let Some(stroke_width) = &attrs.stroke_width {
        layer.set_outline_thickness(*stroke_width);
    }
    if let Some(stroke_color) = &attrs.stroke_color {
        layer.set_outline_color(color_from_proxy(stroke_color));
    }
    if let Some(fill_color) = &attrs.fill_color {
        layer.set_fill_color(color_from_proxy(fill_color));
    }
    if let Some((dash_len, gap)) = &attrs.dash {
        layer.set_line_dash_pattern(LineDashPattern::new(
            0,
            *dash_len,
            Some(*gap),
            None,
            None,
            None,
            None,
        ));
    }
}

fn color_from_proxy(proxy: &ColorProxy) -> Color {
    Color::Rgb(Rgb::new(proxy.r, proxy.g, proxy.b, None))
}

fn draw_instruction_to_layer(
    layer: &PdfLayerReference,
    instruction: &Instruction,
    font_map: &FontMap,
) {
    match instruction {
        Instruction::Line(wline) => layer.add_shape(wline.as_line()),
        Instruction::Rect(wrect) => layer.add_shape(wrect.as_line()),
        Instruction::Attrs(attrs) => execute_attrs(layer, attrs),
        Instruction::Text(txt) => layer.use_text(
            txt.s.clone(),
            txt.text_height,
            txt.x.into(),
            txt.y.into(),
            font_map.lookup(txt.font),
        ),
        Instruction::PushState => layer.save_graphics_state(),
        Instruction::PopState => layer.restore_graphics_state(),
        Instruction::Rotate(r) => layer.set_ctm(CurTransMat::Rotate(*r)),
        Instruction::Translate(xdelta, ydelta) => {
            layer.set_ctm(CurTransMat::Translate(xdelta.into(), ydelta.into()))
        }
    }
}

trait AsLine {
    fn as_line(&self) -> Line;
}

impl AsLine for WLine {
    fn as_line(&self) -> Line {
        Line {
            points: vec![
                point_pair(self.x1, self.y1, false),
                point_pair(self.x2, self.y2, false),
            ],
            has_stroke: true,
            has_fill: false,
            ..Line::default()
        }
    }
}

impl AsLine for WRect {
    fn as_line(&self) -> Line {
        Line {
            // In Q1, rects grow downward toward the bottom.
            points: vec![
                point_pair(self.left, self.top, false),
                point_pair(self.left + self.width, self.top, false),
                point_pair(self.left + self.width, self.top - self.height, false),
                point_pair(self.left, self.top - self.height, false),
            ],
            has_fill: self.has_fill,
            has_stroke: self.has_stroke,
            is_closed: true,
            ..Line::default()
        }
    }
}

pub fn save_one_page_document<F>(
    title: &str,
    filename: impl AsRef<Path>,
    page_bounds: &WRect,
    callback: F,
) -> Result<()>
where
    // TODO: can I get rid of PdfDocumentReference on callback?
    F: FnOnce(&PdfDocumentReference, &WRect) -> Result<Instructions>,
{
    let (doc, page, layer) = PdfDocument::new(
        title,
        page_bounds.width().into(),
        page_bounds.height().into(),
        "Layer 1",
    );

    draw_instructions_to_layer(
        &doc,
        &doc.get_page(page).get_layer(layer),
        &callback(&doc, page_bounds)?,
    )?;
    //XXXcallback(&doc, page_bounds)?.draw_to_layer(&doc, &doc.get_page(page).get_layer(layer))?;

    doc.save(&mut BufWriter::new(File::create(filename)?))?;
    Ok(())
}
