use super::*;

pub struct SingleValueHandler;
impl CronValueParsingHandler for SingleValueHandler {
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<CronValue> {
        let single_value = date_part.translate(value).parse::<u32>().unwrap();
        if single_value < date_part.min() {
            panic!("{} is less than allowed", single_value);
        }
        if single_value > date_part.max() {
            panic!("{} is greater than allowed", single_value);
        }
        Some(CronValue::Single(single_value))
    }
}