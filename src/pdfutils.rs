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

#[derive(Default, Debug)]
pub struct Instructions {
    instructions: Vec<Instruction>,
}

impl Instructions {
    pub fn push_state(&mut self) {
        self.instructions.push(Instruction::PushState);
    }

    pub fn pop_state(&mut self) {
        self.instructions.push(Instruction::PopState);
    }

    pub fn rotate(&mut self, deg: f64) {
        self.instructions.push(Instruction::Rotate(deg));
    }

    pub fn translate(&mut self, x: Unit, y: Unit) {
        self.instructions.push(Instruction::Translate(x, y));
    }

    pub fn push_shape(&mut self, shape: Line) {
        self.instructions.push(Instruction::Shape(shape));
    }

    pub fn push_text(&mut self, s: &str, text_height: f64, x: Unit, y: Unit, font: FontProxy) {
        self.instructions.push(Instruction::Text(TextValues {
            s: s.to_string(),
            text_height,
            x,
            y,
            font,
        }))
    }

    pub fn set_stroke_color(&mut self, color: Color) {
        self.last_attr_mut().stroke_color = Some(color);
    }

    pub fn set_stroke_width(&mut self, width: f64) {
        self.last_attr_mut().stroke_width = Some(width);
    }

    pub fn set_fill_color(&mut self, color: Color) {
        self.last_attr_mut().fill_color = Some(color);
    }

    pub fn clear_fill_color(&mut self) {
        self.last_attr_mut().fill_color = None;
    }

    pub fn set_dash(&mut self, dash_len: i64, gap_len: i64) {
        self.last_attr_mut().dash = Some((Some(dash_len), gap_len));
    }

    pub fn clear_dash(&mut self) {
        self.last_attr_mut().dash = Some((None, 0));
    }

    pub fn last_attr_mut(&mut self) -> &mut Attributes {
        if !matches!(self.instructions.last(), Some(Instruction::Attrs(_))) {
            self.instructions
                .push(Instruction::Attrs(Attributes::default()));
        }
        // unwrap: The last three lines ensure that an Attrs is last in the instructions list.
        return self.instructions.last_mut().unwrap().attrs_mut().unwrap();
    }

    pub fn draw_to_layer(
        &self,
        document: &PdfDocumentReference,
        layer: &PdfLayerReference,
    ) -> Result<()> {
        // TODO: should we ensure that the document and layer are consistent?
        let font_map = FontMap::default().resolve_fonts(document, self)?;

        for instruction in &self.instructions {
            instruction.draw_to_layer(layer, &font_map);
        }
        Ok(())
    }
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

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum FontProxy {
    // first bool is Bold, second bool is Italics
    Helvetica(bool, bool),
    Times(bool, bool),
}

impl FontProxy {
    pub fn times_bold() -> FontProxy {
        FontProxy::Times(true, false)
    }
    pub fn helvetica_bold() -> FontProxy {
        FontProxy::Helvetica(true, false)
    }
}

impl Default for FontProxy {
    fn default() -> Self {
        FontProxy::Times(false, false)
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

#[derive(Debug)]
pub enum Instruction {
    Shape(Line),
    Attrs(Attributes),
    Text(TextValues),

    PushState,
    PopState,
    Rotate(f64),
    Translate(Unit, Unit),
}

impl Instruction {
    fn attrs_mut(&mut self) -> Option<&mut Attributes> {
        match self {
            Instruction::Attrs(attrs) => Some(attrs),
            _ => None,
        }
    }

    fn draw_to_layer(&self, layer: &PdfLayerReference, font_map: &FontMap) {
        match self {
            Instruction::Shape(line) => layer.add_shape(line.clone()),
            Instruction::Attrs(attrs) => attrs.execute_in_layer(layer),
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
            Instruction::Translate(x, y) => {
                layer.set_ctm(CurTransMat::Translate(x.into(), y.into()));
            }
        };
    }
}

#[derive(Debug, Default)]
pub struct Attributes {
    stroke_width: Option<f64>,
    stroke_color: Option<Color>,
    fill_color: Option<Color>,
    dash: Option<(Option<i64>, i64)>,
}

impl Attributes {
    pub fn with_stroke_width(mut self, width: f64) -> Self {
        self.stroke_width = Some(width);
        self
    }

    pub fn with_stroke_color(mut self, color: &Color) -> Self {
        self.stroke_color = Some(color.clone());
        self
    }

    pub fn with_dash(mut self, dash: i64, gap: i64) -> Self {
        self.dash = Some((Some(dash), gap));
        self
    }

    pub fn render(&self, instructions: &mut Instructions, f: impl FnOnce(&mut Instructions)) {
        let setting_something = self.stroke_width.is_some()
            || self.stroke_color.is_some()
            || self.fill_color.is_some()
            || self.dash.is_some();

        if setting_something {
            instructions.push_state();
        }

        if let Some(width) = self.stroke_width {
            instructions.set_stroke_width(width);
        }
        if let Some(stroke_color) = &self.stroke_color {
            instructions.set_stroke_color(stroke_color.clone());
        }
        if let Some(fill_color) = &self.fill_color {
            instructions.set_fill_color(fill_color.clone());
        }
        if let Some(dash) = self.dash {
            if let (Some(length), gap) = dash {
                instructions.set_dash(length, gap);
            } else {
                instructions.clear_dash();
            }
        }

        f(instructions);

        if setting_something {
            instructions.pop_state();
        }
    }

    fn execute_in_layer(&self, layer: &PdfLayerReference) {
        if let Some(stroke_width) = &self.stroke_width {
            layer.set_outline_thickness(*stroke_width);
        }
        if let Some(stroke_color) = &self.stroke_color {
            layer.set_outline_color(stroke_color.clone());
        }
        if let Some(fill_color) = &self.fill_color {
            layer.set_fill_color(fill_color.clone());
        }
        if let Some((dash_len, gap)) = &self.dash {
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
}

#[derive(Debug)]
pub struct TextValues {
    s: String,
    text_height: f64,
    x: Unit,
    y: Unit,
    font: FontProxy,
}

pub struct Colors {}

impl Colors {
    pub fn rgb(r: f64, g: f64, b: f64) -> Color {
        Color::Rgb(Rgb::new(r, g, b, None))
    }

    pub fn gray(level: f64) -> Color {
        Colors::rgb(level, level, level)
    }

    pub fn black() -> Color {
        Colors::gray(0.0)
    }

    pub fn white() -> Color {
        Colors::gray(1.0)
    }

    pub fn red() -> Color {
        Self::rgb(1.0, 0.0, 0.0)
    }

    pub fn green() -> Color {
        Self::rgb(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Color {
        Self::rgb(0.0, 0.0, 1.0)
    }
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

    // Remarkable claims to want 1404??1872 pixel images. (4/3 aspect ratio)
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
