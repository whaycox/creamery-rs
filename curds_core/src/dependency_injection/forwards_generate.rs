#[cfg(test)]
mod tests {
    use super::super::*;

    #[injected]
    struct ConcreteFoo {}
    impl Foo for ConcreteFoo {
        fn foo(&self) -> u32 { EXPECTED_FOO }
    }

    #[service_provider]
    #[generates(ConcreteFoo)]
    struct BaseProvider {}

    #[service_provider]
    #[forwards(ConcreteFoo <- base)]
    struct TestServiceProvider {
        base: Rc<BaseProvider>
    }
    impl TestServiceProvider {
        pub fn new() -> Self {
            Self::construct(
                Rc::new(BaseProvider::construct()))
        }
    }

    #[test]
    fn generates_concrete_foo_from_base() {
        let provider = TestServiceProvider::new();
        let foo = ServiceGenerator::<Rc<ConcreteFoo>>::generate(&provider);

        assert_eq!(EXPECTED_FOO, foo.foo())
    }
}
