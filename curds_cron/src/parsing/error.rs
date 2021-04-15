use super::*;

/// An enum covering the possible failures when parsing a CronExpression.
#[derive(Debug, PartialEq, Error)]
pub enum CronParsingError {
    /// Returned when the provided expression does not have the correct number of fields.
    #[error("Expression {expression} had {parts} fields.")]
    FieldCount { 
        /// The raw expression being parsed.
        expression: String, 
        /// The number of parts supplied.
        parts: usize,
    },
    /// Returned when a value is never handled in parsing.
    #[error("Value {value} is invalid for {date_part}.")]
    InvalidValue { 
        /// The CronDatePart for which the value was supplied.
        date_part: CronDatePart, 
        /// The value that was not handled.
        value: String 
    },
    /// Returned when a parsed value is outside the allowable limits for that field.
    #[error("Value '{raw_value}'=>{supplied} is outside the bound allowed {allowed}")]
    ValueOutOfBounds {
        /// The raw value being parsed.
        raw_value: String,
        /// The allowed boundary value.
        allowed: u32, 
        /// The supplied value.
        supplied: u32,
        /// The CronDatePart for which the value was supplied.
        date_part: CronDatePart,
    },
    /// Returned when a supplied value does not end up parsing properly.
    #[error("{value} could not parse into a value for {date_part}")]
    ParsedValue { 
        /// The value for which parsing was attempted.
        value: String, 
        /// The CronDatePart for which the value was supplied.
        date_part: CronDatePart 
    },
    /// Returned when an inverted range is supplied.
    #[error("Inverted range {min}-{max} is not allowed")]
    InvertedRange {
        /// The min value supplied.
        min: u32,
        /// The max value supplied.
        max: u32,
    }
}