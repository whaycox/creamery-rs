use super::*;

const RANGE_PATTERN: &str = r"^([0-9a-zA-Z]{1,3})-([0-9a-zA-Z]{1,3})$";
static RANGE_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn parse_range(value: &str, field_type: &CronFieldType) -> Option<Result<CronValue, CronParsingError>> {
    if let Some(captures) = RANGE_REGEX.get_or_init(|| Regex::new(RANGE_PATTERN).unwrap()).captures(value) {
        let min = field_type.translate(&captures[1]);
        let max = field_type.translate(&captures[2]);

        match min.parse::<u32>() {
            Err(_) => return Some(Err(CronParsingError::ParsedValue {
                value: min.to_owned(),
                field_type: field_type.clone(), 
            })),
            Ok(min_value) => {
                let min_bound = field_type.min();
                if min_value < min_bound {
                    return Some(Err(CronParsingError::ValueOutOfBounds {
                        raw_value: value.to_owned(),
                        supplied: min_value,
                        allowed: min_bound,
                        field_type: field_type.clone(),
                    }))
                }

                match max.parse::<u32>() {
                    Err(_) => return Some(Err(CronParsingError::ParsedValue {
                        value: max.to_owned(),
                        field_type: field_type.clone(), 
                    })),
                    Ok(max_value) => {
                        let max_bound = field_type.max();
                        if max_value > max_bound {
                            return Some(Err(CronParsingError::ValueOutOfBounds {
                                raw_value: value.to_owned(),
                                supplied: max_value,
                                allowed: max_bound,
                                field_type: field_type.clone(),
                            }))
                        }            
                        if min_value > max_value {
                            return Some(Err(CronParsingError::InvertedRange {
                                raw_value: value.to_owned(),
                                min: min_value,
                                max: max_value,
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