#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates(ConcreteFoo)]
    struct StructProvider {}
    
    #[test]
    fn generates_foo_struct() {
        let provider = StructProvider::construct();
        let foo = ServiceGenerator::<Rc<ConcreteFoo>>::generate(&provider);

        assert_eq!(EXPECTED_FOO, foo.foo())
    }

    #[service_provider]
    #[generates(dyn Foo ~ ConcreteFoo)]
    struct TraitProvider {}

    #[test]
    fn generates_foo_trait() {
        let provider = TraitProvider::construct();
        let foo = ServiceGenerator::<Rc<dyn Foo>>::generate(&provider);

        assert_eq!(EXPECTED_FOO, foo.foo())
    }
}
