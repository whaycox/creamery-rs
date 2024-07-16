use super::*;

const LAST_DAY_OF_MONTH_PATTERN: &str = r"^[Ll](?:-(\d+))?$";
static LAST_DAY_OF_MONTH_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn parse_last_day_of_month(value: &str, field_type: &CronFieldType) -> Option<Result<CronValue, CronParsingError>> {
    if let Some(captures) = LAST_DAY_OF_MONTH_REGEX.get_or_init(|| Regex::new(LAST_DAY_OF_MONTH_PATTERN).unwrap()).captures(value) {
        if let Some(offset) = captures.get(1) {
            let offset_value = offset.as_str().parse::<u32>().unwrap();
            let max_offset = field_type.max() - 1;
            if offset_value > max_offset {
                return Some(Err(CronParsingError::ValueOutOfBounds {
                    value: value.to_owned(),
                    allowed: max_offset,
                    field_type: field_type.clone(),
                }))
            }
            return Some(Ok(CronValue::LastDayOfMonth { offset: offset_value }))
        }
        return Some(Ok(CronValue::LastDayOfMonth { offset: 0 }))
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_without_offset() {
        let actual = parse_last_day_of_month("L", &CronFieldType::DayOfMonth)
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::LastDayOfMonth { offset: 0 }, actual);
    }

    #[test]
    fn captures_with_offset() {
        let actual = parse_last_day_of_month("l-30", &CronFieldType::DayOfMonth)
            .unwrap()
            .unwrap();

        assert_eq!(CronValue::LastDayOfMonth { offset: 30 }, actual);
    }

    #[test]
    fn nonmatch_returns_none() {
        assert_eq!(None, parse_last_day_of_month("DayOfMonth", &CronFieldType::DayOfMonth));
    }

    #[test]
    fn offset_larger_than_allowed_returns_error() {
        match parse_last_day_of_month("L-31", &CronFieldType::DayOfMonth).unwrap() {
            Err(CronParsingError::ValueOutOfBounds { value, allowed, field_type }) => {
                assert_eq!("L-31", value);
                assert_eq!(30, allowed);
                assert_eq!(CronFieldType::DayOfMonth, field_type);
            },
            _ => panic!("Did not get the expected error"),
        }
    }
}