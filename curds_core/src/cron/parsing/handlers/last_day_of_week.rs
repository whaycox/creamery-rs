use super::*;

const LAST_DAY_OF_WEEK_PATTERN: &str = r"^([a-zA-Z0-6]{1,3})[lL]$";
static LAST_DAY_OF_WEEK_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn parse_last_day_of_week(value: &str, field_type: &CronFieldType) -> Option<Result<CronValue, CronParsingError>> {
    if let Some(captures) = LAST_DAY_OF_WEEK_REGEX.get_or_init(|| Regex::new(LAST_DAY_OF_WEEK_PATTERN).unwrap()).captures(value) {
        if let Ok(parsed_value) = field_type.translate(&captures[1]).parse::<u32>() {
            if parsed_value > field_type.max() {
                return Some(Err(CronParsingError::ValueOutOfBounds {
                    value: value.to_owned(),
                    allowed: field_type.max(),
                    field_type: field_type.clone(),
                }))
            }
            return Some(Ok(CronValue::LastDayOfWeek { day_of_week: parsed_value }))
        }
        else {
            return Some(Err(CronParsingError::InvalidValue {
                value: value.to_owned(),
                field_type: field_type.clone(),
            }))
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_day_of_week_as_int() {
        let actual = parse_last_day_of_week("001L", &CronFieldType::DayOfWeek)
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::LastDayOfWeek { day_of_week: 1 }, actual);
    }

    #[test]
    fn translates_values() {
        let actual = parse_last_day_of_week("sAtl", &CronFieldType::DayOfWeek)
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::LastDayOfWeek { day_of_week: 6 }, actual);
    }

    #[test]
    fn nonmatch_returns_none() {
        assert_eq!(None, parse_last_day_of_week("DayOfWeek", &CronFieldType::DayOfWeek));
    }

    #[test]
    fn value_larger_than_max_returns_error() {
        match parse_last_day_of_week("60l", &CronFieldType::DayOfWeek).unwrap() {
            Err(CronParsingError::ValueOutOfBounds { value, allowed, field_type }) => {
                assert_eq!("60l", value);
                assert_eq!(6, allowed);
                assert_eq!(CronFieldType::DayOfWeek, field_type);
            },
            _ => panic!("Did not get expected error"),
        }
    }

    #[test]
    fn unparseable_value_returns_error() {
        match parse_last_day_of_week("OEUL", &CronFieldType::DayOfWeek).unwrap() {
            Err(CronParsingError::InvalidValue { value, field_type }) => {
                assert_eq!("OEUL", value);
                assert_eq!(CronFieldType::DayOfWeek, field_type);
            },
            _ => panic!("Did not get expected error"),
        }
    }
}