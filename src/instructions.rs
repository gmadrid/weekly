use crate::proxies::{ColorProxy, FontProxy};
use crate::units::Unit;
use crate::WLine;
use crate::WRect;

#[derive(Default, Debug)]
pub struct Instructions {
    // TODO: can you just export iter()?
    pub instructions: Vec<Instruction>,
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

    pub fn push_line(&mut self, proxy: WLine) {
        self.instructions.push(Instruction::Line(proxy));
    }

    pub fn push_rect(&mut self, proxy: WRect) {
        self.instructions.push(Instruction::Rect(proxy));
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

    pub fn set_stroke_color(&mut self, color: ColorProxy) {
        self.last_attr_mut().stroke_color = Some(color);
    }

    pub fn set_stroke_width(&mut self, width: f64) {
        self.last_attr_mut().stroke_width = Some(width);
    }

    pub fn set_fill_color(&mut self, color: ColorProxy) {
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

    // pub fn draw_to_layer(
    //     &self,
    //     document: &PdfDocumentReference,
    //     layer: &PdfLayerReference,
    // ) -> Result<()> {
    //     // TODO: should we ensure that the document and layer are consistent?
    //     let font_map = FontMap::default().resolve_fonts(document, self)?;
    //
    //     for instruction in &self.instructions {
    //         instruction.draw_to_layer(layer, &font_map);
    //     }
    //     Ok(())
    // }
}

#[derive(Debug)]
pub enum Instruction {
    Line(WLine),
    Rect(WRect),
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

    // fn draw_to_layer(&self, layer: &PdfLayerReference, font_map: &FontMap) {
    //     match self {
    //         Instruction::Shape(line) => layer.add_shape(line.clone()),
    //         Instruction::Attrs(attrs) => attrs.execute_in_layer(layer),
    //         Instruction::Text(txt) => layer.use_text(
    //             txt.s.clone(),
    //             txt.text_height,
    //             txt.x.into(),
    //             txt.y.into(),
    //             font_map.lookup(txt.font),
    //         ),
    //         Instruction::PushState => layer.save_graphics_state(),
    //         Instruction::PopState => layer.restore_graphics_state(),
    //         Instruction::Rotate(r) => layer.set_ctm(CurTransMat::Rotate(*r)),
    //         Instruction::Translate(x, y) => {
    //             layer.set_ctm(CurTransMat::Translate(x.into(), y.into()));
    //         }
    //     };
    // }
}

#[derive(Debug, Default)]
pub struct Attributes {
    pub stroke_width: Option<f64>,
    pub stroke_color: Option<ColorProxy>,
    pub fill_color: Option<ColorProxy>,
    pub dash: Option<(Option<i64>, i64)>,
}

impl Attributes {
    pub fn with_stroke_width(mut self, width: f64) -> Self {
        self.stroke_width = Some(width);
        self
    }

    pub fn with_stroke_color(mut self, color: &ColorProxy) -> Self {
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

    // fn execute_in_layer(&self, layer: &PdfLayerReference) {
    //     if let Some(stroke_width) = &self.stroke_width {
    //         layer.set_outline_thickness(*stroke_width);
    //     }
    //     if let Some(stroke_color) = &self.stroke_color {
    //         layer.set_outline_color(stroke_color.clone());
    //     }
    //     if let Some(fill_color) = &self.fill_color {
    //         layer.set_fill_color(fill_color.clone());
    //     }
    //     if let Some((dash_len, gap)) = &self.dash {
    //         layer.set_line_dash_pattern(LineDashPattern::new(
    //             0,
    //             *dash_len,
    //             Some(*gap),
    //             None,
    //             None,
    //             None,
    //             None,
    //         ));
    //     }
    // }
}

#[derive(Debug)]
pub struct TextValues {
    pub s: String,
    pub text_height: f64,
    pub x: Unit,
    pub y: Unit,
    pub font: FontProxy,
}
