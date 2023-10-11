#[cfg(test)]
mod tests {
    use super::super::*;

    const FOO_MOD: u32 = 3;

    #[message_dispatch(TestMessages ! FooMessageError)]
    #[first(FooMessage ~ FooMessageContext -> bool)]
    #[second(FooMessage ~ FooMessageContext -> u32)]
    #[third(FooMessage ~ FooMessageContext & -> FooMessage)]
    #[pipeline_default(PreProcessor -> $message, Processor)]
    struct TestMessagesProvider {}

    impl FirstPreProcessor for FooMessageContext {
        fn handle(&self, _: &dyn TestMessages, input: FooMessage) -> Result<FooMessage, FooMessageError> {
            Ok(FooMessage::test(input.foo % FOO_MOD))
        }
    }
    impl FirstProcessor for FooMessageContext {
        fn handle(&self, _: &dyn TestMessages, input: FooMessage) -> Result<bool, FooMessageError> {
            Ok(input.foo % 2 == 0)
        }
    }

    impl SecondPreProcessor for FooMessageContext {
        fn handle(&self, _: &dyn TestMessages, input: FooMessage) -> Result<FooMessage, FooMessageError> {
            Ok(FooMessage::test(input.foo % (FOO_MOD + 1)))
        }
    }
    impl SecondProcessor for FooMessageContext {
        fn handle(&self, _: &dyn TestMessages, input: FooMessage) -> Result<u32, FooMessageError> {
            Ok(input.foo)
        }
    }

    impl ThirdPreProcessor for FooMessageContext {
        fn handle(&self, _: &dyn TestMessages, input: FooMessage) -> Result<FooMessage, FooMessageError> {
            Ok(FooMessage::test(input.foo % (FOO_MOD + 2)))
        }
    }
    impl ThirdProcessor for FooMessageContext {
        fn handle(&self, _: &dyn TestMessages, input: FooMessage) -> Result<FooMessage, FooMessageError> {
            Ok(FooMessage::test(input.foo))
        }
    }

    #[whey_context(TestMessagesProvider)]
    struct DefaultPipelineRequestContext {}

    #[whey(DefaultPipelineRequestContext ~ context)]
    fn handles_first() {
        let provider = context.test_type();

        assert_eq!(true, provider.first(FooMessage::test(FOO_MOD)).unwrap());
        assert_eq!(false, provider.first(FooMessage::test(FOO_MOD + 1)).unwrap());
        assert_eq!(true, provider.first(FooMessage::test(FOO_MOD + 2)).unwrap());
    }

    #[whey(DefaultPipelineRequestContext ~ context)]
    fn handles_second() {
        let provider = context.test_type();

        assert_eq!(FOO_MOD, provider.second(FooMessage::test(FOO_MOD)).unwrap());
        assert_eq!(0, provider.second(FooMessage::test(FOO_MOD + 1)).unwrap());
        assert_eq!(1, provider.second(FooMessage::test(FOO_MOD + 2)).unwrap());
    }

    #[whey(DefaultPipelineRequestContext ~ context)]
    fn handles_third() {
        let provider = context.test_type();

        assert_eq!(FooMessage::test(FOO_MOD), provider.third(FooMessage::test(FOO_MOD)).unwrap());
        assert_eq!(FooMessage::test(FOO_MOD + 1), provider.third(FooMessage::test(FOO_MOD + 1)).unwrap());
        assert_eq!(FooMessage::test(0), provider.third(FooMessage::test(FOO_MOD + 2)).unwrap());
    }
}