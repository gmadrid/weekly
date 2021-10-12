use crate::units::Unit;
use printpdf::*;

pub fn point_pair(x: Unit, y: Unit) -> (Point, bool) {
    (Point::new(x.into(), y.into()), false)
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

    pub fn set_stroke_color(&mut self, color: &Color) {
        self.last_attr_mut().stroke_color = Some(color.clone());
    }

    pub fn set_stroke_width(&mut self, width: f64) {
        self.last_attr_mut().stroke_width = Some(width);
    }

    pub fn set_fill_color(&mut self, color: &Color) {
        self.last_attr_mut().fill_color = Some(color.clone());
    }

    pub fn set_dash(&mut self, dash_len: i64, gap_len: i64) {
        self.last_attr_mut().dash = Some((Some(dash_len), gap_len));
    }

    pub fn clear_dash(&mut self) {
        self.last_attr_mut().dash = Some((None, 0));
    }

    pub fn last_attr_mut(&mut self) -> &mut Attributes {
        if !matches!(self.instructions.last(), Some(Instruction::Attrs(_))) {
            self.instructions.push(Instruction::Attrs(Attributes::default()));
        }
        // unwrap: The last three lines ensure that an Attrs is last in the instructions list.
        return self.instructions.last_mut().unwrap().attrs_mut().unwrap()
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
