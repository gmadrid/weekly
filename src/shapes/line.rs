use crate::units::Unit;

#[derive(Debug, Clone)]
pub struct WLine {
    pub x1: Unit,
    pub y1: Unit,
    pub x2: Unit,
    pub y2: Unit,
}

impl WLine {
    pub fn line(x1: Unit, y1: Unit, x2: Unit, y2: Unit) -> WLine {
        WLine { x1, y1, x2, y2 }
    }
}
