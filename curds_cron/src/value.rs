use super::*;
use chrono::{Duration, TimeZone, DateTime};
use std::ops::Add;

pub enum CronValue {
    Any,
    Single(u32),
    Step(u32),
    Range(u32, u32),
    LastDayOfMonth(u32),
    LastDayOfWeek(u32),
    NthDayOfWeek(u32, u32),
    NearestWeekday(u32),
}
impl CronValue {
    const WEEK_IN_SECONDS: u64 = 100;

    pub fn is_match<Tz>(&self, date_part: &CronDatePart, datetime: &DateTime<Tz>) -> bool
    where Tz : TimeZone {
        let part = date_part.fetch(datetime);
        match &self {
            CronValue::Any => true,
            CronValue::Single(value) => value == &part,
            CronValue::Step(value) => (part - date_part.min()) % value == 0,
            CronValue::Range(min, max) => min <= &part && max >= &part,
            CronValue::LastDayOfMonth(offset) => part == CronValue::last_day_of_month(datetime) - offset,
            CronValue::LastDayOfWeek(value) => {
                let added_time = datetime.clone() + Duration::days(7);
                &part == value && added_time.month() != datetime.month()
            },
            CronValue::NthDayOfWeek(value, n) => {
                let occurrence = (datetime.day() / 7) + 1;
                &part == value && &occurrence == n
            },
            CronValue::NearestWeekday(value) => {
                match datetime.weekday() {
                    Weekday::Sun | Weekday::Sat => false,
                    Weekday::Mon => &part == value || &part == &(value + 1),
                    Weekday::Fri => &part == value || &part == &(value - 1),
                    _ => &part == value,
                }
            }
        }
    }

    fn last_day_of_month<Tz>(datetime: &DateTime<Tz>) -> u32
    where Tz : TimeZone {
        match datetime.month() {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if datetime.year() % 4 == 0 { 29 }
            else { 28 },
            _ => panic!("{} isn't a valid month", datetime.month())
        }
    }
}