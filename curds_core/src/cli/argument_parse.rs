use super::*;
use std::{
    str::FromStr, 
    error::Error,
    any::type_name,
    sync::OnceLock,
};
use regex::Regex;

pub trait CliArgumentParse {
    fn parse(arguments: &mut Vec<String>) -> Result<Self, CliArgumentParseError> where Self: Sized;
    fn usage() -> String;
}

static TYPE_SIMPLIFIER: OnceLock<Regex> = OnceLock::new();
impl<TType> CliArgumentParse for TType where TType : FromStr, TType::Err : Error {
    fn parse(arguments: &mut Vec<String>) -> Result<Self, CliArgumentParseError> {
        match arguments.pop() {
            Some(string) => match FromStr::from_str(&string) {
                Ok(value) => Ok(value),
                Err(parse_error) => Err(CliArgumentParseError::Parse(string, parse_error.to_string())),
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