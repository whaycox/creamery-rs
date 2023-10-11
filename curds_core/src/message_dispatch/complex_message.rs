#[cfg(test)]
mod tests {
    use super::super::*;

    const MAX_INPUT: u32 = 350;
    const MIN_OUTPUT: u32 = 100;

    #[message_dispatch(TestMessages ! FooMessageError)]
    #[foo_message(FooMessage ~ mut FooMessageContext & { PreValidator, Handler -> u32, PostValidator })]
    struct TestMessagesProvider {}

    impl FooMessagePreValidator for FooMessageContext {
        fn handle(&mut self, _: &mut dyn TestMessages, input: &FooMessage) -> Result<(), FooMessageError> {
            if input.foo > MAX_INPUT {
                return Err(FooMessageError {})
            }
            Ok(())
        }
    }

    impl FooMessageHandler for FooMessageContext {
        fn handle(&mut self, _: &mut dyn TestMessages, input: FooMessage) -> Result<u32, FooMessageError> {
            Ok(input.foo / 3)
        }
    }

    impl FooMessagePostValidator for FooMessageContext {
        fn handle(&mut self, _: &mut dyn TestMessages, input: u32) -> Result<(), FooMessageError> {
            if input < MIN_OUTPUT {
                return Err(FooMessageError {})
            }
            Ok(())
        }
    }

    #[whey_context(TestMessagesProvider)]
    struct ComplexMessageContext {}

    #[whey(ComplexMessageContext ~ context)]
    fn handles_incoming_message() {
        context
            .test_type()
            .foo_message(FooMessage::test(325))
            .unwrap();
    }

    #[whey(ComplexMessageContext ~ context)]
    fn prevalidator_fires() {
        context
            .test_type()
            .foo_message(FooMessage::test(MAX_INPUT + 1))
            .unwrap_err();
    }

    #[whey(ComplexMessageContext ~ context)]
    fn postvalidator_fires() {
        context
            .test_type()
            .foo_message(FooMessage::test(MIN_OUTPUT))
            .unwrap_err();
    }
}
