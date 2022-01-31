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

// Remarkable claims to want 1404×1872 pixel images. (4/3 aspect ratio)
// These dimensions below are producing a 928x1237 pixel image.
const REMARKABLE_WIDTH_MM: f64 = 157.2;
const REMARKABLE_HEIGHT_MM: f64 = 209.6;

pub fn remarkable2() -> WRect {
    quadrant1(REMARKABLE_WIDTH_MM.mm(), REMARKABLE_HEIGHT_MM.mm())
}

const fn quadrant1(width: Unit, height: Unit) -> WRect {
    WRect::with_dimensions(width, height).move_to(Unit::zero(), height)
}
