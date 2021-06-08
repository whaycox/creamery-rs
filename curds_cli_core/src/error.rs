use super::*;

#[derive(Debug, PartialEq, Error)]
pub enum CliParseError {
    #[error("Help was requested")]
    Help,
    #[error("Unsupported key {key} provided.")]
    UnsupportedKey {
        key: String,
    },
    #[error("An argument value was expected but none provided")]
    MissingValue,
}

impl CliParseError {
    pub fn write_to_terminal<TTerminal>(&self)
    where TTerminal : CliTerminal {
        match self {
            CliParseError::UnsupportedKey { key } => TTerminal::write_error(&format!("Unsupported key {} provided", key)),
            CliParseError::MissingValue => TTerminal::write_error("An argument value was expected but none provided"),
            _ => {},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use serial_test::serial;
    
    mock! {
        Terminal {}
        impl CliTerminal for Terminal {
            fn write_out(message: &str);
            fn write_error(message: &str);

            fn exit(code: i32);
        }
    }

    #[test]
    #[serial]
    fn help_doesnt_write_to_terminal() {
        let terminal_write_context = MockTerminal::write_error_context();
        terminal_write_context
            .expect()
            .with(predicate::always())
            .times(0)
            .returning(|_text|());
        let test_error = CliParseError::Help;

        test_error.write_to_terminal::<MockTerminal>()
    }

    #[test]
    #[serial]
    fn key_writes_to_terminal() {
        let terminal_write_context = MockTerminal::write_error_context();
        terminal_write_context
            .expect()
            .with(predicate::eq("Unsupported key test provided"))
            .times(1)
            .returning(|_text|());
        let test_error = CliParseError::UnsupportedKey { 
            key: String::from("test") 
        };

        test_error.write_to_terminal::<MockTerminal>()
    }

    #[test]
    #[serial]
    fn missing_value_writes_to_terminal() {
        let terminal_write_context = MockTerminal::write_error_context();
        terminal_write_context
            .expect()
            .with(predicate::eq("An argument value was expected but none provided"))
            .times(1)
            .returning(|_text|());
        let test_error = CliParseError::MissingValue;

        test_error.write_to_terminal::<MockTerminal>()
    }
}