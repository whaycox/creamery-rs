use super::*;

pub fn parse_single(value: &str, field_type: &CronFieldType) -> Option<Result<CronValue, CronParsingError>> {
    if let Ok(single_value) = field_type.translate(value).parse::<u32>() {
        if single_value < field_type.min() {
            return Some(Err(CronParsingError::ValueOutOfBounds {
                value: value.to_owned(),
                allowed: field_type.min(),
                field_type: field_type.clone(),
            }))
        }
        if single_value > field_type.max() {
            return Some(Err(CronParsingError::ValueOutOfBounds {
                value: value.to_owned(),
                allowed: field_type.max(),
                field_type: field_type.clone(),
            }))
        }
        Some(Ok(CronValue::Single(single_value)))
    }
    else {
        Some(Err(CronParsingError::InvalidValue {
            value: value.to_owned(),
            field_type: field_type.clone(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_value_as_int() {
        let actual = parse_single("6", &CronFieldType::Hour)
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::Single(6), actual);
    }

    #[test]
    fn translates_month() {
        let actual = parse_single("apr", &CronFieldType::Month)
            .unwrap()
            .unwrap();
        
        assert_eq!(CronValue::Single(4), actual);
    }

    #[test]
    fn translates_day_of_week() {
        let actual = parse_single("thu", &CronFieldType::DayOfWeek)
            .unwrap()
            .unwrap();
        
        assert_eq!(CronValue::Single(4), actual);
    }

    #[test]
    fn less_than_min_returns_error() {
        match parse_single("0", &CronFieldType::Month).unwrap() {
            Err(CronParsingError::ValueOutOfBounds { value, allowed, field_type }) => {
                assert_eq!("0", value);
                assert_eq!(1, allowed);
                assert_eq!(CronFieldType::Month, field_type);
            },
            _ => panic!("Did not get expected error"),
        }
    }

    #[test]
    fn greater_than_max_returns_error() {
        match parse_single("13", &CronFieldType::Month).unwrap() {
            Err(CronParsingError::ValueOutOfBounds { value, allowed, field_type }) => {
                assert_eq!("13", value);
                assert_eq!(12, allowed);
                assert_eq!(CronFieldType::Month, field_type);
            },
            _ => panic!("Did not get expected error"),
        }
    }

    #[test]
    fn nonint_returns_error() {
        match parse_single("Month", &CronFieldType::Month).unwrap() {
            Err(CronParsingError::InvalidValue { value, field_type }) => {
                assert_eq!("Month", value);
                assert_eq!(CronFieldType::Month, field_type);
            },
            _ => panic!("Did not get expected error"),
        }
    }
}