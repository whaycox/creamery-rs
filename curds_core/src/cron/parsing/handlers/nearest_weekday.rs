use super::*;

const NEAREST_WEEKDAY_PATTERN: &str = r"^(\d{1,2})[Ww]$";
static NEAREST_WEEKDAY_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn parse_nearest_weekday(value: &str, field_type: &CronFieldType) -> Option<Result<CronValue, CronParsingError>> {
    if let Some(captures) = NEAREST_WEEKDAY_REGEX.get_or_init(|| Regex::new(NEAREST_WEEKDAY_PATTERN).unwrap()).captures(value) {
        let parsed_value = &captures[1].parse::<u32>().unwrap();
        if *parsed_value < field_type.min() {
            return Some(Err(CronParsingError::ValueOutOfBounds {
                raw_value: value.to_owned(),
                allowed: field_type.min(),
                supplied: *parsed_value,
                field_type: field_type.clone(),
            }))
        }
        if *parsed_value > field_type.max() {
            return Some(Err(CronParsingError::ValueOutOfBounds {
                raw_value: value.to_owned(),
                allowed: field_type.max(),
                supplied: *parsed_value,
                field_type: field_type.clone(),
            }))
        }
        return Some(Ok(CronValue::NearestWeekday { day_of_month: *parsed_value }))
    }
    None
}