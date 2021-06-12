use chrono::Datelike;
use crate::LocalDate;

pub trait CalRenderer {
    fn render_cal_for_date_in_month(&self, date: LocalDate) {
	let weeks = crate::weeks_for_month(date);

	self.render_month_name(date);

	// Use the second week, since it will always have 7 days.
	for (idx, day) in weeks[1].iter().enumerate() {
	    self.render_day_name(idx, day.date);
	}

	for (week_idx, week) in weeks.iter().enumerate() {
	    for (day_idx, day) in week.iter().enumerate() {
		self.render_date(week_idx, day_idx, day.date);
	    }
	}
	    
    }
    fn render_month_name(&self, date: LocalDate);
    fn render_day_name(&self, col_index: usize, date: LocalDate);
    fn render_date(&self, row_index: usize, col_index: usize, date: LocalDate);
}

pub struct TextRenderer;

impl CalRenderer for TextRenderer {
    fn render_month_name(&self, date: LocalDate) {
	println!("{:^27}", date.format("%b %Y"));
    }
    fn render_day_name(&self, _col_index: usize, date: LocalDate) {
	// Assumes that the month name has rendered a newline.
	print!("{}", date.format("%a "));
    }
    fn render_date(&self, _row_index: usize, col_index: usize, date: LocalDate) {
	// Assumes that the previous row has _not_ rendered a newline, so a
	// new row needs a new line.
	if col_index == 0 {
	    println!();
	}

	print!("{:3} ", date.day());
    }
}

pub fn print_cal_for_month(date: crate::LocalDate, _prune: bool) {
    let tr = TextRenderer;

    tr.render_cal_for_date_in_month(date);
}

