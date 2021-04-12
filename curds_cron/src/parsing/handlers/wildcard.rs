use super::*;

pub struct WildcardValueHandler;
impl CronValueParsingHandler for WildcardValueHandler {  
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<CronValue> { 
        lazy_static! {
            static ref WILDCARD_REGEX: Regex = Regex::new(r"^\*(?:/(\d+))?$").unwrap();
        }
        if let Some(captures) = WILDCARD_REGEX.captures(value) {
            if let Some(range_capture) = captures.get(1) {
                let step_value = range_capture.as_str().parse::<u32>().unwrap();
                if date_part.min() + step_value > date_part.max() {
                    panic!("Cannot supply a step value of {}", step_value)
                }
                return Some(CronValue::Step(step_value))
            }
            return Some(CronValue::Any)
        }
        None
    }
}