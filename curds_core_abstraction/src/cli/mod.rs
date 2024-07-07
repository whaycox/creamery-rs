use std::{
    str::FromStr, 
    fmt::Display, 
    error::Error, 
    any::type_name,
    sync::OnceLock,
};
use regex::Regex;

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

pub trait CliArgumentParse {
    fn parse(arguments: &mut Vec<String>) -> Result<Self, CliArgumentParseError> where Self: Sized;
    fn usage() -> String;
}

static TYPE_SIMPLIFIER: OnceLock<Regex> = OnceLock::new();
impl<TType> CliArgumentParse for TType where TType : FromStr {
    fn parse(arguments: &mut Vec<String>) -> Result<Self, CliArgumentParseError> {
        match arguments.pop() {
            Some(string) => match FromStr::from_str(&string) {
                Ok(value) => Ok(value),
                Err(_) => Err(CliArgumentParseError::Parse(string)),
            },
            None => Err(CliArgumentParseError::ArgumentExpected),
        }
    }

    fn usage() -> String {
        let simplifier = TYPE_SIMPLIFIER.get_or_init(|| Regex::new("[^:]+$").unwrap());
        let type_name: &str = simplifier.find(type_name::<TType>())
            .unwrap()
            .as_str();
        format!("<{}>", type_name) 
    }
}