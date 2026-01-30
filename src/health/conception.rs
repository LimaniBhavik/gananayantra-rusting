use chrono::{NaiveDate, Duration};

pub fn estimate_conception_date(lmp: NaiveDate) -> NaiveDate {
    lmp + Duration::days(14)
}
