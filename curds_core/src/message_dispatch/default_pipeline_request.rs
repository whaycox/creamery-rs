#[cfg(test)]
mod tests {
    use super::super::*;

    const FOO_MOD: u32 = 3;

    #[message_dispatch(TestMessages)]
    #[first(FooMessage ~ FooMessageContext -> bool)]
    #[second(FooMessage ~ FooMessageContext -> u32)]
    #[third(FooMessage ~ FooMessageContext & -> FooMessage)]
    #[request(PreProcessor -> $request, Processor)]
    struct TestMessagesProvider {}

    impl FirstPreProcessor for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<FooMessage> {
            Ok(FooMessage::test(input.foo % FOO_MOD))
        }
    }
    impl FirstProcessor for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<bool> {
            Ok(input.foo % 2 == 0)
        }
    }

    impl SecondPreProcessor for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<FooMessage> {
            Ok(FooMessage::test(input.foo % (FOO_MOD + 1)))
        }
    }
    impl SecondProcessor for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<u32> {
            Ok(input.foo)
        }
    }

    impl ThirdPreProcessor for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<FooMessage> {
            Ok(FooMessage::test(input.foo % (FOO_MOD + 2)))
        }
    }
    impl ThirdProcessor for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<FooMessage> {
            Ok(FooMessage::test(input.foo))
        }
    }

    #[test]
    fn handles_first() {
        let provider = TestMessagesProvider::construct();

        assert_eq!(true, provider.first(FooMessage::test(FOO_MOD)).unwrap());
        assert_eq!(false, provider.first(FooMessage::test(FOO_MOD + 1)).unwrap());
        assert_eq!(true, provider.first(FooMessage::test(FOO_MOD + 2)).unwrap());
    }

    #[test]
    fn handles_second() {
        let provider = TestMessagesProvider::construct();

        assert_eq!(FOO_MOD, provider.second(FooMessage::test(FOO_MOD)).unwrap());
        assert_eq!(0, provider.second(FooMessage::test(FOO_MOD + 1)).unwrap());
        assert_eq!(1, provider.second(FooMessage::test(FOO_MOD + 2)).unwrap());
    }

    #[test]
    fn handles_third() {
        let provider = TestMessagesProvider::construct();

        assert_eq!(FooMessage::test(FOO_MOD), provider.third(FooMessage::test(FOO_MOD)).unwrap());
        assert_eq!(FooMessage::test(FOO_MOD + 1), provider.third(FooMessage::test(FOO_MOD + 1)).unwrap());
        assert_eq!(FooMessage::test(0), provider.third(FooMessage::test(FOO_MOD + 2)).unwrap());
    }
}