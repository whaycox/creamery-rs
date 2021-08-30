#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[maps(Foo <- ConcreteFoo)]
    pub struct TestServiceProvider {}

    #[injected]
    struct ConcreteFoo {}
    impl Foo for ConcreteFoo {
        fn foo(&self) -> u32 { EXPECTED_FOO }
    }

    #[test]
    fn maps_foo() {
        let provider = TestServiceProvider::root_inject();
        let foo = ServiceGenerator::<Rc<dyn Foo>>::generate(&provider);

        assert_eq!(EXPECTED_FOO, foo.foo())
    }
}
