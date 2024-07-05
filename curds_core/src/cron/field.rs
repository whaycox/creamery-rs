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