mod argument_factory;
mod terminal;

use super::*;
use argument_factory::*;
use terminal::*;

pub use curds_core_abstraction::cli::CliArgumentParse;

pub struct Cli {}
impl Cli {
    fn arguments<TOperation : CliArgumentParse>() -> Vec<TOperation> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[cli_arguments]
    #[derive(PartialEq, Debug)]
    enum TestOperations {
        FirstBoolean,
        SecondBool,
        Message(String, u32),
    }

    // impl TestOperations {
    //     fn parse_first() -> TestOperations {
    //         TestOperations::First
    //     }
    //     fn parse_second() -> TestOperations {
    //         TestOperations::Second
    //     }
    //     fn parse_message(arguments: &mut Vec<String>) -> TestOperations {
    //         let operation = TestOperations::Message(<String as CliArgumentParse>::parse(arguments));
    //         return  operation;
    //     }
    // }

    #[whey_context(CliArgumentParser)]
    #[mocks(dyn ArgumentFactory)]
    struct CliArgumentParserContext {}

    #[whey(CliArgumentParserContext ~ context)]
    fn parses_boolean_operations() {
        mock_return!(context ~ ArgumentFactory ~ create, || vec![
            String::from("--first_boolean"),
            String::from("--second_bool"),
        ], 1);

        let actual = context
            .test_type()
            .parse();

        assert_eq!(2, actual.len());
        assert_eq!(TestOperations::FirstBoolean, actual[0]);
        assert_eq!(TestOperations::SecondBool, actual[1]);
    }

    #[whey(CliArgumentParserContext ~ context)]
    fn parses_operations_with_anonymous_values() {
        mock_return!(context ~ ArgumentFactory ~ create, || vec![
            String::from("--message"),
            String::from("This is a test message"),
            String::from("123"),
        ], 1);

        let actual = context
            .test_type()
            .parse();

        assert_eq!(1, actual.len());
        assert_eq!(TestOperations::Message(String::from("This is a test message"), 123), actual[0]);
    }
}