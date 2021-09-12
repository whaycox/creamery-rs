#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates(IncrementingFoo)]
    #[generates(dyn Foo <- IncrementingFoo)]
    struct BaseProvider {}

    #[service_provider]
    #[forwards_singleton(dyn Foo <- IncrementingFoo <- base)]
    #[forwards_singleton(IncrementingFoo <- base)]
    struct ForwardedProvider {
        base: Rc<BaseProvider>,
    }
    impl ForwardedProvider {
        fn new() -> Rc<Self> {
            Self::construct(BaseProvider::construct())
        }
    }

    #[test]
    fn forwards_generate_struct_to_base_but_stores_as_singleton() {
        let provider = ForwardedProvider::new();

        for i in 0..10 {
            let foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&provider);

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }

    #[test]
    fn generates_trait_via_base_but_stores_concrete_as_singleton() {
        let provider = ForwardedProvider::new();

        for i in 0..10 {
            let foo = ServiceGenerator::<Rc<dyn Foo>>::generate(&provider);

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }

    #[test]
    fn trait_and_struct_same_singleton() {
        let provider = ForwardedProvider::new();

        for i in 0..10 {
            let foo_trait = ServiceGenerator::<Rc<dyn Foo>>::generate(&provider);
            let foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&provider);

            assert_eq!(i * 3, foo_trait.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo_trait.foo());
        }
    }

    #[service_provider]
    #[forwards_singleton(dyn Foo <- base)]
    #[forwards_singleton(IncrementingFoo <- base)]
    struct ForwardedWithoutIntermediateProvider {
        base: Rc<BaseProvider>,
    }
    impl ForwardedWithoutIntermediateProvider {
        fn new() -> Rc<Self> {
            Self::construct(BaseProvider::construct())
        }
    }

    #[test]
    fn can_store_trait_singleton_without_intermediate() {
        let provider = ForwardedWithoutIntermediateProvider::new();

        for i in 0..10 {
            let foo = ServiceGenerator::<Rc<dyn Foo>>::generate(&provider);

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }

    #[test]
    fn trait_singleton_without_intermediate_is_stored_separately() {
        let provider = ForwardedWithoutIntermediateProvider::new();

        for i in 0..10 {
            let foo_trait = ServiceGenerator::<Rc<dyn Foo>>::generate(&provider);
            let foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&provider);

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3, foo_trait.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 1, foo_trait.foo());
            assert_eq!(i * 3 + 2, foo.foo());
            assert_eq!(i * 3 + 2, foo_trait.foo());
        }
    }
}
