#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates_singleton(IncrementingFoo)]
    struct BaseProvider {}
    impl Bar for BaseProvider {
        fn bar(&self) -> u32 {
            let foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&*self);
            foo.foo()
        }
    }

    #[service_provider]
    #[scopes(base)]
    struct ScopedStructProvider {
        base: Rc<BaseProvider>,
    }

    #[test]
    fn scoped_provider_doesnt_keep_parent_singletons() {
        let base_provider = BaseProvider::construct();
        let provider = ScopedStructProvider::construct(base_provider.clone());
        let base_foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&*base_provider);

        for i in 0..10 {
            let scoped_provider = ServiceGenerator::<Rc<BaseProvider>>::generate(&*provider);
            let foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&*scoped_provider);

            assert_eq!(i, base_foo.foo());
            assert_eq!(0, foo.foo());
            assert_eq!(1, foo.foo());
            assert_eq!(2, foo.foo());
        }
    }

    #[service_provider]
    #[scopes(dyn Bar <- base)]
    struct ScopedTraitProvider {
        base: Rc<BaseProvider>,
    }

    #[test]
    fn scoped_provider_as_trait_doesnt_keep_parent_singletons() {
        let base_provider = BaseProvider::construct();
        let provider = ScopedTraitProvider::construct(base_provider.clone());
        let base_foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&*base_provider);

        for i in 0..10 {
            let scoped_provider = ServiceGenerator::<Rc<dyn Bar>>::generate(&*provider);

            assert_eq!(i, base_foo.foo());
            assert_eq!(0, scoped_provider.bar());
            assert_eq!(1, scoped_provider.bar());
            assert_eq!(2, scoped_provider.bar());
        }
    }
}
