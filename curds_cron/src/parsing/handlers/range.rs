use super::*;

pub struct RangeValueHandler;
impl CronValueParsingHandler for RangeValueHandler {
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<Result<CronValue, CronParsingError>> { 
        lazy_static! {
            static ref RANGE_REGEX: Regex = Regex::new(r"^([0-9a-zA-Z]{1,3})-([0-9a-zA-Z]{1,3})$").unwrap();
        }
        if let Some(captures) = RANGE_REGEX.captures(value) {
            let min = date_part.translate(&captures[1]);
            if let Ok(min_value) = min.parse::<u32>() {
                let min_bound = date_part.min();
                if min_value < min_bound {
                    return Some(Err(CronParsingError::ValueOutOfBounds {
                        raw_value: value.clone().to_owned(),
                        supplied: min_value,
                        allowed: min_bound,
                        date_part: *date_part,
                    }))
                }
        
                let max = date_part.translate(&captures[2]);
                if let Ok(max_value) = max.parse::<u32>() {
                    let max_bound = date_part.max();
                    if max_value > max_bound {
                        return Some(Err(CronParsingError::ValueOutOfBounds {
                            raw_value: value.clone().to_owned(),
                            supplied: max_value,
                            allowed: max_bound,
                            date_part: *date_part,
                        }))
                    }
        
                    if min_value > max_value {
                        return Some(Err(CronParsingError::InvertedRange {
                            min: min_value,
                            max: max_value,
                        }))
                    }
                    
                    return Some(Ok(CronValue::Range { 
                        min: min_value, 
                        max: max_value 
                    }))
                }
                else {
                    return Some(Err(CronParsingError::ParsedValue {
                        value: max.clone().to_owned(),
                        date_part: *date_part,
                    }))
                }
            }
            else {
                return Some(Err(CronParsingError::ParsedValue {
                    value: min.clone().to_owned(),
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
    fn parses_range_as_int() {
        let test_object = RangeValueHandler;

        let actual = test_object.parse(&CronDatePart::Minutes, "4-8")
            .unwrap()
            .unwrap();
        
        assert_eq!(CronValue::Range { min: 4, max: 8 }, actual);
    }

    #[test]
    fn translates_min() {
        let test_object = RangeValueHandler;

        let actual = test_object.parse(&CronDatePart::Month, "MAR-6")
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::Range { min: 3, max: 6 }, actual);
    }

    #[test]
    fn translates_max() {
        let test_object = RangeValueHandler;

        let actual = test_object.parse(&CronDatePart::DayOfWeek, "4-sat")
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::Range { min: 4, max: 6 }, actual);
    }

    #[test]
    fn nonmatch_return_none() {
        let test_object = RangeValueHandler;

        assert_eq!(None, test_object.parse(&CronDatePart::DayOfWeek, "DayOfWeek"));
    }

    #[test]
    fn nonint_min_returns_error() {
        let test_object = RangeValueHandler;

        test_object.parse(&CronDatePart::DayOfWeek, "OEU-5")
            .unwrap()
            .expect_err("Expected min to fail parsing: ");
    }

    #[test]
    fn nonint_max_returns_error() {
        let test_object = RangeValueHandler;

        test_object.parse(&CronDatePart::DayOfWeek, "03-OEU")
            .unwrap()
            .expect_err("Expected max to fail parsing: ");
    }

    #[test]
    fn inverted_range_returns_error() {
        let test_object = RangeValueHandler;

        test_object.parse(&CronDatePart::DayOfWeek, "THU-MON")
            .unwrap()
            .expect_err("Expected inverted range to error: ");
    }
}