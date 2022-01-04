#[cfg(test)]
mod tests {
    use super::super::*;

    const MAX_INPUT: u32 = 350;
    const MIN_OUTPUT: u32 = 100;

    #[message_dispatch(TestMessages)]
    #[foo_message(FooMessage ~ FooMessageContext & { PreValidator, Handler -> u32, PostValidator })]
    struct TestMessagesProvider {}

    impl FooMessagePreValidator for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<()> {
            if input.foo > MAX_INPUT {
                return Err(Box::new(FooMessageError::new("Foo was too big")))
            }
            Ok(())
        }
    }

    impl FooMessageHandler for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<u32> {
            Ok(input.foo / 3)
        }
    }

    impl FooMessagePostValidator for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &u32) -> Result<()> {
            if input < &MIN_OUTPUT {
                return Err(Box::new(FooMessageError::new("Foo was too small")))
            }
            Ok(())
        }
    }

    #[test]
    fn handles_incoming_message() {
        let provider = TestMessagesProvider::construct();
        provider.foo_message(FooMessage::test(325)).unwrap();
    }

    #[test]
    fn prevalidator_fires() {
        let provider = TestMessagesProvider::construct();
        provider.foo_message(FooMessage::test(MAX_INPUT + 1)).unwrap_err();
    }

    #[test]
    fn postvalidator_fires() {
        let provider = TestMessagesProvider::construct();
        provider.foo_message(FooMessage::test(MIN_OUTPUT)).unwrap_err();
    }
}
