#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Unit(pub(crate) f64);

impl Unit {
    pub fn zero() -> Unit {
        Unit(0.0)
    }
}

impl From<Unit> for printpdf::Mm {
    fn from(unit: Unit) -> Self {
        printpdf::Mm(unit.0)
    }
}

impl From<Unit> for f64 {
    fn from(unit: Unit) -> Self {
        unit.0
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

impl std::ops::Mul<u16> for Unit {
    type Output = Unit;

    fn mul(self, rhs: u16) -> Self::Output {
        Unit(self.0 * rhs as f64)
    }
}

impl std::ops::Div<u16> for Unit {
    type Output = Unit;

    fn div(self, rhs: u16) -> Self::Output {
        Unit(self.0 / rhs as f64)
    }
}
