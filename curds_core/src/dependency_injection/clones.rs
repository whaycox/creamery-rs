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
        let base_provider = ClonedSelfProvider::construct();

        for i in 0..10 {
            {
                let base_singleton: Singleton<IncrementingFoo> = base_provider.generate();
                let mut base_foo = base_singleton.write().unwrap();
                assert_eq!(i * 3, base_foo.foo());
            }
            {
                let cloned_provider: ClonedSelfProvider = base_provider.generate();
                let cloned_singleton: Singleton<IncrementingFoo> = cloned_provider.generate();
                let mut cloned_foo = cloned_singleton.write().unwrap();
                assert_eq!(i * 3 + 1, cloned_foo.foo());
                assert_eq!(i * 3 + 2, cloned_foo.foo());
            }
        }
    }

    #[service_provider]
    #[clones(base)]
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
            let provider = ClonedDependencyProvider::construct(base_dependency);
                        
            for _ in 0..i {
                let mut clone: IncrementingFoo = provider.generate();

                assert_eq!(i, clone.foo());
                assert_eq!(i + 1, clone.foo());
            }
        }
    }
}
