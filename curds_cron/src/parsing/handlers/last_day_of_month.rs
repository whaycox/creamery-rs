use super::*;

pub struct LastDayOfMonthValueHandler;
impl CronValueParsingHandler for LastDayOfMonthValueHandler {
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<Result<CronValue, CronParsingError>> { 
        lazy_static! {
            static ref LAST_DAY_OF_MONTH_REGEX: Regex = Regex::new(r"^[Ll](?:-(\d+))?$").unwrap();
        }
        if let Some(captures) = LAST_DAY_OF_MONTH_REGEX.captures(value) {
            if let Some(offset) = captures.get(1) {
                let offset_value = offset.as_str().parse::<u32>().unwrap();
                let max_offset = date_part.max() - 1;
                if offset_value > max_offset {
                    return Some(Err(CronParsingError::ValueOutOfBounds {
                        raw_value: value.to_owned(),
                        allowed: max_offset,
                        supplied: offset_value,
                        date_part: *date_part,
                    }))
                }
                return Some(Ok(CronValue::LastDayOfMonth { offset: offset_value }))
            }
            return Some(Ok(CronValue::LastDayOfMonth { offset: 0 }))
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_without_offset() {
        let test_object = LastDayOfMonthValueHandler;

        let actual = test_object.parse(&CronDatePart::DayOfMonth, "L")
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::LastDayOfMonth { offset: 0 }, actual);
    }

    #[test]
    fn captures_with_offset() {
        let test_object = LastDayOfMonthValueHandler;

        let actual = test_object.parse(&CronDatePart::DayOfMonth, "l-30")
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::LastDayOfMonth { offset: 30 }, actual);
    }

    #[test]
    fn nonmatch_returns_none() {
        let test_object = LastDayOfMonthValueHandler;

        assert_eq!(None, test_object.parse(&CronDatePart::DayOfMonth, "DayOfMonth"));
    }

    #[test]
    fn offset_larger_than_allowed_returns_error() {
        let test_object = LastDayOfMonthValueHandler;

        test_object.parse(&CronDatePart::DayOfMonth, "L-31")
            .unwrap()
            .expect_err("Expected an offset that is out of bounds: ");
    }
}