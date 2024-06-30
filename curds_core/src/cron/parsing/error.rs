use std::fmt::Display;
use std::error::Error;

use super::field_type::CronFieldType;

#[derive(Debug)]
pub enum CronParsingError {
    FieldCount(String, usize),
    InvalidValue(String),
    ValueOutOfBounds {
        raw_value: String,
        allowed: u32, 
        supplied: u32,
        field_type: CronFieldType,
    },
    ParsedValue {
        value: String,
        field_type: CronFieldType 
    },
    InvertedRange {
        raw_value: String,
        min: u32,
        max: u32,
    }
}

impl Display for CronParsingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl Error for CronParsingError {}