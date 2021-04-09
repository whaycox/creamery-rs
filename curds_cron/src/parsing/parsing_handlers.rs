use super::super::*;
use regex::*;
use lazy_static::*;

pub struct SingleValueHandler;
impl CronValueParsingHandler for SingleValueHandler {
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<CronValue> { 
        let single_value = date_part.translate(value).parse::<u32>().unwrap();
        if single_value < date_part.min() {
            panic!("{} is less than allowed", single_value);
        }
        if single_value > date_part.max() {
            panic!("{} is greater than allowed", single_value);
        }
        Some(CronValue::Single(single_value))
    }
}

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
            
            return Some(CronValue::Range(min_value, max_value))
        }
        None
    }
}

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
            return Some(CronValue::NearestWeekday(*value))
        }
        None
    }
}

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
                return Some(CronValue::LastDayOfMonth(offset_value))
            }
            return Some(CronValue::LastDayOfMonth(0))
        }
        None
    }
}

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
            
            return Some(CronValue::NthDayOfWeek(value, n))
        }
        None
    }
}

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
            return Some(CronValue::LastDayOfWeek(value))
        }
        None
    }
}