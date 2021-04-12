use super::*;

pub struct NthDayOfWeekValueHandler;
impl CronValueParsingHandler for NthDayOfWeekValueHandler {
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<CronValue> {  
        lazy_static! {
            static ref NTH_DAY_OF_WEEK_REGEX: Regex = Regex::new(r"^([a-zA-Z0-6]{1,3})#([1-5])$").unwrap();
        }
        if let Some(captures) = NTH_DAY_OF_WEEK_REGEX.captures(value) {
            let value = date_part.translate(&captures[1]).parse::<u32>().unwrap();
            if value < date_part.min() {
                panic!("{} is less than allowed", value);
            }
            if value > date_part.max() {
                panic!("{} is greater than allowed", value);
            }
            let n = captures[2].parse::<u32>().unwrap();
            
            return Some(CronValue::NthDayOfWeek { day_of_week: value, n: n })
        }
        None
    }
}