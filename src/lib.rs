use thiserror::Error;

pub use datetools::{today, Datetools};
pub use pdfutils::sizes;
pub use pdfutils::{
    save_one_page_document, Attributes, Colors, FontProxy, Instructions, LineModifiers,
};
pub use shapes::line::WLine;
pub use shapes::rect::WRect;
pub use shapes::AsPdfLine;
pub use tgrid::description::GridDescription;
pub use tgrid::TGrid;
pub use units::{NumericUnit, Unit};

mod datetools;
mod pdfutils;
mod shapes;
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
