#[cfg(test)]
mod tests {
    use super::super::*;

    const FOO_MOD: u32 = 3;

    #[message_dispatch(TestMessages)]
    #[first(FooMessage ~ FooRepositoryContext)]
    #[second(FooMessage ~ FooRepositoryContext)]
    #[third(FooMessage ~ FooRepositoryContext)]
    #[pipeline(Validator, Handler)]
    #[generates_singleton(dyn FooRepository ~ ConcreteRepository)]
    struct TestMessagesProvider {}

    impl FirstValidator for FooRepositoryContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<()> {
            if input.foo % FOO_MOD == 0 {
                Ok(())
            }
            else {
                Err(Box::new(FooMessageError::new(&format!("Foo wasn't a multiple of {}", FOO_MOD))))
            }
        }
    }
    impl FirstHandler for FooRepositoryContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<()> {
            self.repo.store(input.foo);
            Ok(())
        }
    }

    impl SecondValidator for FooRepositoryContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<()> {
            if input.foo % (FOO_MOD + 1) == 0 {
                Ok(())
            }
            else {
                Err(Box::new(FooMessageError::new(&format!("Foo wasn't a multiple of {}", FOO_MOD + 1))))
            }
        }
    }
    impl SecondHandler for FooRepositoryContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<()> {
            self.repo.store(input.foo + 1);
            Ok(())
        }
    }

    impl ThirdValidator for FooRepositoryContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<()> {
            if input.foo % (FOO_MOD + 2) == 0 {
                Ok(())
            }
            else {
                Err(Box::new(FooMessageError::new(&format!("Foo wasn't a multiple of {}", FOO_MOD + 2))))
            }
        }
    }
    impl ThirdHandler for FooRepositoryContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<()> {
            self.repo.store(input.foo + 2);
            Ok(())
        }
    }

    #[test]
    fn handles_first_successes() {
        let provider = TestMessagesProvider::construct();
        let repo = ServiceGenerator::<Rc<dyn FooRepository>>::generate(&provider);

        assert_eq!((), provider.first(FooMessage::test(FOO_MOD)).unwrap());
        assert_eq!(FOO_MOD, repo.get().unwrap());
        assert_eq!((), provider.first(FooMessage::test(FOO_MOD * FOO_MOD)).unwrap());
        assert_eq!(FOO_MOD * FOO_MOD, repo.get().unwrap());
    }

    #[test]
    fn handles_first_failure() {
        let provider = TestMessagesProvider::construct();
        let repo = ServiceGenerator::<Rc<dyn FooRepository>>::generate(&provider);

        provider.first(FooMessage::test(FOO_MOD + 1)).unwrap_err();
        assert_eq!(None, repo.get());
    }

    #[test]
    fn handles_second_successes() {
        let provider = TestMessagesProvider::construct();
        let repo = ServiceGenerator::<Rc<dyn FooRepository>>::generate(&provider);

        assert_eq!((), provider.second(FooMessage::test(FOO_MOD + 1)).unwrap());
        assert_eq!(FOO_MOD + 2, repo.get().unwrap());
        assert_eq!((), provider.second(FooMessage::test((FOO_MOD + 1) * (FOO_MOD + 1))).unwrap());
        assert_eq!(((FOO_MOD + 1) * (FOO_MOD + 1) + 1), repo.get().unwrap());
    }

    #[test]
    fn handles_second_failure() {
        let provider = TestMessagesProvider::construct();
        let repo = ServiceGenerator::<Rc<dyn FooRepository>>::generate(&provider);

        provider.second(FooMessage::test(FOO_MOD + 2)).unwrap_err();
        assert_eq!(None, repo.get());
    }

    #[test]
    fn handles_third_successes() {
        let provider = TestMessagesProvider::construct();
        let repo = ServiceGenerator::<Rc<dyn FooRepository>>::generate(&provider);

        assert_eq!((), provider.third(FooMessage::test(FOO_MOD + 2)).unwrap());
        assert_eq!(FOO_MOD + 4, repo.get().unwrap());
        assert_eq!((), provider.third(FooMessage::test((FOO_MOD + 2) * (FOO_MOD + 2))).unwrap());
        assert_eq!(((FOO_MOD + 2) * (FOO_MOD + 2) + 2), repo.get().unwrap());
    }

    #[test]
    fn handles_third_failure() {
        let provider = TestMessagesProvider::construct();
        let repo = ServiceGenerator::<Rc<dyn FooRepository>>::generate(&provider);

        provider.third(FooMessage::test(FOO_MOD + 3)).unwrap_err();
        assert_eq!(None, repo.get());
    }
}
