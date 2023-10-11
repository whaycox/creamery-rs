#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates_singleton(dyn Foo ~ IncrementingFoo)]
    #[generates_singleton(IncrementingFoo)]
    struct BaseSingletonProvider {}

    #[service_provider]
    #[forwards_singleton(dyn Foo ~ base)]
    #[forwards_singleton(IncrementingFoo ~ base)]
    struct UnpromotedProvider {
        base: BaseSingletonProvider,
    }

    #[test]
    fn forwards_trait_to_base_singleton() {
        for count in 0..10 {
            forwards_trait_to_base_singleton_helper(count);
        }
    }
    fn forwards_trait_to_base_singleton_helper(count: u32) {
        let base_provider: BaseSingletonProvider = BaseSingletonProvider::construct();
        {
            let base_singleton: Singleton<Box<dyn Foo>> = base_provider.generate();
            let mut base_foo = base_singleton.write().unwrap();
            for _ in 0..count {
                base_foo.foo();
            }
        }
        let provider = UnpromotedProvider::construct(base_provider);

        for i in 0..10 {
            let singleton: Singleton<Box<dyn Foo>> = provider.generate();
            let mut foo = singleton.write().unwrap();

            assert_eq!(count + (i * 3), foo.foo());
            assert_eq!(count + (i * 3) + 1, foo.foo());
            assert_eq!(count + (i * 3) + 2, foo.foo());
        }
    }

    #[test]
    fn forwards_struct_to_base_singleton() {
        for count in 0..10 {
            forwards_struct_to_base_singleton_helper(count);
        }
    }
    fn forwards_struct_to_base_singleton_helper(count: u32) {
        let base_provider: BaseSingletonProvider = BaseSingletonProvider::construct();
        {
            let base_singleton: Singleton<IncrementingFoo> = base_provider.generate();
            let mut base_foo = base_singleton.write().unwrap();
            for _ in 0..count {
                base_foo.foo();
            }
        }
        let provider = UnpromotedProvider::construct(base_provider);

        for i in 0..10 {
            let singleton: Singleton<IncrementingFoo> = provider.generate();
            let mut foo = singleton.write().unwrap();

            assert_eq!(count + (i * 3), foo.foo());
            assert_eq!(count + (i * 3) + 1, foo.foo());
            assert_eq!(count + (i * 3) + 2, foo.foo());
        }
    }
}
