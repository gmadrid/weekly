use chrono::{Datelike, Duration, Local, NaiveDate};

pub fn today() -> NaiveDate {
    Local::now().date().naive_local()
}

pub trait Datetools {
    fn as_naive_date(&self) -> NaiveDate;
    fn dates_in_month(&self) -> Vec<NaiveDate>;
    fn num_days_in_month(&self) -> i64;
    fn first_of_month(&self) -> NaiveDate;
    fn next_month(&self) -> NaiveDate;
    fn date_range(&self, num_days: i64) -> Vec<NaiveDate>;
}

impl<D> Datetools for D
where
    D: Datelike,
{
    fn as_naive_date(&self) -> NaiveDate {
        NaiveDate::from_ymd(self.year(), self.month(), self.day())
    }

    fn dates_in_month(&self) -> Vec<NaiveDate> {
        let first = self.first_of_month();
        let num_days = self.num_days_in_month();
        first.date_range(num_days)
        //(0..num_days).map(|n| first + Duration::days(n)).collect()
    }

    fn date_range(&self, num_days: i64) -> Vec<NaiveDate> {
        let first = self.as_naive_date();
        (0..num_days).map(|n| first + Duration::days(n)).collect()
    }

    fn num_days_in_month(&self) -> i64 {
        self.next_month()
            .signed_duration_since(NaiveDate::from_ymd(self.year(), self.month(), 1))
            .num_days()
    }

    fn first_of_month(&self) -> NaiveDate {
        NaiveDate::from_ymd(self.year(), self.month(), 1)
    }

    fn next_month(&self) -> NaiveDate {
        if self.month() == 12 {
            NaiveDate::from_ymd(self.year() + 1, 1, 1)
        } else {
            NaiveDate::from_ymd(self.year(), self.month() + 1, 1)
        }
    }
}
