#[derive(Debug, Default, PartialEq, PartialOrd, Copy, Clone)]
// A `Unit` is a number of millimeters (mm) internally.
pub struct Unit(f64);

impl Unit {
    pub const fn zero() -> Unit {
        Unit(0.0)
    }

    pub fn abs(self) -> Unit {
        Unit(self.0.abs())
    }

    pub fn min(self, other: Unit) -> Unit {
        Unit(self.0.min(other.0))
    }

    pub fn max(self, other: Unit) -> Unit {
        Unit(self.0.max(other.0))
    }

    // Returns a new Unit that is `percentage` times Self.
    // percentage is expressed such that `x.pct(100.0) == x`.
    pub fn pct(self, percentage: f64) -> Unit {
        Unit(self.0 * (percentage / 100.0))
    }

    // The only way to strip the units off of a Unit is to return is as millimeters.
    pub fn to_mm(self) -> f64 {
        self.0
    }
}

impl From<Unit> for printpdf::Mm {
    fn from(unit: Unit) -> Self {
        printpdf::Mm(unit.0)
    }
}

impl From<&Unit> for printpdf::Mm {
    fn from(unit: &Unit) -> Self {
        printpdf::Mm(unit.0)
    }
}

impl From<Unit> for printpdf::Pt {
    fn from(value: Unit) -> Self {
        printpdf::Pt::from(printpdf::Mm::from(value))
    }
}

impl From<&Unit> for printpdf::Pt {
    fn from(value: &Unit) -> Self {
        printpdf::Pt::from(printpdf::Mm::from(value))
    }
}

pub trait NumericUnit {
    fn inches(self) -> Unit;
    fn mm(self) -> Unit;
}

impl<T> NumericUnit for T
where
    T: Into<f64>,
{
    fn inches(self) -> Unit {
        Unit(self.into() * 25.4)
    }

    fn mm(self) -> Unit {
        Unit(self.into())
    }
}

impl std::ops::Add for Unit {
    type Output = Unit;

    fn add(self, rhs: Self) -> Self::Output {
        Unit(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Unit {
    type Output = Unit;

    fn sub(self, rhs: Self) -> Self::Output {
        Unit(self.0 - rhs.0)
    }
}

impl<T> std::ops::Mul<T> for Unit
where
    T: Into<f64>,
{
    type Output = Unit;

    fn mul(self, rhs: T) -> Self::Output {
        Unit(self.0 * rhs.into())
    }
}

impl std::ops::Div for Unit {
    // Dividing a Unit by a Unit removes the unit and returns a ratio.
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl<T> std::ops::Div<T> for Unit
where
    T: Into<f64>,
{
    // Dividing a Unit by a number returns a new Unit
    type Output = Unit;

    fn div(self, rhs: T) -> Self::Output {
        Unit(self.0 / rhs.into())
    }
}

impl std::ops::Neg for Unit {
    type Output = Unit;

    fn neg(self) -> Self::Output {
        Unit(-(self.0))
    }
}

impl std::ops::Neg for &Unit {
    type Output = Unit;

    fn neg(self) -> Self::Output {
        Unit(-(self.0))
    }
}
