use super::*;

pub struct LastDayOfWeekValueHandler;
impl CronValueParsingHandler for LastDayOfWeekValueHandler {
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<CronValue> { 
        lazy_static! {
            static ref LAST_DAY_OF_WEEK_REGEX: Regex = Regex::new(r"^([a-zA-Z0-6]{1,3})[lL]$").unwrap();
        }
        if let Some(captures) = LAST_DAY_OF_WEEK_REGEX.captures(value) {
            let value = date_part.translate(&captures[1]).parse::<u32>().unwrap();
            if value < date_part.min() {
                panic!("{} is less than allowed", value);
            }
            if value > date_part.max() {
                panic!("{} is greater than allowed", value);
            }
            return Some(CronValue::LastDayOfWeek { day_of_week: value })
        }
        None
    }
}