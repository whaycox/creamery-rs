use super::*;

const NTH_DAY_OF_WEEK_PATTERN: &str = r"^([a-zA-Z0-6]{1,3})#([1-5])$";
static NTH_DAY_OF_WEEK_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn parse_nth_day_of_week(value: &str, field_type: &CronFieldType) -> Option<Result<CronValue, CronParsingError>> {
    if let Some(captures) = NTH_DAY_OF_WEEK_REGEX.get_or_init(|| Regex::new(NTH_DAY_OF_WEEK_PATTERN).unwrap()).captures(value) {
        if let Ok(parsed_value) = field_type.translate(&captures[1]).parse::<u32>() {
            if parsed_value > field_type.max() {
                return Some(Err(CronParsingError::ValueOutOfBounds {
                    value: value.to_owned(),
                    allowed: field_type.max(),
                    field_type: field_type.clone(),
                }))
            }
            let n = captures[2].parse::<u32>().unwrap();                
            return Some(Ok(CronValue::NthDayOfWeek { 
                day_of_week: parsed_value, 
                n: n 
            }))
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