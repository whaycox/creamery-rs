#[cfg(test)]
mod tests {
    use super::super::*;

    #[message_dispatch(TestMessages)]
    #[foo_message(FooMessage <- FooMessageContext)]
    struct TestMessagesProvider {}

    #[service_provider]
    #[generates(FooMessageContext)]
    struct TestMessagesProvider {}
    impl TestMessages for TestMessagesProvider {
        fn foo_message(&self, message: FooMessage) -> Result<()> {
            let context = ServiceGenerator::<Rc<FooMessageContext>>::generate(self);
            context.handle(self, message)?;
            Ok(())
        }
    }

    trait FooMessageHandler {
        fn handle(&self, dispatch: &dyn TestMessages, message: FooMessage) -> Result<()>;
    }

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
