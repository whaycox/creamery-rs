#[cfg(test)]
mod tests {
    use super::super::*;

    const FOO_MOD: u32 = 3;

    #[message_dispatch(TestMessages ! FooMessageError)]
    #[chain_message(FooMessage ~ FooMessageContext | [First, Second, Third] -> u32)]
    struct TestMessagesProvider {}

    impl ChainMessageFirst for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Option<Result<u32, FooMessageError>> {
            if input.foo % FOO_MOD == 0 {
                Some(Ok(FOO_MOD))
            }
            else {
                None
            }
        }
    }

    impl ChainMessageSecond for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Option<Result<u32, FooMessageError>> {
            if input.foo % (FOO_MOD + 1) == 0 {
                Some(Ok(FOO_MOD + 1))
            }
            else {
                None
            }
        }
    }

    impl ChainMessageThird for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Option<Result<u32, FooMessageError>> {
            if input.foo % (FOO_MOD + 2) == 0 {
                Some(Ok(FOO_MOD + 2))
            }
            else {
                None
            }
        }
    }

    #[whey_context(TestMessagesProvider)]
    struct ChainRequestContext {}

    #[whey(ChainRequestContext ~ context)]
    fn handled_by_first_stage() {
        let actual = context
            .test_type()
            .chain_message(FooMessage::test(FOO_MOD))
            .unwrap()
            .unwrap();
         
        assert_eq!(FOO_MOD, actual);
    }

    #[whey(ChainRequestContext ~ context)]
    fn handled_by_second_stage() {
        let actual = context
            .test_type()
            .chain_message(FooMessage::test(FOO_MOD + 1))
            .unwrap()
            .unwrap();
         
        assert_eq!(FOO_MOD + 1, actual);
    }

    #[whey(ChainRequestContext ~ context)]
    fn handled_by_third_stage() {
        let actual = context
            .test_type()
            .chain_message(FooMessage::test(FOO_MOD + 2))
            .unwrap()
            .unwrap();
         
        assert_eq!(FOO_MOD + 2, actual);
    }

    #[whey(ChainRequestContext ~ context)]
    fn unhandled() {
        assert_eq!(true, context.test_type().chain_message(FooMessage::test(7)).is_none());
    }
}
