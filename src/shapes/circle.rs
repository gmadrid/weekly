use crate::{ToPdfLine, NumericUnit, Unit};
use printpdf::Line;

#[derive(Debug)]
pub struct Circle {
    radius: Unit,
    x: Unit,
    y: Unit,
}

impl Circle {
    // pub fn circle(radius: Unit, x: Unit, y: Unit) -> Circle {
    //     Circle { radius, x, y }
    // }

    pub fn at_zero(radius: Unit) -> Circle {
        Circle {
            radius,
            x: Unit::zero(),
            y: Unit::zero(),
        }
    }

    pub fn unit_at(x: Unit, y: Unit) -> Circle {
        Circle {
            radius: 1.0.mm(),
            x,
            y,
        }
    }

    pub fn move_to(&self, x: Unit, y: Unit) -> Circle {
        Circle { x, y, ..*self }
    }
}

impl ToPdfLine for Circle {
    fn to_pdf_line(self) -> Line {
        let points = printpdf::calculate_points_for_circle(self.radius, self.x, self.y);
        Line {
            points,
            has_stroke: true,
            is_closed: true,
            ..Line::default()
        }
    }
}
