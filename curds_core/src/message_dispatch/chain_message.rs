#[cfg(test)]
mod tests {
    use super::super::*;

    const FOO_MOD: u32 = 3;

    #[message_dispatch(TestMessages ! FooMessageError)]
    #[chain_message(FooMessage ~ FooMessageContext | [First, Second, Third])]
    struct TestMessagesProvider {}

    impl ChainMessageFirst for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Option<Result<(), FooMessageError>> {
            if input.foo % FOO_MOD == 0 {
                Some(Ok(()))
            }
            else {
                None
            }
        }
    }

    impl ChainMessageSecond for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Option<Result<(), FooMessageError>> {
            if input.foo % (FOO_MOD + 1) == 0 {
                Some(Ok(()))
            }
            else {
                None
            }
        }
    }

    impl ChainMessageThird for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Option<Result<(), FooMessageError>> {
            if input.foo % (FOO_MOD + 2) == 0 {
                Some(Ok(()))
            }
            else {
                None
            }
        }
    }

    #[whey_context(TestMessagesProvider)]
    struct ChainMessageContext {}

    #[whey(ChainMessageContext ~ context)]
    fn handled_by_first_stage() {
        context
            .test_type()
            .chain_message(FooMessage::test(FOO_MOD))
            .unwrap()
            .unwrap();
    }

    #[whey(ChainMessageContext ~ context)]
    fn handled_by_second_stage() {
        context
            .test_type()
            .chain_message(FooMessage::test(FOO_MOD + 1))
            .unwrap()
            .unwrap();
    }

    #[whey(ChainMessageContext ~ context)]
    fn handled_by_third_stage() {
        context
            .test_type()
            .chain_message(FooMessage::test(FOO_MOD + 2))
            .unwrap()
            .unwrap();
    }

    #[whey(ChainMessageContext ~ context)]
    fn unhandled() {         
        assert_eq!(true, context.test_type().chain_message(FooMessage::test(7)).is_none());
    }
}
