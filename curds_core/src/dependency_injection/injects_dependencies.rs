#[cfg(test)]
mod tests {
    use super::super::*;

    #[injected]
    struct ConcreteFoo {}
    impl Foo for ConcreteFoo {
        fn foo(&self) -> u32 { EXPECTED_FOO }
    }

    #[injected]
    struct ConcreteBar {
        foo: Rc<dyn Foo>,
    }
    impl Bar for ConcreteBar {
        fn bar(&self) -> u32 { EXPECTED_BAR * self.foo.foo() }
    }

    #[service_provider]
    #[maps(Foo <- ConcreteFoo)]
    #[maps(Bar <- ConcreteBar)]
    pub struct TestServiceProvider {}

    #[test]
    fn injects_foo_into_bar() {
        let provider = TestServiceProvider::root_inject();
        let bar = ServiceGenerator::<Rc<dyn Bar>>::generate(&provider);

        assert_eq!(EXPECTED_FOO * EXPECTED_BAR, bar.bar())
    }
}
