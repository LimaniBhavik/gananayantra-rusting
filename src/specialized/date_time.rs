use chrono::{Datelike, NaiveDate};

pub fn calculate_age(birth_date: NaiveDate, current_date: NaiveDate) -> i32 {
    let mut years = current_date.year() - birth_date.year();
    if current_date.month() < birth_date.month()
        || (current_date.month() == birth_date.month() && current_date.day() < birth_date.day())
    {
        years -= 1;
    }
    years
}

pub fn date_difference_days(date1: NaiveDate, date2: NaiveDate) -> i64 {
    (date2 - date1).num_days()
}

pub fn hours_to_minutes_seconds(hours: f64) -> (f64, f64) {
    (hours * 60.0, hours * 3600.0)
}

pub fn get_day_of_week(date: NaiveDate) -> String {
    format!("{:?}", date.weekday())
}
