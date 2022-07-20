#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates(ConcreteFoo)]
    #[generates(dyn Foo ~ ConcreteFoo)]
    #[generates(ForwardedStructProvider)]
    #[generates(ForwardedTraitProvider)]
    #[generates(ForwardedIntermediateProvider)]
    #[clones_self]
    #[derive(Clone)]
    struct BaseProvider {}

    #[service_provider]
    #[forwards(ConcreteFoo ~ base)]
    struct ForwardedStructProvider {
        base: BaseProvider,
    }

    #[test]
    fn forwards_generate_struct_to_base() {
        let base_provider: BaseProvider = BaseProvider::construct();
        let provider: ForwardedStructProvider = base_provider.generate();
        let foo: ConcreteFoo = provider.generate();

        assert_eq!(EXPECTED_FOO, foo.foo())
    }

    #[service_provider]
    #[forwards(dyn Foo ~ base)]
    struct ForwardedTraitProvider {
        base: BaseProvider,
    }

    #[test]
    fn forwards_generate_trait_to_base() {
        let base_provider: BaseProvider = BaseProvider::construct();
        let provider: ForwardedTraitProvider = base_provider.generate();
        let foo: Box<dyn Foo> = provider.generate();

        assert_eq!(EXPECTED_FOO, foo.foo())
    }

    #[service_provider]
    #[forwards(dyn Foo ~ ConcreteFoo ~ base)]
    struct ForwardedIntermediateProvider {
        base: BaseProvider,
    }

    #[test]
    fn forwards_generate_trait_via_concrete_to_base() {
        let base_provider: BaseProvider = BaseProvider::construct();
        let provider: ForwardedIntermediateProvider = base_provider.generate();
        let foo: Box<dyn Foo> = provider.generate();

        assert_eq!(EXPECTED_FOO, foo.foo())
    }
}
