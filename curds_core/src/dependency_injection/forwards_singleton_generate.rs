#[cfg(test)]
mod tests {
    use super::super::*;

    #[injected]
    #[defaults(value)]
    struct IncrementingFoo {
        value: Cell<u32>,
    }
    impl Foo for IncrementingFoo {
        fn foo(&self) -> u32 {
            let value = self.value.get();
            self.value.set(value + 1);
            value
        }
    }

    #[service_provider]
    #[generates(IncrementingFoo)]
    struct BaseProvider {}

    #[service_provider]
    #[forwards_singleton(IncrementingFoo <- base)]
    struct ForwardedProvider {
        base: Rc<BaseProvider>
    }
    impl ForwardedProvider {
        pub fn new() -> Self {
            Self::construct(
                Rc::new(BaseProvider::construct()))
        }
    }

    #[test]
    fn forwarded_provider_is_singleton() {
        let provider = ForwardedProvider::new();

        for i in 0..10 {
            let foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&provider);

            assert_eq!(3 * i, foo.foo());
            assert_eq!(3 * i + 1, foo.foo());
            assert_eq!(3 * i + 2, foo.foo());
        }
    }

    #[test]
    fn underlying_provider_is_transient() {
        let provider = BaseProvider::construct();

        for _ in 0..10 {
            let foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&provider);

            assert_eq!(0, foo.foo());
            assert_eq!(1, foo.foo());
            assert_eq!(2, foo.foo());
        }
    }
}
