mod expression;
mod field;
mod field_type;
mod value;
mod parsing;

pub use expression::CronExpression;
pub use parsing::*;

use super::*;
use field::CronField;
use field_type::CronFieldType;
use value::CronValue;

pub use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Weekday};
use std::{fmt::{Display, Formatter}, str::FromStr, sync::OnceLock};

impl FromStr for CronExpression {
    type Err = CronParsingError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        CronExpression::parse::<CurdsCronFieldParser>(input, &CurdsCronFieldParser)
    }
}