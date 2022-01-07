use chrono::{Datelike, NaiveDate, Weekday};
use printpdf::{BuiltinFont, IndirectFontRef, PdfDocumentReference};
use std::borrow::Cow;
use weekly::{save_one_page_document, sizes, today, Datetools, NumericUnit, TGrid, Unit, WRect};
use weekly::{GridDescription, Instructions};

mod data {
    use chrono::Weekday;
    use chrono::Weekday::{Fri, Mon, Thu, Tue, Wed};
    use lazy_static::lazy_static;
    use std::collections::HashSet;

    #[derive(Default, Debug)]
    pub struct DailyTask<'a> {
        pub name: &'a str,
        // No set means ALL days. Empty set means NO days.
        days: Option<HashSet<Weekday>>,
    }

    fn weekdays_only() -> HashSet<Weekday> {
        vec![Mon, Tue, Wed, Thu, Fri].into_iter().collect()
    }

    lazy_static! {
        pub static ref TASKS: Vec<DailyTask<'static>> = {
            vec![
                DailyTask {
                    name: "Plank",
                    days: None,
                },
                DailyTask {
                    name: "Door stretch",
                    days: None,
                },
                DailyTask {
                    name: "Walk",
                    days: None,
                },
                DailyTask {
                    name: "",
                    days: None,
                },
                DailyTask {
                    name: "",
                    days: None,
                },
                DailyTask {
                    name: "Journal",
                    days: None,
                },
                DailyTask {
                    name: "Virtuemap",
                    days: None,
                },
                DailyTask {
                    name: "Add item to bucket list",
                    days: None,
                },
                DailyTask {
                    name: "",
                    days: None,
                },
                DailyTask {
                    name: "",
                    days: None,
                },
                DailyTask {
                    name: "Check calendar",
                    days: None,
                },
                DailyTask {
                    name: "Check ToDo list",
                    days: None,
                },
                DailyTask {
                    name: "",
                    days: None,
                },
                DailyTask {
                    name: "Brush teeth",
                    days: None,
                },
                DailyTask {
                    name: "Floss",
                    days: None,
                },
                DailyTask {
                    name: "",
                    days: None,
                },
                DailyTask {
                    name: "",
                    days: None,
                },
                DailyTask {
                    name: "Knit",
                    days: None,
                },
                DailyTask {
                    name: "Magic",
                    days: None,
                },
                DailyTask {
                    name: "Chess",
                    days: None,
                },
                DailyTask {
                    name: "",
                    days: None,
                },
                DailyTask {
                    name: "",
                    days: None,
                },
                DailyTask {
                    name: "Bug sweep",
                    days: Some(weekdays_only()),
                },
                DailyTask {
                    name: "Code reviews",
                    days: Some(weekdays_only()),
                },
                DailyTask {
                    name: "Inbox Zero",
                    days: Some(weekdays_only()),
                },
            ]
        };
    }
}

struct DailyDescription {
    bounds: WRect,
    dates_in_month: Vec<NaiveDate>,
    font: IndirectFontRef,
}

impl DailyDescription {
    pub fn for_month<DL>(date: &DL, bounds: WRect, font: IndirectFontRef) -> DailyDescription
    where
        DL: Datelike,
    {
        DailyDescription {
            bounds,
            dates_in_month: date.dates_in_month(),
            font,
        }
    }
}

impl GridDescription for DailyDescription {
    fn bounds(&self) -> WRect {
        self.bounds.clone()
    }

    fn num_rows(&self) -> Option<usize> {
        Some(self.dates_in_month.len())
    }

    fn num_cols(&self) -> Option<usize> {
        Some(25)
    }

    fn row_label_width(&self) -> Option<Unit> {
        Some(1.0.inches())
    }

    fn col_label_height(&self) -> Option<Unit> {
        Some(2.0.inches())
    }

    fn row_label(&self, index: usize) -> Cow<'static, str> {
        self.dates_in_month[index]
            .format("%b %e")
            .to_string()
            .into()
    }

    fn col_label(&self, index: usize) -> Cow<'static, str> {
        if index < data::TASKS.len() {
            data::TASKS[index].name.into()
        } else {
            "".into()
        }
    }

    fn horiz_line_width(&self, row: usize) -> f64 {
        if row < self.dates_in_month.len() {
            if self.dates_in_month[row].weekday() == Weekday::Sun {
                1.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    fn vert_line_width(&self, col: usize) -> f64 {
        if col > 0 && col % 5 == 0 {
            1.0
        } else {
            0.0
        }
    }

    fn font(&self) -> &IndirectFontRef {
        &self.font
    }
}

fn render_dailies(doc: &PdfDocumentReference, page_rect: &WRect) -> Instructions {
    let grid_rect =
        page_rect.inset_all_q1(0.5.inches(), 0.25.inches(), 0.25.inches(), 0.25.inches());
    let font = doc.add_builtin_font(BuiltinFont::TimesBold).unwrap();
    let description = DailyDescription::for_month(&today(), grid_rect, font);
    let grid = TGrid::with_description(description);
    grid.generate_instructions()
}

fn main() {
    save_one_page_document("Quux", "quux.pdf", &sizes::letter(), render_dailies);
}
