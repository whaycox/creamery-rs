#[cfg(test)]
mod tests {
    use super::super::*;

    #[message_dispatch(TestMessages)]
    #[foo_message(FooMessage ~ FooMessageContext)]
    struct TestMessagesProvider {}

    // impl FooMessageHandler for FooMessageContext {
    //     fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<()> {
    //         if EXPECTED_FOO != input.foo {
    //             Err(Box::new(FooMessageError::new("Foo was not as expected")))
    //         }
    //         else {
    //             Ok(())
    //         }
    //     }
    // }
/* 
    #[test]
    fn handles_incoming_message() {
        let provider = TestMessagesProvider::construct();
        provider.foo_message(FooMessage::new()).unwrap();
    }

    #[test]
    fn returns_error() {
        let provider = TestMessagesProvider::construct();
        let message = FooMessage {
            foo: 0,
        };

        provider.foo_message(message).unwrap_err();
    } */
}
