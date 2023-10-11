#[cfg(test)]
mod tests {
    use super::super::*;

    const FOO_MOD: u32 = 3;

    #[message_dispatch(TestMessages ! FooMessageError)]
    #[first(FooMessage ~ FooMessageContext |)]
    #[second(FooMessage ~ FooMessageContext | -> u32)]
    #[chain_default(Primary, Secondary, Tertiary)]
    struct TestMessagesProvider {}

    impl FirstPrimary for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Option<Result<(), FooMessageError>> {
            if input.foo % FOO_MOD == 0 {
                Some(Ok(()))
            }
            else {
                None
            }
        }
    }
    impl FirstSecondary for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Option<Result<(), FooMessageError>> {
            if input.foo % (FOO_MOD + 1) == 0 {
                Some(Ok(()))
            }
            else {
                None
            }
        }
    }
    impl FirstTertiary for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Option<Result<(), FooMessageError>> {
            if input.foo % (FOO_MOD + 2) == 0 {
                Some(Ok(()))
            }
            else {
                None
            }
        }
    }

    impl SecondPrimary for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Option<Result<u32, FooMessageError>> {
            if input.foo % FOO_MOD == 0 {
                Some(Ok(1))
            }
            else {
                None
            }
        }
    }
    impl SecondSecondary for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Option<Result<u32, FooMessageError>> {
            if input.foo % (FOO_MOD + 1) == 0 {
                Some(Ok(2))
            }
            else {
                None
            }
        }
    }
    impl SecondTertiary for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Option<Result<u32, FooMessageError>> {
            if input.foo % (FOO_MOD + 2) == 0 {
                Some(Ok(3))
            }
            else {
                None
            }
        }
    }

    #[whey_context(TestMessagesProvider)]
    struct DefaultChainContext {}

    #[whey(DefaultChainContext ~ context)]
    fn handles_first() {
        let provider = context.test_type();

        assert_eq!(true, provider.first(FooMessage::test(FOO_MOD - 1)).is_none());
        assert_eq!((), provider.first(FooMessage::test(FOO_MOD)).unwrap().unwrap());
        assert_eq!((), provider.first(FooMessage::test(FOO_MOD + 1)).unwrap().unwrap());
        assert_eq!((), provider.first(FooMessage::test(FOO_MOD + 2)).unwrap().unwrap());
        assert_eq!((), provider.first(FooMessage::test(FOO_MOD + 3)).unwrap().unwrap());
        assert_eq!(true, provider.first(FooMessage::test(FOO_MOD + 4)).is_none());
    }

    #[whey(DefaultChainContext ~ context)]
    fn handles_second() {
        let provider = context.test_type();

        assert_eq!(true, provider.second(FooMessage::test(FOO_MOD - 1)).is_none());
        assert_eq!(1, provider.second(FooMessage::test(FOO_MOD)).unwrap().unwrap());
        assert_eq!(2, provider.second(FooMessage::test(FOO_MOD + 1)).unwrap().unwrap());
        assert_eq!(3, provider.second(FooMessage::test(FOO_MOD + 2)).unwrap().unwrap());
        assert_eq!(1, provider.second(FooMessage::test(FOO_MOD + 3)).unwrap().unwrap());
        assert_eq!(true, provider.second(FooMessage::test(FOO_MOD + 4)).is_none());
    }
}