#[cfg(test)]
mod tests {
    use super::super::*;

    #[injected]
    struct ConcreteFoo {}
    impl Foo for ConcreteFoo {
        fn foo(&self) -> u32 { EXPECTED_FOO }
    }

    #[service_provider]
    #[maps(Foo <- ConcreteFoo)]
    struct FooProvider {}

    #[service_provider]
    #[generates(BarProvider)]
    #[clones(FooProvider <- provider)]
    struct TestServiceProvider {
        provider: Rc<FooProvider>,
    }
    impl TestServiceProvider {
        pub fn new() -> Self {
            Self::construct(Rc::new(FooProvider::construct()))
        }
    }

    #[service_provider]
    #[forwards(Foo <- provider)]
    struct BarProvider {
        provider: Rc<FooProvider>,
    }

    #[test]
    fn clones_foo_provider() {
        let provider = TestServiceProvider::new();
        let bar_provider = ServiceGenerator::<Rc<BarProvider>>::generate(&provider);
        let foo= ServiceGenerator::<Rc<dyn Foo>>::generate(&*bar_provider);

        assert_eq!(EXPECTED_FOO, foo.foo())
    }
}
