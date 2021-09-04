#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates_singleton(IncrementingFoo)]
    struct SingletonStructProvider {}

    #[test]
    fn generates_singleton_foo_struct() {
        let provider = SingletonStructProvider::construct();

        for i in 0..10 {
            let foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&*provider);

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }

    #[service_provider]
    #[generates(IncrementingFoo)]
    struct TransientStructProvider {}

    #[test]
    fn transient_foo_struct_resets() {
        let provider = TransientStructProvider::construct();

        for _ in 0..10 {
            let foo = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&*provider);

            assert_eq!(0, foo.foo());
            assert_eq!(1, foo.foo());
            assert_eq!(2, foo.foo());
        }
    }

    #[service_provider]
    #[generates_singleton(dyn Foo <- IncrementingFoo)]
    struct SingletonTraitProvider {}

    #[test]
    fn generates_singleton_foo_trait() {
        let provider = SingletonTraitProvider::construct();

        for i in 0..10 {
            let foo = ServiceGenerator::<Rc<dyn Foo>>::generate(&*provider);

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }

    #[service_provider]
    #[generates(dyn Foo <- IncrementingFoo)]
    struct TransientTraitProvider {}

    #[test]
    fn transient_foo_trait_resets() {
        let provider = TransientTraitProvider::construct();

        for _ in 0..10 {
            let foo = ServiceGenerator::<Rc<dyn Foo>>::generate(&*provider);

            assert_eq!(0, foo.foo());
            assert_eq!(1, foo.foo());
            assert_eq!(2, foo.foo());
        }
    }
}
