mod argument_factory;
mod error;
mod argument_parse;

#[cfg(test)]
mod tests;

use super::*;

pub use argument_factory::*;
pub use argument_parse::*;
pub use error::*;
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
    
    pub fn usage<TOperation : CliArgumentParse>() {
        CliArgumentParser::new().usage::<TOperation>();
    }
}

struct CliArgumentParser<TFactory : ArgumentFactory> {
    factory: TFactory,
}

impl CliArgumentParser<CliArgumentFactory> {
    pub fn new() -> Self {
        Self {
            factory: CliArgumentFactory,
        }
    }
}

impl<TFactory> CliArgumentParser<TFactory> where 
TFactory : ArgumentFactory {
    fn parse<TOperation : CliArgumentParse>(&self) -> Result<Vec<TOperation>, CliArgumentParseError> {
        let mut arguments = self.factory.create();
        arguments.reverse();
        let mut parsed_operations: Vec<TOperation> = vec![];
        loop {
            if arguments.len() > 0 {
                match TOperation::parse(&mut arguments) {
                    Ok(parsed) => parsed_operations.push(parsed),
                    Err(error) => {
                        self.usage::<TOperation>();
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

    fn usage<TOperation : CliArgumentParse>(&self) {
        log::info!("{} {}", self.factory.application_name(), TOperation::usage());
        if let Some(descriptions) = TOperation::description() {
            for description in descriptions {
                log::info!("{}", description);
            }
        }
    }
}