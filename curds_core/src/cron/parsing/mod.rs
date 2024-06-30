mod parser;
mod link;
mod error;
mod handlers;

pub use parser::*;
pub use error::CronParsingError;

use super::*;
use link::*;
use regex::Regex;

type CronParsingHandler = fn(&str, &CronFieldType) -> Option<Result<CronValue, CronParsingError>>;