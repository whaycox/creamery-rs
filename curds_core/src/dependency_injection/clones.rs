#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates_singleton(IncrementingFoo)]
    #[clones_self]
    #[derive(Clone)]
    struct ClonedSelfProvider {}

    #[test]
    fn generated_provider_is_cloned_from_base_each_time() {
        let mut base_provider = ClonedSelfProvider::construct();

        for i in 0..10 {
            let mut cloned_provider: ClonedSelfProvider = base_provider.generate();
            let base_foo: &mut IncrementingFoo = base_provider.lend_mut();
            let cloned_foo: &mut IncrementingFoo = cloned_provider.lend_mut();

            assert_eq!(i, base_foo.foo());
            assert_eq!(i, cloned_foo.foo());
            assert_eq!(i + 1, cloned_foo.foo());
            assert_eq!(i + 2, cloned_foo.foo());
        }
    }

    #[service_provider]
    #[clones(base)]
    #[derive(Clone)]
    struct ClonedDependencyProvider {
        base: IncrementingFoo,
    }

    #[test]
    fn clones_service_from_dependency() {
        for i in 0..10 {
            let mut base_dependency = IncrementingFoo::construct();
            for _ in 0..i {
                base_dependency.foo();
            }
            let mut provider = ClonedDependencyProvider::construct(base_dependency);
                        
            for _ in 0..i {
                let mut clone: IncrementingFoo = provider.generate();

                assert_eq!(i, clone.foo());
                assert_eq!(i + 1, clone.foo());
            }
        }
    }
}
