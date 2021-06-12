use printpdf::*;

pub fn inches_to_mm(inches: f64) -> Mm {
    Mm(inches * 25.4)
}

pub fn point_pair(x: f64, y: f64) -> (Point, bool) {
    (Point::new(Mm(x), Mm(y)), false)
}
