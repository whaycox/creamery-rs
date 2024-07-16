use super::*;

const NEAREST_WEEKDAY_PATTERN: &str = r"^(\d{1,2})[Ww]$";
static NEAREST_WEEKDAY_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn parse_nearest_weekday(value: &str, field_type: &CronFieldType) -> Option<Result<CronValue, CronParsingError>> {
    if let Some(captures) = NEAREST_WEEKDAY_REGEX.get_or_init(|| Regex::new(NEAREST_WEEKDAY_PATTERN).unwrap()).captures(value) {
        let parsed_value = &captures[1].parse::<u32>().unwrap();
        if *parsed_value < field_type.min() {
            return Some(Err(CronParsingError::ValueOutOfBounds {
                value: value.to_owned(),
                allowed: field_type.min(),
                field_type: field_type.clone(),
            }))
        }
        if *parsed_value > field_type.max() {
            return Some(Err(CronParsingError::ValueOutOfBounds {
                value: value.to_owned(),
                allowed: field_type.max(),
                field_type: field_type.clone(),
            }))
        }
        return Some(Ok(CronValue::NearestWeekday { day_of_month: *parsed_value }))
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_value_as_int() {
        let actual = parse_nearest_weekday("20w", &CronFieldType::DayOfMonth)
            .unwrap()
            .unwrap();
        
        assert_eq!(CronValue::NearestWeekday { day_of_month: 20}, actual);
    }

    #[test]
    fn nonmatch_returns_none() {
        assert_eq!(None, parse_nearest_weekday("DayOfMonth", &CronFieldType::DayOfMonth));
    }

    #[test]
    fn value_larger_than_max_returns_error() {
        match parse_nearest_weekday("32W", &CronFieldType::DayOfMonth).unwrap() {
            Err(CronParsingError::ValueOutOfBounds { value, allowed, field_type }) => {
                assert_eq!("32W", value);
                assert_eq!(31, allowed);
                assert_eq!(CronFieldType::DayOfMonth, field_type);
            },
            _ => panic!("Did not get expected error"),
        }
    }

    #[test]
    fn value_less_than_min_returns_error() {
        match parse_nearest_weekday("0w", &CronFieldType::DayOfMonth).unwrap() {
            Err(CronParsingError::ValueOutOfBounds { value, allowed, field_type }) => {
                assert_eq!("0w", value);
                assert_eq!(1, allowed);
                assert_eq!(CronFieldType::DayOfMonth, field_type);
            },
            _ => panic!("Did not get expected error"),
        }
    }
}