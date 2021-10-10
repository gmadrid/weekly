use crate::units::Unit;
use printpdf::*;

pub fn point_pair(x: Unit, y: Unit) -> (Point, bool) {
    (Point::new(Mm(x.0), Mm(y.0)), false)
}

#[derive(Default, Debug)]
pub struct Instructions {
    instructions: Vec<Instruction>,
}

impl Instructions {
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

    pub fn set_stroke_color(&mut self, color: &Color) {
        self.with_top_attributes(|attrs| attrs.stroke_color = Some(color.clone()));
    }

    pub fn set_stroke_width(&mut self, width: f64) {
        self.with_top_attributes(|attrs| attrs.stroke_width = Some(width));
    }

    pub fn set_fill_color(&mut self, color: &Color) {
        self.with_top_attributes(|attrs| attrs.fill_color = Some(color.clone()));
    }

    pub fn with_top_attributes(&mut self, mut f: impl FnMut(&mut Attributes)) {
        if let Some(Instruction::Attrs(attrs)) = self.instructions.last_mut() {
            f(attrs);
        } else {
            let mut attrs = Attributes::default();
            f(&mut attrs);
            self.instructions.push(Instruction::Attrs(attrs));
        }
    }

    pub fn draw_to_layer(&self, layer: &PdfLayerReference, page_height: Unit) {
        for instruction in &self.instructions {
            instruction.draw_to_layer(layer, page_height);
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Shape(Line),
    Attrs(Attributes),
    Text(TextValues),
}

impl Instruction {
    fn draw_to_layer(&self, layer: &PdfLayerReference, page_height: Unit) {
        match self {
            Instruction::Shape(line) => layer.add_shape(line.clone()),
            Instruction::Attrs(attrs) => attrs.execute_in_layer(layer),
            Instruction::Text(txt) => layer.use_text(
                txt.s.clone(),
                txt.text_height,
                txt.x.into(),
                (page_height - txt.y).into(),
                &txt.font,
            ),
        };
    }
}

#[derive(Debug, Default)]
pub struct Attributes {
    stroke_width: Option<f64>,
    stroke_color: Option<Color>,
    fill_color: Option<Color>,
}

impl Attributes {
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
