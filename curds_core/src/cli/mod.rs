mod argument_factory;
mod terminal;

#[cfg(test)]
mod tests;

use super::*;
pub use argument_factory::*;
use terminal::*;

pub use curds_core_abstraction::cli::*;
pub use curds_core_macro::cli_arguments;

pub struct Cli {}
impl Cli {
    pub fn arguments<TOperation : CliArgumentParse>() -> Vec<TOperation> {
        let parser = CliArgumentParser::new();

        match parser.parse() {
            Ok(parsed) => parsed,
            Err(error) => panic!("Failed to parse arguments: {}", error),
        }
    }
}

struct CliArgumentParser<
TFactory : ArgumentFactory,
TTerminal : Terminal> {
    factory: TFactory,
    terminal: TTerminal,
}

impl CliArgumentParser<CliArgumentFactory, CliTerminal> {
    pub fn new() -> Self {
        Self {
            factory: CliArgumentFactory,
            terminal: CliTerminal,
        }
    }
}

impl<TFactory, TTerminal> CliArgumentParser<TFactory, TTerminal> where 
TFactory : ArgumentFactory,
TTerminal : Terminal {
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