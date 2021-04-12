use super::*;

pub struct RangeValueHandler;
impl CronValueParsingHandler for RangeValueHandler {
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<CronValue> { 
        lazy_static! {
            static ref RANGE_REGEX: Regex = Regex::new(r"^([0-9a-zA-Z]{1,3})-([0-9a-zA-Z]{1,3})$").unwrap();
        }
        if let Some(captures) = RANGE_REGEX.captures(value) {
            let min = date_part.translate(&captures[1]);    
            let min_value = min.parse::<u32>().unwrap();
            let min_bound = date_part.min();
            if min_value < min_bound {
                panic!("{} is less than the allowed {}", min_value, min_bound)
            }
    
            let max = date_part.translate(&captures[2]);
            let max_value = max.parse::<u32>().unwrap();
            let max_bound = date_part.max();
            if max_value > max_bound {
                panic!("{} is larger than allowed {}", max_value, max_bound)
            }

            if min_value > max_value {
                panic!("Cannot supply an inverted range of {}-{}", min_value, max_value);
            }
            
            return Some(CronValue::Range { min: min_value, max: max_value })
        }
        None
    }
}