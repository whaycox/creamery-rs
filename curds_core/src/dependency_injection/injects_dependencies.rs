#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates(dyn Foo <- ConcreteFoo)]
    #[generates(dyn Bar <- FooedBar)]
    struct CompositeProvider {}

    #[test]
    fn injects_foo_into_bar() {
        let provider = CompositeProvider::construct();
        let bar = ServiceGenerator::<Rc<dyn Bar>>::generate(&*provider);

        assert_eq!(EXPECTED_FOO * EXPECTED_BAR, bar.bar())
    }
}
