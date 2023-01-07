use crate::shapes::{RenderAttrsImpl, ToPlainPdfLine};
use crate::{NumericUnit, Unit};
use printpdf::Line;

#[derive(Debug, Default)]
pub struct Circle {
    render_attrs: RenderAttrsImpl,

    radius: Unit,
    x: Unit,
    y: Unit,
}

impl Circle {
    pub fn at_zero(radius: Unit) -> Circle {
        Circle {
            radius,
            x: Unit::zero(),
            y: Unit::zero(),
            ..Self::default()
        }
    }

    pub fn unit_at(x: Unit, y: Unit) -> Circle {
        Circle {
            radius: 1.0.mm(),
            x,
            y,
            ..Self::default()
        }
    }

    pub fn move_to(&self, x: Unit, y: Unit) -> Circle {
        Circle { x, y, ..*self }
    }

    pub fn resize(&self, radius: Unit) -> Circle {
        Circle { radius, ..*self }
    }
}

impl AsRef<RenderAttrsImpl> for Circle {
    fn as_ref(&self) -> &RenderAttrsImpl {
        &self.render_attrs
    }
}

impl AsMut<RenderAttrsImpl> for Circle {
    fn as_mut(&mut self) -> &mut RenderAttrsImpl {
        &mut self.render_attrs
    }
}

impl ToPlainPdfLine for Circle {
    fn to_plain_pdf_line(&self) -> Line {
        Line {
            points: printpdf::calculate_points_for_circle(self.radius, self.x, self.y),
            is_closed: true,
            ..Line::default()
        }
    }
}
