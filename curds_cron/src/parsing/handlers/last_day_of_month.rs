use super::*;

pub struct LastDayOfMonthValueHandler;
impl CronValueParsingHandler for LastDayOfMonthValueHandler {
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<CronValue> { 
        lazy_static! {
            static ref LAST_DAY_OF_MONTH_REGEX: Regex = Regex::new(r"^[Ll](?:-(\d+))?$").unwrap();
        }
        if let Some(captures) = LAST_DAY_OF_MONTH_REGEX.captures(value) {
            if let Some(offset) = captures.get(1) {
                let offset_value = offset.as_str().parse::<u32>().unwrap();
                if offset_value - 1 >= date_part.max() {
                    panic!("Cannot supply offset larger than {}", date_part.max())
                }
                return Some(CronValue::LastDayOfMonth { offset: offset_value })
            }
            return Some(CronValue::LastDayOfMonth { offset: 0 })
        }
        None
    }
}