use super::*;

#[derive(Debug)]
pub struct CronField {
    field_type: CronFieldType,
    values: Vec<CronValue>,
}

impl CronField {
    pub fn new(field_type: CronFieldType, values: Vec<CronValue>) -> Self {
        Self {
            field_type,
            values,
        }
    }
}