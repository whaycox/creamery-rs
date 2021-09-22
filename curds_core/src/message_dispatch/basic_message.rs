#[cfg(test)]
mod tests {
    use super::super::*;

    #[message_dispatch(TestMessages)]
    #[foo_message(FooMessage ~ FooMessageContext)]
    struct TestMessagesProvider {}

    impl FooMessageHandler for FooMessageContext {
        fn handle(&self, dispatch: &dyn TestMessages, message: FooMessage) -> Result<()> {
            assert_eq!(EXPECTED_FOO, message.foo);
            Ok(())
        }
    }


    #[test]
    fn handles_incoming_message() {
        let provider = TestMessagesProvider::construct();
        provider.foo_message(FooMessage::new()).unwrap();
    }
}
