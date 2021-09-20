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
    #[clones(base)]
    struct ClonedStructProvider {
        base: Rc<BaseProvider>,
    }

    #[test]
    fn generated_provider_is_cloned_from_base_each_time() {
        let base_provider = BaseProvider::construct();
        let provider = ClonedStructProvider::construct(base_provider.clone());
        let base_foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&base_provider);

        for i in 0..10 {
            let cloned_provider = ServiceGenerator::<Rc<BaseProvider>>::generate(&provider);
            let foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&cloned_provider);

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, base_foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }

    #[service_provider]
    #[clones(dyn Bar ~ base)]
    struct ClonedTraitProvider {
        base: Rc<BaseProvider>,
    }

    #[test]
    fn generated_provider_as_trait_is_cloned_from_base_each_time() {
        let base_provider = BaseProvider::construct();
        let provider = ClonedTraitProvider::construct(base_provider.clone());
        let base_foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&base_provider);

        for i in 0..10 {
            let cloned_provider_trait = ServiceGenerator::<Rc<dyn Bar>>::generate(&provider);

            assert_eq!(i * 3, cloned_provider_trait.bar());
            assert_eq!(i * 3 + 1, base_foo.foo());
            assert_eq!(i * 3 + 2, cloned_provider_trait.bar());
        }
    }
}
