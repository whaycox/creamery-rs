#[cfg(test)]
mod tests {
    use super::super::*;

    const FOO_MOD: u32 = 3;

    #[message_dispatch(TestMessages ! FooMessageError)]
    #[first(FooMessage ~ FooRepositoryContext)]
    #[second(FooMessage ~ FooRepositoryContext)]
    #[third(FooMessage ~ FooRepositoryContext &)]
    #[pipeline_default(Validator, Handler)]
    #[generates_singleton(dyn FooRepository ~ ConcreteRepository)]
    struct TestMessagesProvider {}

    impl FirstValidator for FooRepositoryContext {
        fn handle(&self, _: &dyn TestMessages, input: &FooMessage) -> Result<(), FooMessageError> {
            if input.foo % FOO_MOD == 0 {
                Ok(())
            }
            else {
                Err(FooMessageError {})
            }
        }
    }

    impl FirstHandler for FooRepositoryContext {
        fn handle(&self, _: &dyn TestMessages, input: FooMessage) -> Result<(), FooMessageError> {
            self.repo
                .write()
                .unwrap()
                .store(input.foo);
            Ok(())
        }
    }

    impl SecondValidator for FooRepositoryContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<(), FooMessageError> {
            if input.foo % (FOO_MOD + 1) == 0 {
                Ok(())
            }
            else {
                Err(FooMessageError {})
            }
        }
    }
    impl SecondHandler for FooRepositoryContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: FooMessage) -> Result<(), FooMessageError> {
            self.repo
                .write()
                .unwrap()
                .store(input.foo + 1);
            Ok(())
        }
    }

    impl ThirdValidator for FooRepositoryContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<(), FooMessageError> {
            if input.foo % (FOO_MOD + 2) == 0 {
                Ok(())
            }
            else {
                Err(FooMessageError {})
            }
        }
    }
    impl ThirdHandler for FooRepositoryContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: FooMessage) -> Result<(), FooMessageError> {
            self.repo
                .write()
                .unwrap()
                .store(input.foo + 2);
            Ok(())
        }
    }

    #[whey_context(TestMessagesProvider)]
    struct DefaultPipelineMessageContext {}

    #[whey(DefaultPipelineMessageContext ~ context)]
    fn handles_first_successes() {
        let provider = context.test_type();
        let repo: Singleton<Box<dyn FooRepository>> = provider.generate();

        assert_eq!((), provider.first(FooMessage::test(FOO_MOD)).unwrap());
        assert_eq!(FOO_MOD, repo.read().unwrap().get().unwrap());
        assert_eq!((), provider.first(FooMessage::test(FOO_MOD * FOO_MOD)).unwrap());
        assert_eq!(FOO_MOD * FOO_MOD, repo.read().unwrap().get().unwrap());
    }

    #[whey(DefaultPipelineMessageContext ~ context)]
    fn handles_first_failure() {
        let provider = context.test_type();
        let repo: Singleton<Box<dyn FooRepository>> = provider.generate();

        provider.first(FooMessage::test(FOO_MOD + 1)).unwrap_err();
        assert_eq!(None, *repo.read().unwrap().get());
    }

    #[whey(DefaultPipelineMessageContext ~ context)]
    fn handles_second_successes() {
        let provider = context.test_type();
        let repo: Singleton<Box<dyn FooRepository>> = provider.generate();

        assert_eq!((), provider.second(FooMessage::test(FOO_MOD + 1)).unwrap());
        assert_eq!(FOO_MOD + 2, repo.read().unwrap().get().unwrap());
        assert_eq!((), provider.second(FooMessage::test((FOO_MOD + 1) * (FOO_MOD + 1))).unwrap());
        assert_eq!(((FOO_MOD + 1) * (FOO_MOD + 1) + 1), repo.read().unwrap().get().unwrap());
    }

    #[whey(DefaultPipelineMessageContext ~ context)]
    fn handles_second_failure() {
        let provider = context.test_type();
        let repo: Singleton<Box<dyn FooRepository>> = provider.generate();

        provider.second(FooMessage::test(FOO_MOD + 2)).unwrap_err();
        assert_eq!(None, *repo.read().unwrap().get());
    }

    #[whey(DefaultPipelineMessageContext ~ context)]
    fn handles_third_successes() {
        let provider = context.test_type();
        let repo: Singleton<Box<dyn FooRepository>> = provider.generate();

        assert_eq!((), provider.third(FooMessage::test(FOO_MOD + 2)).unwrap());
        assert_eq!(FOO_MOD + 4, repo.read().unwrap().get().unwrap());
        assert_eq!((), provider.third(FooMessage::test((FOO_MOD + 2) * (FOO_MOD + 2))).unwrap());
        assert_eq!(((FOO_MOD + 2) * (FOO_MOD + 2) + 2), repo.read().unwrap().get().unwrap());
    }

    #[whey(DefaultPipelineMessageContext ~ context)]
    fn handles_third_failure() {
        let provider = context.test_type();
        let repo: Singleton<Box<dyn FooRepository>> = provider.generate();

        provider.third(FooMessage::test(FOO_MOD + 3)).unwrap_err();
        assert_eq!(None, *repo.read().unwrap().get());
    }
}