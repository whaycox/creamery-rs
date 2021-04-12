use super::*;

#[derive(Debug, PartialEq, Error)]
pub enum CronParsingError {
    #[error("Expression {expression} has an empty field.")]
    EmptyField { expression: String },
    #[error("Expression {expression} had {parts} fields.")]
    FieldCount { expression: String, parts: usize },
    #[error("Value {value} is invalid for {date_part}.")]
    InvalidValue { date_part: CronDatePart, value: String },
}