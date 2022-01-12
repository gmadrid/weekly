use crate::units::Unit;
use crate::{Result, WRect};
use printpdf::*;
use std::fs::File;
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

    pub fn push_text(
        &mut self,
        s: &str,
        text_height: f64,
        x: Unit,
        y: Unit,
        font: &IndirectFontRef,
    ) {
        self.instructions.push(Instruction::Text(TextValues {
            s: s.to_string(),
            text_height,
            x,
            y,
            font: font.clone(),
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

    pub fn draw_to_layer(&self, layer: &PdfLayerReference) {
        for instruction in &self.instructions {
            instruction.draw_to_layer(layer);
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

    fn draw_to_layer(&self, layer: &PdfLayerReference) {
        match self {
            Instruction::Shape(line) => layer.add_shape(line.clone()),
            Instruction::Attrs(attrs) => attrs.execute_in_layer(layer),
            Instruction::Text(txt) => layer.use_text(
                txt.s.clone(),
                txt.text_height,
                txt.x.into(),
                txt.y.into(),
                &txt.font,
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

    pub fn execute_in_layer(&self, layer: &PdfLayerReference) {
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
    font: IndirectFontRef,
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

    pub fn letter() -> WRect {
        quadrant1(8.5.inches(), 11.0.inches())
        //WRect::with_dimensions(8.5.inches(), 11.0.inches()).move_to(0.0.inches(), 11.0.inches())
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

    fn quadrant1(width: Unit, height: Unit) -> WRect {
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

    callback(&doc, page_bounds)?.draw_to_layer(&doc.get_page(page).get_layer(layer));

    doc.save(&mut BufWriter::new(File::create(filename)?))?;
    Ok(())
}
