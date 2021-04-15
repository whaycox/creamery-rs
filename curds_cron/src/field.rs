use super::*;

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_builds_expected() {
        let field = CronField::new(CronDatePart::DayOfWeek, vec!(CronValue::Any));

        assert_eq!(CronDatePart::DayOfWeek, field.date_part);
        assert_eq!(1, field.values.len());
    }

    #[test]
    fn matches_if_any_value() {
        let values = vec![CronValue::Single(99), CronValue::Any, CronValue::Single(99)];
        let test_object = CronField::new(CronDatePart::Hours, values);

        assert_eq!(true, test_object.is_match(&Utc::now()));
    }

    #[test]
    fn doesnt_match_when_no_values_match() {
        let values = vec![CronValue::Single(99)];
        let test_object = CronField::new(CronDatePart::Hours, values);

        assert_eq!(false, test_object.is_match(&Utc::now()));
    }
}