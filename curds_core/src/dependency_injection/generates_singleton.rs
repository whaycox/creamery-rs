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
            let foo: Rc<IncrementingFoo> = provider.generate();

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }

    #[test]
    fn transient_foo_struct_resets() {
        let provider = SingletonProvider::construct();

        for _ in 0..10 {
            let foo: IncrementingFoo = provider.generate();

            assert_eq!(0, foo.foo());
            assert_eq!(1, foo.foo());
            assert_eq!(2, foo.foo());
        }
    }

    #[test]
    fn generates_singleton_foo_trait() {
        let provider = SingletonProvider::construct();

        for i in 0..10 {
            let foo: Rc<dyn Foo> = provider.generate();

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }

    #[test]
    fn transient_foo_trait_resets() {
        let provider = SingletonProvider::construct();

        for _ in 0..10 {
            let foo: Box<dyn Foo> = provider.generate();

            assert_eq!(0, foo.foo());
            assert_eq!(1, foo.foo());
            assert_eq!(2, foo.foo());
        }
    }

    #[test]
    fn trait_and_struct_are_same() {
        let provider = SingletonProvider::construct();

        for i in 0..10 {
            let foo_trait: Rc<dyn Foo> = provider.generate();
            let foo: Rc<IncrementingFoo> = provider.generate();

            assert_eq!(i * 3, foo_trait.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo_trait.foo());
        }
    }
}
