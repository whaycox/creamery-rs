use super::*;

pub struct NearestWeekdayValueHandler;
impl CronValueParsingHandler for NearestWeekdayValueHandler {
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<CronValue> { 
        lazy_static! {
            static ref NEAREST_WEEKDAY_REGEX: Regex = Regex::new(r"^(\d+)[Ww]$").unwrap();
        }
        if let Some(captures) = NEAREST_WEEKDAY_REGEX.captures(value) {
            let value = &captures[1].parse::<u32>().unwrap();
            if value < &date_part.min() {
                panic!("{} is less than allowed", value);
            }
            if value > &date_part.max() {
                panic!("{} is greater than allowed", value);
            }
            return Some(CronValue::NearestWeekday { day_of_month: *value })
        }
        None
    }
}