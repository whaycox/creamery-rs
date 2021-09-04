#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates(IncrementingFoo)]
    #[generates(dyn Foo <- IncrementingFoo)]
    struct BaseProvider {}

    #[service_provider]
    #[forwards_singleton(IncrementingFoo <- base)]
    struct ForwardedStructProvider {
        base: Rc<BaseProvider>,
    }
    impl ForwardedStructProvider {
        fn new() -> Rc<Self> {
            Self::construct(BaseProvider::construct())
        }
    }

    #[test]
    fn forwards_generate_struct_to_base_but_stores_as_singleton() {
        let provider = ForwardedStructProvider::new();

        for i in 0..10 {
            let foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&*provider);

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }

    #[service_provider]
    #[forwards_singleton(dyn Foo <- IncrementingFoo <- base)]
    struct ForwardedTraitProvider {
        base: Rc<BaseProvider>,
    }
    impl ForwardedTraitProvider {
        fn new() -> Rc<Self> {
            Self::construct(BaseProvider::construct())
        }
    }

    #[test]
    fn generates_trait_via_base_but_stores_concrete_as_singleton() {
        let provider = ForwardedTraitProvider::new();

        for i in 0..10 {
            let foo = ServiceGenerator::<Rc<dyn Foo>>::generate(&*provider);

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }
}
