use chrono::{Duration, NaiveDate};

pub fn estimate_due_date(lmp: NaiveDate) -> NaiveDate {
    lmp + Duration::days(280)
}
