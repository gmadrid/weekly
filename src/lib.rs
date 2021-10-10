mod grid;
mod pdfutils;
mod shapes;
mod units;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WeeklyError {}

pub type Result<T> = std::result::Result<T, WeeklyError>;

pub use grid::table_grid;
pub use pdfutils::Colors;
pub use pdfutils::Instructions;
pub use shapes::line::WLine;
pub use shapes::rect::WRect;
pub use units::NumericUnit;

// mod shape;
//
//
//
//
//
// mod cal;
// //mod line;
// mod pdfutils;
// mod rect;
// mod weeks;
//
// pub use cal::print_cal_for_month;
// pub use line::PLine;
// pub use pdfutils::{inches_to_mm, point_pair};
// pub use rect::PRect;
// pub use weeks::{weeks_for_month, LocalDate, WeekDesc};
