#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates(dyn Foo ~ ConcreteFoo)]
    #[generates(dyn Bar ~ FooedBar)]
    struct CompositeProvider {}

    #[test]
    fn injects_foo_into_bar() {
        let provider = CompositeProvider::construct();
        let bar: Box<dyn Bar> = provider.generate();

        assert_eq!(EXPECTED_FOO * EXPECTED_BAR, bar.bar())
    }

    #[service_provider]
    #[generates_singleton(dyn Foo ~ SeededFoo)]
    #[generates(dyn Foo ~ SeededFoo)]
    struct SeededProvider {}
  
    #[test]
    fn generates_defaulted_with_value() {
        let provider = SeededProvider::construct();

        for _ in 0..10 {
            let foo: Box<dyn Foo> = provider.generate();

            assert_eq!(EXPECTED_FOO, foo.foo());
            assert_eq!(EXPECTED_FOO + 1, foo.foo());
            assert_eq!(EXPECTED_FOO + 2, foo.foo());
        }
    }
  
    #[test]
    fn generates_singleton_defaulted_with_value() {
        let provider = SeededProvider::construct();

        for i in 0..10 {
            let foo: Rc<dyn Foo> = provider.generate();

            assert_eq!(EXPECTED_FOO + i * 3, foo.foo());
            assert_eq!(EXPECTED_FOO + i * 3 + 1, foo.foo());
            assert_eq!(EXPECTED_FOO + i * 3 + 2, foo.foo());
        }
    }
}
