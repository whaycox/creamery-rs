use std::str::FromStr;

pub trait CliArgumentParse {
    fn parse(arguments: &mut Vec<String>) -> Self;
}

impl<TType> CliArgumentParse for TType where TType : FromStr {
    fn parse(arguments: &mut Vec<String>) -> Self {
        match arguments.pop() {
            Some(string) => match FromStr::from_str(&string) {
                Ok(value) => value,
                Err(_) => panic!("The value \"{}\" could not be properly parsed into the expected value", string),
            },
            None => panic!("There are no more arguments but more are expected"),
        }

    }
} 