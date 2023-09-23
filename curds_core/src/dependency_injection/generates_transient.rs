#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates(ConcreteFoo)]
    struct StructProvider {}

    #[test]
    fn generates_foo_struct() {
        let mut provider = StructProvider::construct();
        let mut foo: ConcreteFoo = provider.generate();

        assert_eq!(EXPECTED_FOO, foo.foo())
    }

    #[service_provider]
    #[generates(dyn Foo ~ ConcreteFoo)]
    struct TraitProvider {}

    #[test]
    fn generates_foo_trait() {
        let mut provider = TraitProvider::construct();
        let mut foo: Box<dyn Foo> = provider.generate();

        assert_eq!(EXPECTED_FOO, foo.foo())
    }
}
