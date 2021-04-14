use super::*;

pub struct WildcardValueHandler;
impl CronValueParsingHandler for WildcardValueHandler {  
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<Result<CronValue, CronParsingError>> { 
        lazy_static! {
            static ref WILDCARD_REGEX: Regex = Regex::new(r"^\*(?:/(\d+))?$").unwrap();
        }
        if let Some(captures) = WILDCARD_REGEX.captures(value) {
            if let Some(range_capture) = captures.get(1) {
                let step_value = range_capture.as_str().parse::<u32>().unwrap();
                if step_value <= 1 {
                    return Some(Err(CronParsingError::ValueOutOfBounds {
                        raw_value: value.to_owned(),
                        supplied: step_value,
                        allowed: 2,
                        date_part: *date_part,
                    }))
                }
                if date_part.min() + step_value > date_part.max() {
                    return Some(Err(CronParsingError::ValueOutOfBounds {
                        raw_value: value.to_owned(),
                        supplied: step_value,
                        allowed: date_part.max() - date_part.min(),
                        date_part: *date_part,
                    }))
                }
                return Some(Ok(CronValue::Step(step_value)))
            }
            return Some(Ok(CronValue::Any))
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_returns_any() {
        let test_object = WildcardValueHandler;

        let actual = test_object.parse(&CronDatePart::DayOfWeek, "*")
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::Any, actual);
    }

    #[test]
    fn parses_step_if_supplied() {
        let test_object = WildcardValueHandler;

        let actual = test_object.parse(&CronDatePart::DayOfWeek, "*/3")
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::Step(3), actual);
    }

    #[test]
    fn non_match_returns_none() {
        let test_object = WildcardValueHandler;

        assert_eq!(None, test_object.parse(&CronDatePart::Hours, "Hours"));
    }

    #[test]
    fn step_smaller_than_allowed_returns_error() {
        let test_object = WildcardValueHandler;

        test_object.parse(&CronDatePart::Month, "*/1")
            .unwrap()
            .expect_err("Expected a step of 1 to error:");
    }

    #[test]
    fn step_larger_than_allowed_returns_error() {
        let test_object = WildcardValueHandler;

        test_object.parse(&CronDatePart::Minutes, "*/60")
            .unwrap()
            .expect_err("Expected a step of 60 to error for Minutes:");
        test_object.parse(&CronDatePart::Month, "*/12")
            .unwrap()
            .expect_err("Expected a step of 12 to error for Months:");
    }
}