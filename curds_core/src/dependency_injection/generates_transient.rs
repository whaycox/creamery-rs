#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates(ConcreteFoo)]
    #[maps(Bar <- ConcreteBar)]
    pub struct TestServiceProvider {}

    #[injected]
    struct ConcreteFoo {}
    impl Foo for ConcreteFoo {
        fn foo(&self) -> u32 { EXPECTED_FOO }
    }

    #[injected]
    struct ConcreteBar {
        foo: Rc<ConcreteFoo>,
    }
    impl Bar for ConcreteBar {
        fn bar(&self) -> u32 { EXPECTED_BAR * self.foo.foo() }
    }

    #[test]
    fn generates_concrete_foo() {
        let provider = TestServiceProvider::construct();
        let bar = ServiceGenerator::<Rc<dyn Bar>>::generate(&provider);

        assert_eq!(EXPECTED_FOO * EXPECTED_BAR, bar.bar())
    }
}
