#[cfg(test)]
mod tests {
    use super::super::*;

    const FOO_MOD: u32 = 3;

    #[message_dispatch(TestMessages)]
    #[chain_message(FooMessage ~ FooMessageContext | [First, Second, Third])]
    struct TestMessagesProvider {}

    impl ChainMessageFirst for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<Option<()>> {
            if input.foo % FOO_MOD == 0 {
                Ok(Some(()))
            }
            else {
                Ok(None)
            }
        }
    }

    impl ChainMessageSecond for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<Option<()>> {
            if input.foo % (FOO_MOD + 1) == 0 {
                Ok(Some(()))
            }
            else {
                Ok(None)
            }
        }
    }

    impl ChainMessageThird for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<Option<()>> {
            if input.foo % (FOO_MOD + 2) == 0 {
                Ok(Some(()))
            }
            else {
                Ok(None)
            }
        }
    }

    #[test]
    fn handled_by_first_stage() {
        let provider = TestMessagesProvider::construct();
         
        assert_eq!(Some(()), provider.chain_message(FooMessage::test(FOO_MOD)).unwrap());
    }

    #[test]
    fn handled_by_second_stage() {
        let provider = TestMessagesProvider::construct();
         
        assert_eq!(Some(()), provider.chain_message(FooMessage::test(FOO_MOD + 1)).unwrap());
    }

    #[test]
    fn handled_by_third_stage() {
        let provider = TestMessagesProvider::construct();
         
        assert_eq!(Some(()), provider.chain_message(FooMessage::test(FOO_MOD + 2)).unwrap());
    }

    #[test]
    fn returns_none_when_unhandled() {
        let provider = TestMessagesProvider::construct();
         
        assert_eq!(None, provider.chain_message(FooMessage::test(7)).unwrap());
    }
}
