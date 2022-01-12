#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
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

    pub fn pct(self, percentage: f64) -> Unit {
        Unit(self.0 * (percentage / 100.0))
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

impl From<Unit> for f64 {
    fn from(unit: Unit) -> Self {
        unit.0
    }
}

impl From<f64> for Unit {
    fn from(f: f64) -> Self {
        Unit(f)
    }
}

pub trait NumericUnit {
    fn inches(self) -> Unit;
    fn mm(self) -> Unit;
}

impl NumericUnit for f64 {
    fn inches(self) -> Unit {
        Unit(self * 25.4)
    }
    fn mm(self) -> Unit {
        Unit(self)
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

impl std::ops::Mul<usize> for Unit {
    type Output = Unit;

    fn mul(self, rhs: usize) -> Self::Output {
        Unit(self.0 * rhs as f64)
    }
}

impl std::ops::Mul<f64> for Unit {
    type Output = Unit;

    fn mul(self, rhs: f64) -> Self::Output {
        Unit(self.0 * rhs)
    }
}

impl std::ops::Mul<Unit> for Unit {
    type Output = Unit;

    fn mul(self, rhs: Unit) -> Self::Output {
        Unit(self.0 * rhs.0)
    }
}

impl std::ops::Div for Unit {
    type Output = usize;

    fn div(self, rhs: Self) -> Self::Output {
        (self.0 / rhs.0).trunc() as Self::Output
    }
}

impl std::ops::Div<usize> for Unit {
    type Output = Unit;

    fn div(self, rhs: usize) -> Self::Output {
        Unit(self.0 / rhs as f64)
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
