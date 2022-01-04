#[cfg(test)]
mod tests {
    use super::super::*;

    const MIN_FOO: u32 = 100;
    const MAX_FOO: u32 = 200;

    #[message_dispatch(TestMessages)]
    #[foo_message(FooMessage ~ FooMessageContext & { Validator, Handler -> bool })]
    struct TestMessagesProvider {}

    impl FooMessageValidator for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<()> {
            if input.foo < MIN_FOO || input.foo > MAX_FOO {
                return Err(Box::new(FooMessageError::new("Foo was invalid")))
            }
            Ok(())
        }
    }

    impl FooMessageHandler for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<bool> {
            Ok(input.foo % 2 == 0)
        }
    }

    #[test]
    fn handles_incoming_message() {
        let provider = TestMessagesProvider::construct();

        assert_eq!(true, provider.foo_message(FooMessage::test(MIN_FOO)).unwrap());
        assert_eq!(false, provider.foo_message(FooMessage::test(MIN_FOO + 1)).unwrap());
    }

    #[test]
    fn validator_checks_too_small() {
        let provider = TestMessagesProvider::construct();
        provider.foo_message(FooMessage::test(MIN_FOO - 1)).unwrap_err();
    }

    #[test]
    fn validator_checks_too_big() {
        let provider = TestMessagesProvider::construct();
        provider.foo_message(FooMessage::test(MAX_FOO + 1)).unwrap_err();
    }
}
