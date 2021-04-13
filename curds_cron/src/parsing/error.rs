use super::*;

#[derive(Debug, PartialEq, Error)]
pub enum CronParsingError {
    #[error("Expression {expression} has an empty field.")]
    EmptyField { 
        expression: String 
    },
    #[error("Expression {expression} had {parts} fields.")]
    FieldCount { 
        expression: String, 
        parts: usize,
    },
    #[error("Value {value} is invalid for {date_part}.")]
    InvalidValue { 
        date_part: CronDatePart, 
        value: String 
    },
    #[error("Value '{raw_value}'=>{supplied} is outside the bound allowed {allowed}")]
    ValueOutOfBounds { 
        raw_value: String,
        allowed: u32, 
        supplied: u32,
        date_part: CronDatePart,
    },
    #[error("{value} could not parse into a value for {date_part}")]
    ParsedValue { 
        value: String, 
        date_part: CronDatePart 
    },
    #[error("Inverted range {min}-{max} is not allowed")]
    InvertedRange {
        min: u32,
        max: u32,
    }
}