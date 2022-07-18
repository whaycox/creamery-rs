#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates_singleton(IncrementingFoo)]
    #[generates(ClonedStructProvider)]
    #[clones_self]
    struct BaseProvider {}

    #[service_provider]
    #[clones(base)]
    struct ClonedStructProvider {
        base: BaseProvider,
    }

    #[test]
    fn generated_provider_is_cloned_from_base_each_time() {
        let base_provider = BaseProvider::construct();
        let base_foo: Rc<IncrementingFoo> = base_provider.generate();
        let provider: ClonedStructProvider = base_provider.generate();

        for i in 0..10 {
            let cloned_provider: BaseProvider = provider.generate();
            let foo: Rc<IncrementingFoo> = cloned_provider.generate();

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, base_foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }
}
