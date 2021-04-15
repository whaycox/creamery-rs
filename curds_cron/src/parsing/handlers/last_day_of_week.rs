use super::*;

pub struct LastDayOfWeekValueHandler;
impl CronValueParsingHandler for LastDayOfWeekValueHandler {
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<Result<CronValue, CronParsingError>> { 
        lazy_static! {
            static ref LAST_DAY_OF_WEEK_REGEX: Regex = Regex::new(r"^([a-zA-Z0-6]{1,3})[lL]$").unwrap();
        }
        if let Some(captures) = LAST_DAY_OF_WEEK_REGEX.captures(value) {
            if let Ok(parsed_value) = date_part.translate(&captures[1]).parse::<u32>() {
                if parsed_value > date_part.max() {
                    return Some(Err(CronParsingError::ValueOutOfBounds {
                        raw_value: value.to_owned(),
                        allowed: date_part.max(),
                        supplied: parsed_value,
                        date_part: *date_part,
                    }))
                }
                return Some(Ok(CronValue::LastDayOfWeek { day_of_week: parsed_value }))
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
    fn parses_day_of_week_as_int() {
        let test_object = LastDayOfWeekValueHandler;

        let actual = test_object.parse(&CronDatePart::DayOfWeek, "001L")
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::LastDayOfWeek { day_of_week: 1 }, actual);
    }

    #[test]
    fn translates_values() {
        let test_object = LastDayOfWeekValueHandler;

        let actual = test_object.parse(&CronDatePart::DayOfWeek, "sAtl")
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::LastDayOfWeek { day_of_week: 6 }, actual);
    }

    #[test]
    fn nonmatch_returns_none() {
        let test_object = LastDayOfWeekValueHandler;

        assert_eq!(None, test_object.parse(&CronDatePart::DayOfWeek, "DayOfWeek"));
    }

    #[test]
    fn value_larger_than_max_returns_error() {
        let test_object = LastDayOfWeekValueHandler;

        test_object.parse(&CronDatePart::DayOfWeek, "60l")
            .unwrap()
            .expect_err("Expected an out of bounds value: ");
    }

    #[test]
    fn unparseable_value_returns_error() {
        let test_object = LastDayOfWeekValueHandler;

        test_object.parse(&CronDatePart::DayOfWeek, "OEUL")
            .unwrap()
            .expect_err("Expected an unparseable value: ");
    }
}