#[cfg(test)]
mod tests {
    use super::super::*;
    
    #[service_provider]
    #[maps(Foo <- ConcreteFoo)]
    struct FooProvider {}

    #[injected]
    struct ConcreteFoo {}
    impl Foo for ConcreteFoo {
        fn foo(&self) -> u32 { EXPECTED_FOO }   
    }

    #[service_provider]
    #[maps(Bar <- ConcreteBar)]
    #[forwards(Foo <- provider)]
    struct BarProvider {
        provider: Rc<FooProvider>,
    }
    impl BarProvider {
        pub fn new() -> Self {
            Self::construct(Rc::new(FooProvider::construct()))
        }
    }

    #[injected]
    struct ConcreteBar {
        foo: Rc<dyn Foo>,
    }
    impl Bar for ConcreteBar {
        fn bar(&self) -> u32 { EXPECTED_BAR * self.foo.foo() }
    }

    #[test]
    fn forwards_foo_generate_to_provider() {
        let provider = BarProvider::new();
        let bar = ServiceGenerator::<Rc<dyn Bar>>::generate(&provider);

        assert_eq!(EXPECTED_FOO * EXPECTED_BAR, bar.bar())
    }
}
