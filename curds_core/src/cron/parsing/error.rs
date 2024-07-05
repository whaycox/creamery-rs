use thiserror::Error;
use super::field_type::CronFieldType;

#[derive(Debug, Error)]
pub enum CronParsingError {
    #[error("Expression {expression} contains {parts} fields")]
    FieldCount {
        expression: String,
        parts: usize,
    },
    #[error("The value \"{value}\" is invalid for the {field_type} field")]
    InvalidValue {
        value: String,
        field_type: CronFieldType
    },
    #[error("The value \"{value}\" is outside the acceptable {field_type} bounds; {allowed} is the nearest allowed value")]
    ValueOutOfBounds {
        value: String,
        allowed: u32,
        field_type: CronFieldType,
    },
    #[error("The value \"{raw_value}\" represents an inverted range; this is not allowed")]
    InvertedRange {
        raw_value: String,
    }
}