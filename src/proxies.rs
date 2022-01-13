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

#[derive(Debug, Clone)]
pub struct ColorProxy {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl ColorProxy {
    pub fn rgb(r: f64, g: f64, b: f64) -> ColorProxy {
        ColorProxy { r, g, b }
    }

    pub fn gray(level: f64) -> ColorProxy {
        Self::rgb(level, level, level)
    }

    pub fn black() -> ColorProxy {
        Self::gray(0.0)
    }

    pub fn white() -> ColorProxy {
        Self::gray(1.0)
    }

    pub fn red() -> ColorProxy {
        Self::rgb(1.0, 0.0, 0.0)
    }

    pub fn green() -> ColorProxy {
        Self::rgb(0.0, 1.0, 0.0)
    }

    pub fn blue() -> ColorProxy {
        Self::rgb(0.0, 0.0, 1.0)
    }
}
