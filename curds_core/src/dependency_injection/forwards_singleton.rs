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
        let mut base_provider: BaseSingletonProvider = BaseSingletonProvider::construct();
        {
            let base_foo: &mut Box<dyn Foo> = base_provider.lend_mut();
            for _ in 0..count {
                base_foo.foo();
            }
        }
        let mut provider = UnpromotedProvider::construct(base_provider);

        for i in 0..10 {
            let foo: &mut Box<dyn Foo> = provider.lend_mut();

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
        let mut base_provider: BaseSingletonProvider = BaseSingletonProvider::construct();
        {
            let base_foo: &mut IncrementingFoo = base_provider.lend_mut();
            for _ in 0..count {
                base_foo.foo();
            }
        }
        let mut provider = UnpromotedProvider::construct(base_provider);

        for i in 0..10 {
            let foo: &mut IncrementingFoo = provider.lend_mut();

            assert_eq!(count + (i * 3), foo.foo());
            assert_eq!(count + (i * 3) + 1, foo.foo());
            assert_eq!(count + (i * 3) + 2, foo.foo());
        }
    }
}
