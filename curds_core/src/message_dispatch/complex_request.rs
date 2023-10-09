#[cfg(test)]
mod tests {
    use super::super::*;

    const MIN_FOO: u32 = 100;
    const MAX_FOO: u32 = 200;

    #[message_dispatch(TestMessages ! FooMessageError)]
    #[foo_message(FooMessage ~ FooMessageContext & { Validator, Handler -> bool })]
    struct TestMessagesProvider {}

    impl FooMessageValidator for FooMessageContext {
        fn handle(&self, _: &dyn TestMessages, input: &FooMessage) -> Result<(), FooMessageError> {
            if input.foo < MIN_FOO || input.foo > MAX_FOO {
                return Err(FooMessageError {})
            }
            Ok(())
        }
    }

    impl FooMessageHandler for FooMessageContext {
        fn handle(&self, _: &dyn TestMessages, input: FooMessage) -> Result<bool, FooMessageError> {
            Ok(input.foo % 2 == 0)
        }
    }

    #[whey_context(TestMessagesProvider)]
    struct ComplexRequestContext {}

    #[whey(ComplexRequestContext ~ context)]
    fn handles_incoming_message() {
        let provider = context.test_type();

        assert_eq!(true, provider.foo_message(FooMessage::test(MIN_FOO)).unwrap());
        assert_eq!(false, provider.foo_message(FooMessage::test(MIN_FOO + 1)).unwrap());
    }

    #[whey(ComplexRequestContext ~ context)]
    fn validator_checks_too_small() {
        context
            .test_type()
            .foo_message(FooMessage::test(MIN_FOO - 1))
            .unwrap_err();
    }

    #[whey(ComplexRequestContext ~ context)]
    fn validator_checks_too_big() {
        context
            .test_type()
            .foo_message(FooMessage::test(MAX_FOO + 1))
            .unwrap_err();
    }
}
