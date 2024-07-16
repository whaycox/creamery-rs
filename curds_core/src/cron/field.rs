use super::*;

#[derive(Debug)]
pub struct CronField {
    field_type: CronFieldType,
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
    pub fn new(field_type: CronFieldType, values: Vec<CronValue>) -> Self {
        Self {
            field_type,
            values,
        }
    }

    pub fn is_responsive<T: TimeZone>(&self, time: &DateTime<T>) -> bool {
        for value in &self.values {
            if value.is_responsive(time, &self.field_type) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Utc;

    #[test]
    fn displays_values() {
        let test_object = CronField {
            field_type: CronFieldType::Minute,
            values: vec![CronValue::Any,CronValue::Any,CronValue::Any],
        };

        assert_eq!("*,*,*", &format!("{}", test_object));
    }

    #[test]
    fn new_builds_expected() {
        let field = CronField::new(CronFieldType::DayOfWeek, vec!(CronValue::Any));

        assert_eq!(CronFieldType::DayOfWeek, field.field_type);
        assert_eq!(1, field.values.len());
    }

    #[test]
    fn responsive_when_any_value_is() {
        let values = vec![CronValue::Single(99), CronValue::Any, CronValue::Single(99)];
        let test_object = CronField::new(CronFieldType::Hour, values);

        assert_eq!(true, test_object.is_responsive(&Utc::now()));
    }

    #[test]
    fn unresponsive_when_no_values_are_responsive() {
        let values = vec![CronValue::Single(99)];
        let test_object = CronField::new(CronFieldType::Hour, values);

        assert_eq!(false, test_object.is_responsive(&Utc::now()));
    }
}