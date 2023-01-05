use crate::{FontProxy, Instructions, NumericUnit, Unit};

#[derive(Debug, Clone)]
pub struct TextContext {
    proxy: FontProxy,
    text_height: Unit,
}

impl TextContext {
    pub fn times() -> TextContext {
        TextContext {
            proxy: FontProxy::Times(false, false),
            ..Default::default()
        }
    }

    pub fn helvetica() -> TextContext {
        TextContext {
            proxy: FontProxy::Helvetica(false, false),
            ..Default::default()
        }
    }

    pub fn with_text_height(&self, text_height: Unit) -> TextContext {
        TextContext {
            text_height,
            ..*self
        }
    }

    pub fn bold(&self, bold: bool) -> TextContext {
        TextContext {
            proxy: self.proxy.bold(bold),
            ..*self
        }
    }

    pub fn render(&self, txt: impl AsRef<str>, x: Unit, y: Unit, instructions: &mut Instructions) {
        instructions.push_text(txt.as_ref(), self.text_height.to_mm(), x, y, self.proxy);
    }
}

impl Default for TextContext {
    fn default() -> Self {
        TextContext {
            proxy: Default::default(),
            text_height: 12.0.mm(),
        }
    }
}
