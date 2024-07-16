use super::*;

const WILDCARD_PATTERN: &str = r"^\*(?:/(\d+))?$";
static WILDCARD_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn parse_wildcard(value: &str, field_type: &CronFieldType) -> Option<Result<CronValue, CronParsingError>> {
    if let Some(captures) = WILDCARD_REGEX.get_or_init(|| Regex::new(WILDCARD_PATTERN).unwrap()).captures(value) {
        if let Some(step_capture) = captures.get(1) {
            let step_value = step_capture
                .as_str()
                .parse::<u32>()
                .unwrap();
            if step_value <= 1 {
                return Some(Err(CronParsingError::ValueOutOfBounds {
                    value: value.to_owned(),
                    allowed: 2,
                    field_type: field_type.clone(),
                }))
            }
            if field_type.min() + step_value > field_type.max() {
                return Some(Err(CronParsingError::ValueOutOfBounds {
                    value: value.to_owned(),
                    allowed: field_type.max() - field_type.min(),
                    field_type: field_type.clone(),
                }))
            }
            return Some(Ok(CronValue::Step(step_value)))
        }
        return Some(Ok(CronValue::Any))
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_returns_any() {
        let actual = parse_wildcard("*", &CronFieldType::DayOfWeek)
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::Any, actual);
    }

    #[test]
    fn parses_step_if_supplied() {
        let actual = parse_wildcard("*/3", &CronFieldType::DayOfWeek)
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::Step(3), actual);
    }

    #[test]
    fn non_match_returns_none() {
        assert_eq!(None, parse_wildcard("Hour", &CronFieldType::Hour));
    }

    #[test]
    fn step_smaller_than_allowed_returns_error() {
        match parse_wildcard("*/1", &CronFieldType::Month).unwrap() {
            Err(CronParsingError::ValueOutOfBounds { value, allowed, field_type }) => {
                assert_eq!("*/1", value);
                assert_eq!(2, allowed);
                assert_eq!(CronFieldType::Month, field_type);
            },
            _ => panic!("Did not get expected error"),
        }
    }

    #[test]
    fn step_larger_than_allowed_returns_error() {
        match parse_wildcard("*/60", &CronFieldType::Minute).unwrap() {
            Err(CronParsingError::ValueOutOfBounds { value, allowed, field_type }) => {
                assert_eq!("*/60", value);
                assert_eq!(59, allowed);
                assert_eq!(CronFieldType::Minute, field_type);
            },
            _ => panic!("Did not get expected error"),
        }
        match parse_wildcard("*/12", &CronFieldType::Month).unwrap() {
            Err(CronParsingError::ValueOutOfBounds { value, allowed, field_type }) => {
                assert_eq!("*/12", value);
                assert_eq!(11, allowed);
                assert_eq!(CronFieldType::Month, field_type);
            },
            _ => panic!("Did not get expected error"),
        }
    }
}