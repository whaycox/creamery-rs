use super::*;

const RANGE_PATTERN: &str = r"^([0-9a-zA-Z]{1,3})-([0-9a-zA-Z]{1,3})$";
static RANGE_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn parse_range(value: &str, field_type: &CronFieldType) -> Option<Result<CronValue, CronParsingError>> {
    if let Some(captures) = RANGE_REGEX.get_or_init(|| Regex::new(RANGE_PATTERN).unwrap()).captures(value) {
        let min = field_type.translate(&captures[1]);
        let max = field_type.translate(&captures[2]);

        match min.parse::<u32>() {
            Err(_) => return Some(Err(CronParsingError::InvalidValue {
                value: min.to_owned(),
                field_type: field_type.clone(), 
            })),
            Ok(min_value) => {
                let min_bound = field_type.min();
                if min_value < min_bound {
                    return Some(Err(CronParsingError::ValueOutOfBounds {
                        value: value.to_owned(),
                        allowed: min_bound,
                        field_type: field_type.clone(),
                    }))
                }

                match max.parse::<u32>() {
                    Err(_) => return Some(Err(CronParsingError::InvalidValue {
                        value: max.to_owned(),
                        field_type: field_type.clone(), 
                    })),
                    Ok(max_value) => {
                        let max_bound = field_type.max();
                        if max_value > max_bound {
                            return Some(Err(CronParsingError::ValueOutOfBounds {
                                value: value.to_owned(),
                                allowed: max_bound,
                                field_type: field_type.clone(),
                            }))
                        }            
                        if min_value > max_value {
                            return Some(Err(CronParsingError::InvertedRange {
                                value: value.to_owned(),
                                field_type: field_type.clone(),
                            }))
                        }
                        
                        return Some(Ok(CronValue::Range { 
                            min: min_value, 
                            max: max_value 
                        }))
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_range_as_int() {
        let actual = parse_range("4-8", &CronFieldType::Minute)
            .unwrap()
            .unwrap();
        
        assert_eq!(CronValue::Range { min: 4, max: 8 }, actual);
    }

    #[test]
    fn translates_min() {
        let actual = parse_range("MAR-6", &CronFieldType::Month)
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::Range { min: 3, max: 6 }, actual);
    }

    #[test]
    fn translates_max() {
        let actual = parse_range("4-sat", &CronFieldType::DayOfWeek)
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::Range { min: 4, max: 6 }, actual);
    }

    #[test]
    fn nonmatch_return_none() {
        assert_eq!(None, parse_range("DayOfWeek", &CronFieldType::DayOfWeek));
    }

    #[test]
    fn nonint_min_returns_error() {
        match parse_range("OEU-5", &CronFieldType::DayOfWeek).unwrap() {
            Err(CronParsingError::InvalidValue { value, field_type }) => {
                assert_eq!("OEU", value);
                assert_eq!(CronFieldType::DayOfWeek, field_type);
            },
            _ => panic!("Did not get expected error"),
        }
    }

    #[test]
    fn nonint_max_returns_error() {
        match parse_range("03-OEU", &CronFieldType::DayOfWeek).unwrap() {
            Err(CronParsingError::InvalidValue { value, field_type }) => {
                assert_eq!("OEU", value);
                assert_eq!(CronFieldType::DayOfWeek, field_type);
            },
            _ => panic!("Did not get expected error"),
        }
    }

    #[test]
    fn inverted_range_returns_error() {
        match parse_range("THU-MON", &CronFieldType::DayOfWeek).unwrap() {
            Err(CronParsingError::InvertedRange { value, field_type }) => {
                assert_eq!("THU-MON", value);
                assert_eq!(CronFieldType::DayOfWeek, field_type);
            },
            _ => panic!("Did not get expected error"),
        }
    }
}