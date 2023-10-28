mod argument_factory;
mod terminal;

#[cfg(test)]
mod tests;

use super::*;
use argument_factory::*;
use terminal::*;

pub use curds_core_abstraction::cli::{CliArgumentParse, CliArgumentParseError};
pub use curds_core_macro::cli_arguments;

pub struct Cli {}
impl Cli {
    pub fn arguments<TOperation : CliArgumentParse>() -> Vec<TOperation> {
        let provider = CliArgumentParserProvider {};
        let parser: CliArgumentParser = provider.generate();

        match parser.parse() {
            Ok(parsed) => parsed,
            Err(error) => panic!("Failed to parse arguments: {}", error),
        }
    }
}

#[service_provider]
#[generates(CliArgumentParser)]
#[generates(dyn ArgumentFactory ~ CliArgumentFactory)]
#[generates(dyn Terminal ~ CliTerminal)]
struct CliArgumentParserProvider {}

#[injected]
struct CliArgumentParser {
    factory: Box<dyn ArgumentFactory>,
    terminal: Box<dyn Terminal>,
}

impl CliArgumentParser {
    fn parse<TOperation : CliArgumentParse>(&self) -> Result<Vec<TOperation>, CliArgumentParseError> {
        let mut arguments = self.factory.create();
        arguments.reverse();
        let mut parsed_operations: Vec<TOperation> = vec![];
        loop {
            if arguments.len() > 0 {
                match TOperation::parse(&mut arguments) {
                    Ok(parsed) => parsed_operations.push(parsed),
                    Err(error) => {
                        let application_name = self.factory.application_name();
                        self.terminal.write(&format!("{} {}", application_name, TOperation::usage()));
                        return Err(error);
                    }
                }
            }
            else {
                break;
            }
        }

        Ok(parsed_operations)
    }
}