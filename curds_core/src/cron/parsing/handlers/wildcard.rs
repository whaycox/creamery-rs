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