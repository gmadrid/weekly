mod datetools;
mod grid;
mod pdfutils;
mod shapes;
mod units;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WeeklyError {}

pub type Result<T> = std::result::Result<T, WeeklyError>;

pub use datetools::{today, Datetools};
pub use grid::Builder;
pub use pdfutils::Colors;
pub use pdfutils::Instructions;
pub use shapes::line::WLine;
pub use shapes::rect::WRect;
pub use units::{NumericUnit, Unit};
