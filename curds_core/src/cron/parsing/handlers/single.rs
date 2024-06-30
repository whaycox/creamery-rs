use super::*;

pub fn parse_single(value: &str, field_type: &CronFieldType) -> Option<Result<CronValue, CronParsingError>> {
    if let Ok(single_value) = field_type.translate(value).parse::<u32>() {
        if single_value < field_type.min() {
            return Some(Err(CronParsingError::ValueOutOfBounds {
                raw_value: value.to_owned(),
                supplied: single_value,
                allowed: field_type.min(),
                field_type: field_type.clone(),
            }))
        }
        if single_value > field_type.max() {
            return Some(Err(CronParsingError::ValueOutOfBounds {
                raw_value: value.to_owned(),
                supplied: single_value,
                allowed: field_type.max(),
                field_type: field_type.clone(),
            }))
        }
        Some(Ok(CronValue::Single(single_value)))
    }
    else {
        Some(Err(CronParsingError::InvalidValue(value.to_owned())))
    }
}