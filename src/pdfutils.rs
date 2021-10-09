use crate::Unit;
use printpdf::*;

pub fn point_pair(x: Unit, y: Unit) -> (Point, bool) {
    (Point::new(Mm(x.0), Mm(y.0)), false)
}

// pub fn point_pair(x: f64, y: f64) -> (Point, bool) {
//     (Point::new(Mm(x), Mm(y)), false)
// }
