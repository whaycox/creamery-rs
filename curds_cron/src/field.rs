use super::*;

pub struct CronField {
    date_part: CronDatePart,
    values: Vec<CronValue>,
}
impl CronField {
    pub fn new(date_part: CronDatePart, values: Vec<CronValue>) -> CronField {
        CronField {
            date_part: date_part,
            values: values,
        }
    }

    pub fn is_match<Tz>(&self, datetime: &DateTime<Tz>) -> bool 
    where Tz: TimeZone {
        for value in &self.values {
            if value.is_match(&self.date_part, datetime) {
                return true;
            }
        }
        false
    }
}