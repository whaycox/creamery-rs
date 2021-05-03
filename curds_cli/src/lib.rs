pub use curds_cli_definition::{CliArgumentDefinition};
pub use curds_cli_derive::CliArguments;

use curds_cli_definition::ArgumentCollection;

#[cfg(test)]
use mockall::*;
#[cfg(test)]
use mockall::predicate::*;

pub struct Cli;
impl Cli {
    pub fn args<TDefinition>() -> Vec<TDefinition>
    where TDefinition : CliArgumentDefinition {
        Self::parse_with_factory::<TDefinition, EnvArgumentFactory>()
    }

    fn parse_with_factory<TDefinition, TFactory>() -> Vec<TDefinition>
    where TDefinition : CliArgumentDefinition,
    TFactory : CliArgumentFactory {
        let mut operations = Vec::<TDefinition>::new();
        let mut arguments = TFactory::arguments();
        while arguments.has_values() {
            let key = arguments.pop();
            operations.push(TDefinition::parse(key, &mut arguments));
        }
        operations
    }
}

#[cfg_attr(test, automock)]
trait CliArgumentFactory {
    fn arguments() -> ArgumentCollection;
}

struct EnvArgumentFactory;
impl CliArgumentFactory for EnvArgumentFactory {
    fn arguments() -> ArgumentCollection { 
        ArgumentCollection::new(std::env::args().collect())
    }
}

#[derive(CliArguments)]
#[name("Test App Name", "Other App Name")]
#[description("A testing application description; it's a bit longer but gives a bit more information")]
#[derive(Debug, PartialEq)]
enum TestOperation {
    #[name("A boolean flag")]
    #[description("This indicates that the key has been provided but doesn't look for any value")]
    #[key("flag", "F", "Bool")]
    Flag,
    #[name("A string value")]
    #[description("This indicates that the key has been provided and returns the next value as a String")]
    #[key("value")]
    #[key("V", "VAL")]
    Value(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_arguments {
        ($($test_arg:expr),+) => {
            {
                let context = MockCliArgumentFactory::arguments_context();
                context
                    .expect()
                    .times(1)
                    .returning(|| ArgumentCollection::new(vec![$(String::from($test_arg),)*]));
                
                Cli::parse_with_factory::<TestOperation, MockCliArgumentFactory>()
            }
        }
    }

    #[test]
    fn parses_properly() {
        let actual = test_arguments!["Flag", "Value", "TestingValue", "Flag"];

        let expected = vec![TestOperation::Flag, TestOperation::Value(String::from("TestingValue")), TestOperation::Flag];
        assert_eq!(actual.len(), expected.len());
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(value, &actual[index]);
        }
    }

    #[test]
    fn parses_with_aliases() {
        let actual = test_arguments!["F", "V", "TestingValue", "F"];

        let expected = vec![TestOperation::Flag, TestOperation::Value(String::from("TestingValue")), TestOperation::Flag];
        assert_eq!(actual.len(), expected.len());
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(value, &actual[index]);
        }
    }

}
