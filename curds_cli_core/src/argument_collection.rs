use super::*;

pub struct ArgumentCollection {
    arguments: Vec<String>,
}
impl ArgumentCollection {
    pub fn new(mut arguments: Vec<String>) -> Self {
        arguments.reverse();
        ArgumentCollection {
            arguments: arguments,
        }
    }

    pub fn has_values(&self) -> bool {
        self.arguments.len() > 0
    }

    pub fn pop(&mut self) -> CliParseResult<String> {
        if let Some(argument) = self.arguments.pop() {
            Ok(argument)
        }
        else {
            Err(CliParseError::MissingValue)
        }
    }
}