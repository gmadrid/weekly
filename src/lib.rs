use thiserror::Error;

pub use datetools::{today, Datetools};
pub use instructions::{Attributes, Instructions};
pub use pdfutils::save_one_page_document;
pub use proxies::{ColorProxy, FontProxy};
pub use shapes::line::WLine;
pub use shapes::rect::WRect;
pub use tgrid::description::GridDescription;
pub use tgrid::TGrid;
pub use units::{NumericUnit, Unit};

mod datetools;
mod instructions;
mod pdfutils;
mod proxies;
mod shapes;
pub mod sizes;
mod tgrid;
mod units;

#[derive(Debug, Error)]
pub enum WeeklyError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("PrintPdf error: {0}")]
    PrintPdfError(#[from] printpdf::Error),
}

pub type Result<T> = std::result::Result<T, WeeklyError>;
