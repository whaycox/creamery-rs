use std::fmt::Display;
use std::error::Error;

#[derive(Debug)]
pub enum CliArgumentParseError {
    ArgumentExpected,
    UnrecognizedKey(String),
    Parse(String),
}

impl Display for CliArgumentParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgumentExpected => write!(formatter, "There are no more arguments but more are expected"),
            Self::UnrecognizedKey(key) => write!(formatter, "Value \"{}\" not recognized as an operation key", key),
            Self::Parse(value) => write!(formatter, "The value \"{}\" could not be properly parsed into the expected value", value),
        }
    }
}
impl Error for CliArgumentParseError {}