mod datetools;
mod grid;
mod pdfutils;
mod shapes;
mod tgrid;
mod units;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WeeklyError {}

pub type Result<T> = std::result::Result<T, WeeklyError>;

pub use datetools::{today, Datetools};
pub use grid::Builder;
pub use tgrid::{GridDescription, TGrid};
pub use pdfutils::{Colors, Instructions, LineModifiers};
pub use shapes::line::WLine;
pub use shapes::rect::WRect;
pub use shapes::AsPdfLine;
pub use units::{NumericUnit, Unit};
