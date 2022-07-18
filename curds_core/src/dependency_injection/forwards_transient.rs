#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates(ConcreteFoo)]
    #[generates(dyn Foo ~ ConcreteFoo)]
    struct BaseProvider {}

    #[service_provider]
    #[forwards(ConcreteFoo ~ base)]
    struct ForwardedStructProvider {
        base: BaseProvider,
    }
    impl ForwardedStructProvider {
        fn new() -> Self {
            Self::construct(BaseProvider::construct())
        }
    }

    #[test]
    fn forwards_generate_struct_to_base() {
        let provider = ForwardedStructProvider::new();
        let foo: ConcreteFoo = provider.generate();

        assert_eq!(EXPECTED_FOO, foo.foo())
    }

    #[service_provider]
    #[forwards(dyn Foo ~ base)]
    struct ForwardedTraitProvider {
        base: BaseProvider,
    }
    impl ForwardedTraitProvider {
        fn new() -> Self {
            Self::construct(BaseProvider::construct())
        }
    }

    #[test]
    fn forwards_generate_trait_to_base() {
        let provider = ForwardedTraitProvider::new();
        let foo: Rc<dyn Foo> = provider.generate();

        assert_eq!(EXPECTED_FOO, foo.foo())
    }

    #[service_provider]
    #[forwards(dyn Foo ~ ConcreteFoo ~ base)]
    struct ForwardedIntermediateProvider {
        base: BaseProvider,
    }
    impl ForwardedIntermediateProvider {
        fn new() -> Self {
            Self::construct(BaseProvider::construct())
        }
    }

    #[test]
    fn forwards_generate_trait_via_concrete_to_base() {
        let provider = ForwardedIntermediateProvider::new();
        let foo: Rc<dyn Foo> = provider.generate();

        assert_eq!(EXPECTED_FOO, foo.foo())
    }
}
