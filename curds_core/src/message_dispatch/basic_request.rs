#[cfg(test)]
mod tests {
    use super::super::*;

    const FOO_MOD: u32 = 3;

    #[message_dispatch(TestMessages)]
    #[foo_request(FooMessage ~ FooMessageContext -> u32)]
    struct TestMessagesProvider {}

    // impl FooRequestHandler for FooMessageContext {
    //     fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<u32> {
    //         if input.foo > FOO_MOD {
    //             Ok(input.foo % FOO_MOD)
    //         }
    //         else {
    //             Err(Box::new(FooMessageError::new("Foo was too small")))
    //         }
    //     }
    // }
/* 
    #[test]
    fn handles_incoming_request() {
        let provider = TestMessagesProvider::construct();
        
        let actual = provider.foo_request(FooMessage::new()).unwrap();

        assert_eq!(EXPECTED_FOO % FOO_MOD, actual)
    }

    #[test]
    fn returns_error() {
        let provider = TestMessagesProvider::construct();
        let message = FooMessage {
            foo: 0,
        };

        provider.foo_request(message).unwrap_err();
    } */
}
