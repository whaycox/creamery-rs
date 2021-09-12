#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates(ConcreteFoo)]
    #[generates(dyn Foo <- ConcreteFoo)]
    struct BaseProvider {}

    #[service_provider]
    #[forwards(ConcreteFoo <- base)]
    struct ForwardedStructProvider {
        base: Rc<BaseProvider>,
    }
    impl ForwardedStructProvider {
        fn new() -> Rc<Self> {
            Self::construct(BaseProvider::construct())
        }
    }

    #[test]
    fn forwards_generate_struct_to_base() {
        let provider = ForwardedStructProvider::new();
        let foo = ServiceGenerator::<Rc<ConcreteFoo>>::generate(&provider);

        assert_eq!(EXPECTED_FOO, foo.foo())
    }

    #[service_provider]
    #[forwards(dyn Foo <- base)]
    struct ForwardedTraitProvider {
        base: Rc<BaseProvider>,
    }
    impl ForwardedTraitProvider {
        fn new() -> Rc<Self> {
            Self::construct(BaseProvider::construct())
        }
    }

    #[test]
    fn forwards_generate_trait_to_base() {
        let provider = ForwardedTraitProvider::new();
        let foo = ServiceGenerator::<Rc<dyn Foo>>::generate(&provider);

        assert_eq!(EXPECTED_FOO, foo.foo())
    }

    #[service_provider]
    #[forwards(dyn Foo <- ConcreteFoo <- base)]
    struct ForwardedIntermediateProvider {
        base: Rc<BaseProvider>,
    }
    impl ForwardedIntermediateProvider {
        fn new() -> Rc<Self> {
            Self::construct(BaseProvider::construct())
        }
    }

    #[test]
    fn forwards_generate_trait_via_concrete_to_base() {
        let provider = ForwardedIntermediateProvider::new();
        let foo = ServiceGenerator::<Rc<dyn Foo>>::generate(&provider);

        assert_eq!(EXPECTED_FOO, foo.foo())
    }
}
