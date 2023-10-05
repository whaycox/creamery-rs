#[cfg(test)]
mod tests {
    use super::super::*;

    #[message_dispatch(TestMessages)]
    #[foo_message(FooMessage ~ FooMessageContext)]
    struct TestMessagesProvider {}

    impl FooMessageHandler for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<()> {
            if EXPECTED_FOO != input.foo {
                Err(FooMessageError::test().into())
            }
            else {
                Ok(())
            }
        }
    }

    #[whey_context(TestMessagesProvider)]
    struct TestMessagesContext {}

    #[whey(TestMessagesContext ~ context)]
    fn handles_incoming_message() {
        context
            .test_type()
            .foo_message(FooMessage::new())
            .unwrap();
    }

    #[whey(TestMessagesContext ~ context)]
    fn returns_error() {
        let message = FooMessage {
            foo: 0,
        };

        context
            .test_type()
            .foo_message(message)
            .unwrap_err();
    }
}
