#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates(ConcreteFoo)]
    pub struct TestServiceProvider {}

    #[injected]
    struct ConcreteFoo {}
    impl Foo for ConcreteFoo {
        fn foo(&self) -> u32 { EXPECTED_FOO }
    }

    #[test]
    fn generates_concrete_foo() {
        let provider = TestServiceProvider::construct();
        let foo = ServiceGenerator::<Rc<ConcreteFoo>>::generate(&provider);

        assert_eq!(EXPECTED_FOO, foo.foo())
    }
}
