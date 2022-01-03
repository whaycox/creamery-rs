#[cfg(test)]
mod tests {
    use super::super::*;

    const FOO_MOD: u32 = 3;

    #[message_dispatch(TestMessages)]
    #[first(FooMessage ~ FooMessageContext |)]
    #[second(FooMessage ~ FooMessageContext | -> u32)]
    #[chain(Primary, Secondary, Tertiary)]
    struct TestMessagesProvider {}

    impl FirstPrimary for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<Option<()>> {
            if input.foo % FOO_MOD == 0 {
                Ok(Some(()))
            }
            else {
                Ok(None)
            }
        }
    }
    impl FirstSecondary for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<Option<()>> {
            if input.foo % (FOO_MOD + 1) == 0 {
                Ok(Some(()))
            }
            else {
                Ok(None)
            }
        }
    }
    impl FirstTertiary for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<Option<()>> {
            if input.foo % (FOO_MOD + 2) == 0 {
                Ok(Some(()))
            }
            else {
                Ok(None)
            }
        }
    }

    impl SecondPrimary for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<Option<u32>> {
            if input.foo % FOO_MOD == 0 {
                Ok(Some(1))
            }
            else {
                Ok(None)
            }
        }
    }
    impl SecondSecondary for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<Option<u32>> {
            if input.foo % (FOO_MOD + 1) == 0 {
                Ok(Some(2))
            }
            else {
                Ok(None)
            }
        }
    }
    impl SecondTertiary for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<Option<u32>> {
            if input.foo % (FOO_MOD + 2) == 0 {
                Ok(Some(3))
            }
            else {
                Ok(None)
            }
        }
    }

    #[test]
    fn handles_first() {
        let provider = TestMessagesProvider::construct();

        assert_eq!(None, provider.first(FooMessage::test(FOO_MOD - 1)).unwrap());
        assert_eq!(Some(()), provider.first(FooMessage::test(FOO_MOD)).unwrap());
        assert_eq!(Some(()), provider.first(FooMessage::test(FOO_MOD + 1)).unwrap());
        assert_eq!(Some(()), provider.first(FooMessage::test(FOO_MOD + 2)).unwrap());
        assert_eq!(Some(()), provider.first(FooMessage::test(FOO_MOD + 3)).unwrap());
        assert_eq!(None, provider.first(FooMessage::test(FOO_MOD + 4)).unwrap());
    }

    #[test]
    fn handles_second() {
        let provider = TestMessagesProvider::construct();

        assert_eq!(None, provider.second(FooMessage::test(FOO_MOD - 1)).unwrap());
        assert_eq!(Some(1), provider.second(FooMessage::test(FOO_MOD)).unwrap());
        assert_eq!(Some(2), provider.second(FooMessage::test(FOO_MOD + 1)).unwrap());
        assert_eq!(Some(3), provider.second(FooMessage::test(FOO_MOD + 2)).unwrap());
        assert_eq!(Some(1), provider.second(FooMessage::test(FOO_MOD + 3)).unwrap());
        assert_eq!(None, provider.second(FooMessage::test(FOO_MOD + 4)).unwrap());
    }
}