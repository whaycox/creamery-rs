use super::*;

pub struct NthDayOfWeekValueHandler;
impl CronValueParsingHandler for NthDayOfWeekValueHandler {
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<Result<CronValue, CronParsingError>> {  
        lazy_static! {
            static ref NTH_DAY_OF_WEEK_REGEX: Regex = Regex::new(r"^([a-zA-Z0-6]{1,3})#([1-5])$").unwrap();
        }
        if let Some(captures) = NTH_DAY_OF_WEEK_REGEX.captures(value) {
            if let Ok(parsed_value) = date_part.translate(&captures[1]).parse::<u32>() {
                if parsed_value > date_part.max() {
                    return Some(Err(CronParsingError::ValueOutOfBounds {
                        raw_value: value.to_owned(),
                        allowed: date_part.max(),
                        supplied: parsed_value,
                        date_part: *date_part,
                    }))
                }
                let n = captures[2].parse::<u32>().unwrap();                
                return Some(Ok(CronValue::NthDayOfWeek { 
                    day_of_week: parsed_value, 
                    n: n 
                }))
            }
            else {
                return Some(Err(CronParsingError::ParsedValue {
                    value: value.to_owned(),
                    date_part: *date_part,
                }))
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_values_correctly() {
        let test_object = NthDayOfWeekValueHandler;

        let actual = test_object.parse(&CronDatePart::DayOfWeek, "SUN#2")
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::NthDayOfWeek { day_of_week: 0, n: 2}, actual);
    }

    #[test]
    fn nonmatch_returns_none() {
        let test_object = NthDayOfWeekValueHandler;

        assert_eq!(None, test_object.parse(&CronDatePart::DayOfWeek, "DayOfWeek"));
    }

    #[test]
    fn value_greater_than_max_returns_error() {
        let test_object = NthDayOfWeekValueHandler;

        test_object.parse(&CronDatePart::DayOfWeek, "60#2")
            .unwrap()
            .expect_err("Expected an out of bounds value: ");        
    }
    
    #[test]
    fn unparseable_returns_error() {
        let test_object = NthDayOfWeekValueHandler;

        test_object.parse(&CronDatePart::DayOfWeek, "OEU#2")
            .unwrap()
            .expect_err("Expected an unparseable value: ");
    }
}