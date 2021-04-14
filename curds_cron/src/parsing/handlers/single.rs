use super::*;

pub struct SingleValueHandler;
impl CronValueParsingHandler for SingleValueHandler {
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<Result<CronValue, CronParsingError>> {
        if let Ok(single_value) = date_part.translate(value).parse::<u32>() {
            if single_value < date_part.min() {
                return Some(Err(CronParsingError::ValueOutOfBounds {
                    raw_value: value.to_owned(),
                    supplied: single_value,
                    allowed: date_part.min(),
                    date_part: *date_part,
                }))
            }
            if single_value > date_part.max() {
                return Some(Err(CronParsingError::ValueOutOfBounds {
                    raw_value: value.to_owned(),
                    supplied: single_value,
                    allowed: date_part.max(),
                    date_part: *date_part,
                }))
            }
            Some(Ok(CronValue::Single(single_value)))
        }
        else {
            return Some(Err(CronParsingError::ParsedValue {
                value: value.to_owned(),
                date_part: *date_part,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_value_as_int() {
        let test_object = SingleValueHandler;

        let actual = test_object.parse(&CronDatePart::Hours, "6")
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::Single(6), actual);
    }

    #[test]
    fn translates_month() {
        let test_object = SingleValueHandler;

        let actual = test_object.parse(&CronDatePart::Month, "apr")
            .unwrap()
            .unwrap();
        
        assert_eq!(CronValue::Single(4), actual);
    }

    #[test]
    fn translates_day_of_week() {
        let test_object = SingleValueHandler;

        let actual = test_object.parse(&CronDatePart::DayOfWeek, "thu")
            .unwrap()
            .unwrap();
        
        assert_eq!(CronValue::Single(4), actual);
    }

    #[test]
    fn less_than_min_returns_error() {
        let test_object = SingleValueHandler;

        test_object.parse(&CronDatePart::Month, "0")
            .unwrap()
            .expect_err("Expected an error:");
    }

    #[test]
    fn greater_than_max_returns_error() {
        let test_object = SingleValueHandler;

        test_object.parse(&CronDatePart::Month, "13")
            .unwrap()
            .expect_err("Expected an error:");
    }

    #[test]
    fn nonint_returns_error() {
        let test_object = SingleValueHandler;

        test_object.parse(&CronDatePart::Month, "Blah")
            .unwrap()
            .expect_err("Expected an error trying to parse:");
    }

}