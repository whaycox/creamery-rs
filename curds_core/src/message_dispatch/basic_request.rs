#[cfg(test)]
mod tests {
    use super::super::*;

    const FOO_MOD: u32 = 3;

    #[message_dispatch(TestMessages)]
    #[foo_request(FooMessage ~ FooMessageContext -> u32)]
    struct TestMessagesProvider {}

    impl FooRequestHandler for FooMessageContext {
        fn handle(&self, _: &dyn TestMessages, input: FooMessage) -> Result<u32> {
            if input.foo > FOO_MOD {
                Ok(input.foo % FOO_MOD)
            }
            else {
                Err(FooMessageError::test().into())
            }
        }
    }

    #[whey_context(TestMessagesProvider)]
    struct FooRequestContext {}

    #[whey(FooRequestContext ~ context)]
    fn handles_incoming_request() {
        let actual = context
            .test_type()
            .foo_request(FooMessage::new())
            .unwrap();

        assert_eq!(EXPECTED_FOO % FOO_MOD, actual)
    }

    #[whey(FooRequestContext ~ context)]
    fn returns_error() {
        let message = FooMessage {
            foo: 0,
        };

        context
            .test_type()
            .foo_request(message)
            .unwrap_err();
    }
}
