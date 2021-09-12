#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates_singleton(IncrementingFoo)]
    struct BaseProvider {}
    impl Bar for BaseProvider {
        fn bar(&self) -> u32 {
            let foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(self);
            foo.foo()
        }
    }

    #[service_provider]
    #[scopes_singleton(dyn Bar <- base)]
    #[scopes_singleton(base)]
    struct ScopedProvider {
        base: Rc<BaseProvider>,
    }

    #[test]
    fn scoped_provider_is_stored_as_singleton() {
        let base_provider = BaseProvider::construct();
        let provider = ScopedProvider::construct(base_provider.clone());

        for i in 0..10 {
            let base_foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&base_provider);
            let scoped_provider = ServiceGenerator::<Rc<BaseProvider>>::generate(&provider);
            let foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&scoped_provider);

            assert_eq!(i, base_foo.foo());
            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }

    #[test]
    fn scoped_trait_provider_is_stored_as_singleton() {
        let base_provider = BaseProvider::construct();
        let provider = ScopedProvider::construct(base_provider.clone());

        for i in 0..10 {
            let base_foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&base_provider);
            let scoped_provider = ServiceGenerator::<Rc<dyn Bar>>::generate(&provider);

            assert_eq!(i, base_foo.foo());
            assert_eq!(i * 3, scoped_provider.bar());
            assert_eq!(i * 3 + 1, scoped_provider.bar());
            assert_eq!(i * 3 + 2, scoped_provider.bar());
        }
    }

    #[test]
    fn scoped_provider_is_stored_same_as_trait_or_struct() {
        let base_provider = BaseProvider::construct();
        let provider = ScopedProvider::construct(base_provider.clone());

        for i in 0..10 {
            let base_foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&base_provider);
            let struct_provider = ServiceGenerator::<Rc<BaseProvider>>::generate(&provider);
            let foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&struct_provider);
            let trait_provider = ServiceGenerator::<Rc<dyn Bar>>::generate(&provider);

            assert_eq!(i, base_foo.foo());
            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, trait_provider.bar());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }
}
