use crate::units::Unit;

#[derive(Debug, Clone)]
pub struct WRect {
    x1: Unit,
    y1: Unit,
    x2: Unit,
    y2: Unit,
}

impl WRect {
    pub fn rect(width: Unit, height: Unit) -> WRect {
        WRect {
            x1: Unit::zero(),
            y1: Unit::zero(),
            x2: width,
            y2: height,
        }
    }

    pub fn height(&self) -> Unit {
        // TODO: get rid of these .0 fields.
        Unit(f64::abs((self.y2 - self.y1).0))
    }

    pub fn width(&self) -> Unit {
        Unit(f64::abs((self.x2 - self.x1).0))
    }

    pub fn left(&self) -> Unit {
        Unit(self.x1.0.min(self.x2.0))
    }

    pub fn right(&self) -> Unit {
        Unit(self.x1.0.max(self.x2.0))
    }

    pub fn top(&self) -> Unit {
        Unit(self.y1.0.min(self.y2.0))
    }

    pub fn bottom(&self) -> Unit {
        Unit(self.y1.0.max(self.y2.0))
    }

    pub fn inset(&self, xdelta: Unit, ydelta: Unit) -> WRect {
        WRect {
            x1: self.x1 + xdelta,
            y1: self.y1 + ydelta,
            x2: self.x2 - xdelta,
            y2: self.y2 - ydelta,
        }
    }
}
