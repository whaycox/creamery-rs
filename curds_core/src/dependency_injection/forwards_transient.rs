#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates(ConcreteFoo)]
    #[generates(ForwardedStructProvider)]
    #[generates(ForwardedIntermediateProvider)]
    #[clones_self]
    #[derive(Clone)]
    struct BaseStructProvider {}

    #[service_provider]
    #[forwards(ConcreteFoo ~ base)]
    struct ForwardedStructProvider {
        base: BaseStructProvider,
    }

    #[test]
    fn forwards_generate_struct_to_base() {
        let base_provider: BaseStructProvider = BaseStructProvider::construct();
        let provider: ForwardedStructProvider = base_provider.generate();
        let mut foo: ConcreteFoo = provider.generate();

        assert_eq!(EXPECTED_FOO, foo.foo())
    }

    #[service_provider]
    #[generates(dyn Foo ~ ConcreteFoo)]
    #[generates(ForwardedTraitProvider)]
    #[clones_self]
    #[derive(Clone)]
    struct BaseTraitProvider {}

    #[service_provider]
    #[forwards(dyn Foo ~ base)]
    struct ForwardedTraitProvider {
        base: BaseTraitProvider,
    }

    #[test]
    fn forwards_generate_trait_to_base() {
        let base_provider: BaseTraitProvider = BaseTraitProvider::construct();
        let provider: ForwardedTraitProvider = base_provider.generate();
        let mut foo: Box<dyn Foo> = provider.generate();

        assert_eq!(EXPECTED_FOO, foo.foo())
    }

    #[service_provider]
    #[forwards(dyn Foo ~ ConcreteFoo ~ base)]
    struct ForwardedIntermediateProvider {
        base: BaseStructProvider,
    }

    #[test]
    fn forwards_generate_trait_via_concrete_to_base() {
        let base_provider: BaseStructProvider = BaseStructProvider::construct();
        let provider: ForwardedIntermediateProvider = base_provider.generate();
        let mut foo: Box<dyn Foo> = provider.generate();

        assert_eq!(EXPECTED_FOO, foo.foo())
    }
}
