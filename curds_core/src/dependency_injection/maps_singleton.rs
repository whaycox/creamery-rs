#[cfg(test)]
mod tests {
    use super::super::*;

    #[injected]
    #[defaults(value)]
    struct IncrementingFoo {
        value: Cell<u32>,
    }
    impl Foo for IncrementingFoo {
        fn foo(&self) -> u32 {
            let value = self.value.get();
            self.value.set(value + 1);
            value
        }
    }

    #[service_provider]
    #[maps(Foo <- IncrementingFoo)]
    struct TransientServiceProvider {}

    #[service_provider]
    #[maps_singleton(Foo <- IncrementingFoo)]
    struct SingletonServiceProvider {}

    #[test]
    fn transient_foo_resets() {
        let transient = TransientServiceProvider::construct();

        for _ in 0..10 {
            let foo = ServiceGenerator::<Rc<dyn Foo>>::generate(&transient);

            assert_eq!(0, foo.foo());
            assert_eq!(1, foo.foo());
            assert_eq!(2, foo.foo());
        }
    }

    #[test]
    fn singleton_foo_remembers() {
        let singleton = SingletonServiceProvider::construct();

        for i in 0..10 {
            let foo = ServiceGenerator::<Rc<dyn Foo>>::generate(&singleton);

            assert_eq!(3 * i, foo.foo());
            assert_eq!(3 * i + 1, foo.foo());
            assert_eq!(3 * i + 2, foo.foo());
        }
    }

    #[service_provider]
    #[generates_singleton(IncrementingFoo)]
    #[maps_singleton(Foo <- IncrementingFoo)]
    struct ComboServiceProvider {}

    #[test]
    fn generates_and_maps_same_singleton() {
        let combo = ComboServiceProvider::construct();

        for i in 0..10 {
            let generated = ServiceGenerator::<Rc<IncrementingFoo>>::generate(&combo);
            let mapped = ServiceGenerator::<Rc<dyn Foo>>::generate(&combo);

            assert_eq!(3 * i, mapped.foo());
            assert_eq!(3 * i + 1, generated.foo());
            assert_eq!(3 * i + 2, mapped.foo());
        }
    }
}
