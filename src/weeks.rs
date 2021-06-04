/**
 Describe all of the weeks between two dates.

 Options:
 - just give it a month
 - ask for leading/trailing days from other months

**/
use chrono::{Date, Datelike, Duration, Local, TimeZone, Weekday};

type LocalDate = Date<Local>;

#[derive(Debug, Clone)]
pub struct DayDesc {
    date: LocalDate,
}

pub type WeekDesc = [Option<DayDesc>; 7];

pub fn weeks_for_month(date: LocalDate) -> Vec<WeekDesc> {
    let month_start = Local.ymd(date.year(), date.month(), 1);
    let month_week = start_of_week(month_start);

    let mut result = Vec::new();

    let mut week_start = month_week;
    // Check to see if the beginning or end of the week is still in the month.
    while week_start.month() == date.month()
        || week_start
            .checked_add_signed(Duration::days(6))
            .unwrap()
            .month()
            == date.month()
    {
        result.push(week_with_date(week_start));

        week_start = week_start.checked_add_signed(Duration::weeks(1)).unwrap();
    }

    result
}

fn empty_week() -> WeekDesc {
    [None, None, None, None, None, None, None]
}

fn week_with_date(date: LocalDate) -> WeekDesc {
    // Find the start of the week, and then return the next 7 days.
    let start = start_of_week(date);
    let dates = (0..7)
        .map(|offset| start.checked_add_signed(Duration::days(offset)).unwrap())
        .map(|d| DayDesc { date: d });

    let mut result = empty_week();

    for (idx, dd) in dates.enumerate() {
        result[idx] = Some(dd);
    }

    result
}

fn start_of_week(date: LocalDate) -> LocalDate {
    // Look back up to 6 days for a Sunday.
    (0..7)
        .map(|offset| date.checked_sub_signed(Duration::days(offset)).unwrap())
        .find(|d| d.weekday() == Weekday::Sun)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{Local, TimeZone};

    fn ld(year: i32, month: u32, day: u32) -> LocalDate {
        Local.ymd(year, month, day)
    }

    #[test]
    fn start_of_week__cross_month() {
        let day_in_week = ld(2021, 6, 4);
        let start = ld(2021, 5, 30);

        assert_eq!(start, start_of_week(day_in_week));
    }
    #[test]
    fn start_of_week__cross_year() {
        let day_in_week = ld(2022, 1, 1);
        let start = ld(2021, 12, 26);

        assert_eq!(start, start_of_week(day_in_week));
    }
}
