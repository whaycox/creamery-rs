use super::*;

pub struct NearestWeekdayValueHandler;
impl CronValueParsingHandler for NearestWeekdayValueHandler {
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<Result<CronValue, CronParsingError>> { 
        lazy_static! {
            static ref NEAREST_WEEKDAY_REGEX: Regex = Regex::new(r"^(\d{1,2})[Ww]$").unwrap();
        }
        if let Some(captures) = NEAREST_WEEKDAY_REGEX.captures(value) {
            let parsed_value = &captures[1].parse::<u32>().unwrap();
            if parsed_value < &date_part.min() {
                return Some(Err(CronParsingError::ValueOutOfBounds {
                    raw_value: value.to_owned(),
                    allowed: date_part.min(),
                    supplied: *parsed_value,
                    date_part: *date_part,
                }))
            }
            if parsed_value > &date_part.max() {
                return Some(Err(CronParsingError::ValueOutOfBounds {
                    raw_value: value.to_owned(),
                    allowed: date_part.max(),
                    supplied: *parsed_value,
                    date_part: *date_part,
                }))
            }
            return Some(Ok(CronValue::NearestWeekday { day_of_month: *parsed_value }))
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_value_as_int() {
        let test_object = NearestWeekdayValueHandler;

        let actual = test_object.parse(&CronDatePart::DayOfMonth, "20w")
            .unwrap()
            .unwrap();
        
        assert_eq!(CronValue::NearestWeekday { day_of_month: 20}, actual);
    }

    #[test]
    fn nonmatch_returns_none() {
        let test_object = NearestWeekdayValueHandler;

        assert_eq!(None, test_object.parse(&CronDatePart::DayOfMonth, "DayOfMonth"));
    }

    #[test]
    fn value_larger_than_max_returns_error() {
        let test_object = NearestWeekdayValueHandler;

        test_object.parse(&CronDatePart::DayOfMonth, "32W")
            .unwrap()
            .expect_err("Expected a value out of bounds");
    }

    #[test]
    fn value_less_than_min_returns_error() {
        let test_object = NearestWeekdayValueHandler;

        test_object.parse(&CronDatePart::DayOfMonth, "0w")
            .unwrap()
            .expect_err("Expected a value out of bounds");
    }
}