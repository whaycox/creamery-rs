use super::*;

#[derive(Debug)]
pub struct CronField {
    date_part: CronDatePart,
    values: Vec<CronValue>,
}
impl Display for CronField {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut first_value_displayed = false;
        for value in &self.values {
            if !first_value_displayed {
                write!(formatter, "{}", value)?;
            }
            else {
                write!(formatter, ",{}", value)?;
            }
            if !first_value_displayed {
                first_value_displayed = true;
            }
        }
        Ok(())
    }
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
    fn displays_values() {
        let test_object = CronField {
            date_part: CronDatePart::Minutes,
            values: vec![CronValue::Any,CronValue::Any,CronValue::Any],
        };

        assert_eq!("*,*,*", &format!("{}", test_object));
    }

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