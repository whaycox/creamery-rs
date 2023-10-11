#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates_singleton(dyn Foo ~ IncrementingFoo)]
    #[generates_singleton(IncrementingFoo)]
    #[generates(dyn Foo ~ IncrementingFoo)]
    #[generates(IncrementingFoo)]
    struct SingletonProvider {}

    #[test]
    fn generates_singleton_foo_struct() {
        let provider = SingletonProvider::construct();

        for i in 0..10 {
            let singleton: Singleton<IncrementingFoo> = provider.generate();
            let mut foo = singleton.write().unwrap();

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }

    #[test]
    fn transient_foo_struct_resets() {
        let provider = SingletonProvider::construct();

        for _ in 0..10 {
            let mut foo: IncrementingFoo = provider.generate();

            assert_eq!(0, foo.foo());
            assert_eq!(1, foo.foo());
            assert_eq!(2, foo.foo());
        }
    }

    #[test]
    fn generates_singleton_foo_trait() {
        let provider = SingletonProvider::construct();

        for i in 0..10 {
            let singleton: Singleton<Box<dyn Foo>> = provider.generate();
            let mut foo = singleton.write().unwrap();

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }

    #[test]
    fn transient_foo_trait_resets() {
        let provider = SingletonProvider::construct();

        for _ in 0..10 {
            let mut foo: Box<dyn Foo> = provider.generate();

            assert_eq!(0, foo.foo());
            assert_eq!(1, foo.foo());
            assert_eq!(2, foo.foo());
        }
    }

    #[test]
    fn trait_and_struct_are_not_same() {
        let provider = SingletonProvider::construct();

        for i in 0..10 {
            let trait_singleton: Singleton<Box<dyn Foo>> = provider.generate();
            let struct_singleton: Singleton<IncrementingFoo> = provider.generate();
            let mut trait_foo = trait_singleton.write().unwrap();
            let mut struct_foo = struct_singleton.write().unwrap();

            assert_eq!(i * 3, trait_foo.foo());
            assert_eq!(i * 3 + 1, trait_foo.foo());
            assert_eq!(i * 3 + 2, trait_foo.foo());
            assert_eq!(i * 3, struct_foo.foo());
            assert_eq!(i * 3 + 1, struct_foo.foo());
            assert_eq!(i * 3 + 2, struct_foo.foo());
        }
    }
}
