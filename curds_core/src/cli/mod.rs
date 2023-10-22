mod argument_factory;
mod terminal;

#[cfg(test)]
mod tests;

use super::*;
use argument_factory::*;
use terminal::*;

pub use curds_core_abstraction::cli::CliArgumentParse;

pub struct Cli {}
impl Cli {
    pub fn arguments<TOperation : CliArgumentParse>() -> Vec<TOperation> {
        let provider = CliArgumentParserProvider {};
        let parser: CliArgumentParser = provider.generate();

        parser.parse()
    }
}

#[service_provider]
#[generates(CliArgumentParser)]
#[generates(dyn ArgumentFactory ~ CliArgumentFactory)]
struct CliArgumentParserProvider {}

#[injected]
struct CliArgumentParser {
    factory: Box<dyn ArgumentFactory>,
}

impl CliArgumentParser {
    fn parse<TOperation : CliArgumentParse>(&self) -> Vec<TOperation> {
        let mut arguments = self.factory.create();
        arguments.reverse();
        let mut parsed_operations: Vec<TOperation> = vec![];
        loop {
            if arguments.len() > 0 {
                parsed_operations.push(TOperation::parse(&mut arguments));
            }
            else {
                break;
            }
        }

        parsed_operations
    }
}