use super::*;

pub struct Cli;
impl Cli {
    pub fn args<TDefinition>() -> Vec<TDefinition>
    where TDefinition : CliArgumentDefinition {
        Self::parse::<TDefinition, EnvArgumentFactory, StandardTerminal>()
    }

    fn parse<TDefinition, TArgumentFactory, TTerminal>() -> Vec<TDefinition>
    where TDefinition : CliArgumentDefinition,
    TArgumentFactory : CliArgumentFactory,
    TTerminal : CliTerminal {
        let mut operations = Vec::<TDefinition>::new();
        let mut arguments = TArgumentFactory::arguments();
        while arguments.has_values() {
            let key = arguments.pop().unwrap();
            match TDefinition::parse(key, &mut arguments) {
                Ok(operation) => operations.push(operation),
                Err(parse_error) => Self::usage::<TDefinition, TTerminal>(parse_error),
            }
        }
        operations
    }

    fn usage<TDefinition, TTerminal>(error: CliParseError)
    where TDefinition : CliArgumentDefinition,
    TTerminal : CliTerminal {
        let mut detailed = false;
        match error {
            CliParseError::Help => { detailed = true; },
            _ => error.write_to_terminal::<TTerminal>(),
        }
        TDefinition::usage(detailed);
        TTerminal::exit(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use serial_test::serial;
  
    mock! {
        ArgumentDefinition {}
        impl CliArgumentDefinition for ArgumentDefinition {
            fn parse(key: String, arguments: &mut ArgumentCollection) -> CliParseResult<Self>
            where Self : Sized;
        
            fn usage(detailed: bool);
        }
    }
  
    mock! {
        ArgumentFactory {}
        impl CliArgumentFactory for ArgumentFactory {
            fn arguments() -> ArgumentCollection;
        }
    }
    
    mock! {
        Terminal {}
        impl CliTerminal for Terminal {
            fn write_out(message: &str);
            fn write_error(message: &str);

            fn exit(code: i32);
        }
    }
    
    macro_rules! setup_arguments {
        ($($test_arg:expr),+) => {
            ArgumentCollection::new(vec![$(String::from($test_arg),)*])
        }
    }
    
    #[test]
    #[serial]
    fn parse_is_expected() {
        let mut expectation_sequence = Sequence::new();
        let arguments_context = MockArgumentFactory::arguments_context();
        arguments_context
            .expect()
            .times(1)
            .returning(||setup_arguments!("one", "two"));
        let definition_parse_context = MockArgumentDefinition::parse_context();
        definition_parse_context
            .expect()
            .with(predicate::eq(String::from("one")), predicate::always())
            .times(1)
            .in_sequence(&mut expectation_sequence)
            .returning(|_key, _args| Ok(MockArgumentDefinition::new()));
        definition_parse_context
            .expect()
            .with(predicate::eq(String::from("two")), predicate::always())
            .times(1)
            .in_sequence(&mut expectation_sequence)
            .returning(|_key, _args| Ok(MockArgumentDefinition::new()));

        let actual = Cli::parse::<MockArgumentDefinition, MockArgumentFactory, MockTerminal>();

        assert_eq!(2, actual.len());
    }

    #[test]
    #[serial]
    fn help_usage_is_detailed() {
        let mut expectation_sequence = Sequence::new();
        let definition_usage_context = MockArgumentDefinition::usage_context();
        definition_usage_context
            .expect()
            .with(predicate::eq(true))
            .times(1)
            .in_sequence(&mut expectation_sequence)
            .returning(|_detailed|());
        let terminal_exit_context = MockTerminal::exit_context();
        terminal_exit_context
            .expect()
            .with(predicate::eq(1))
            .times(1)
            .in_sequence(&mut expectation_sequence)
            .returning(|_code|());

        Cli::usage::<MockArgumentDefinition, MockTerminal>(CliParseError::Help);
    }

    fn test_error_isnt_detailed(test_error: CliParseError) {
        let mut expectation_sequence = Sequence::new();
        let terminal_write_context = MockTerminal::write_error_context();
        terminal_write_context
            .expect()
            .with(predicate::always())
            .times(1)
            .in_sequence(&mut expectation_sequence)
            .returning(|_formatted_error|());
        let definition_usage_context = MockArgumentDefinition::usage_context();
        definition_usage_context
            .expect()
            .with(predicate::eq(false))
            .times(1)
            .in_sequence(&mut expectation_sequence)
            .returning(|_detailed|());
        let terminal_exit_context = MockTerminal::exit_context();
        terminal_exit_context
            .expect()
            .with(predicate::eq(1))
            .times(1)
            .in_sequence(&mut expectation_sequence)
            .returning(|_code|());

        Cli::usage::<MockArgumentDefinition, MockTerminal>(test_error)
    }

    #[test]
    #[serial]
    fn key_usage_isnt_detailed() {
        test_error_isnt_detailed(CliParseError::UnsupportedKey { key: String::from("test") })
    }

    #[test]
    #[serial]
    fn value_usage_isnt_detailed() {
        test_error_isnt_detailed(CliParseError::MissingValue)
    }
}
