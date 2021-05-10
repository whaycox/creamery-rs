use super::*;

struct CurdsCliDependencies;
impl CliArgumentFactory for CurdsCliDependencies {
    fn arguments() -> ArgumentCollection { EnvArgumentFactory::arguments() }
}
impl CliTerminal for CurdsCliDependencies { 
    fn exit(code: i32) { StandardTerminal::exit(code) }
}

pub struct Cli;
impl Cli {
    pub fn args<TDefinition>() -> Vec<TDefinition>
    where TDefinition : CliArgumentDefinition {
        Self::parse::<TDefinition, CurdsCliDependencies>()
    }

    fn parse<TDefinition, TDepedencies>() -> Vec<TDefinition>
    where TDefinition : CliArgumentDefinition,
    TDepedencies : CliArgumentFactory + CliTerminal {
        let mut operations = Vec::<TDefinition>::new();
        let mut arguments = TDepedencies::arguments();
        while arguments.has_values() {
            let key = arguments.pop().unwrap();
            match TDefinition::parse(key, &mut arguments) {
                Ok(definition) => operations.push(definition),
                Err(parse_error) => Self::usage::<TDefinition, TDepedencies>(parse_error),
            }
        }
        operations
    }

    fn usage<TDefinition, TTerminal>(error: CliParseError)
    where TDefinition : CliArgumentDefinition,
    TTerminal : CliTerminal {
        TTerminal::exit(1)
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
    use mockall::mock;
    use serial_test::serial;
    
    mock! {
        CliDependencies {}
        impl CliArgumentFactory for CliDependencies {
            fn arguments() -> ArgumentCollection;
        }
        impl CliTerminal for CliDependencies {
            fn exit(code: i32);
        }
    }

    macro_rules! setup_arguments {
        ($($test_arg:expr),+) => {
            {
                let context = MockCliDependencies::arguments_context();
                context
                    .expect()
                    .times(1)
                    .returning(|| ArgumentCollection::new(vec![$(String::from($test_arg),)*]));
                context
            }
        }
    }

    #[test]
    #[serial]
    fn parses_properly() {
        let context = setup_arguments!["Flag", "Value", "TestingValue", "Flag"];

        let actual = Cli::parse::<TestOperation, MockCliDependencies>();

        let expected = vec![TestOperation::Flag, TestOperation::Value(String::from("TestingValue")), TestOperation::Flag];
        assert_eq!(actual.len(), expected.len());
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(value, &actual[index]);
        }
    }

    #[test]
    #[serial]
    fn parses_with_aliases() {
        let context = setup_arguments!["F", "V", "TestingValue", "F"];

        let actual = Cli::parse::<TestOperation, MockCliDependencies>();

        let expected = vec![TestOperation::Flag, TestOperation::Value(String::from("TestingValue")), TestOperation::Flag];
        assert_eq!(actual.len(), expected.len());
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(value, &actual[index]);
        }
    }

    #[test]
    #[serial]
    fn usage_exits() {
        let context = MockCliDependencies::exit_context();
        context
            .expect()
            .with(predicate::eq(1))
            .times(1)
            .returning(|code|());

        Cli::usage::<TestOperation, MockCliDependencies>(CliParseError::MissingValue);
    }
}
