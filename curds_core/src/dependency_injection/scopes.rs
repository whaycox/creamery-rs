#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates_singleton(IncrementingFoo)]
    #[generates(ScopedStructProvider)]
    #[scopes_self]
    struct BaseProvider {}

    #[service_provider]
    #[scopes(base)]
    struct ScopedStructProvider {
        base: BaseProvider,
    }

    #[test]
    fn scoped_provider_doesnt_keep_parent_singletons() {
        let base_provider = BaseProvider::construct();
        let base_foo: Rc<IncrementingFoo> = base_provider.generate();
        let provider: ScopedStructProvider = base_provider.generate();

        for i in 0..10 {
            let scoped_provider: BaseProvider = provider.generate();
            let foo: Rc<IncrementingFoo> = scoped_provider.generate();

            assert_eq!(i, base_foo.foo());
            assert_eq!(0, foo.foo());
            assert_eq!(1, foo.foo());
            assert_eq!(2, foo.foo());
        }
    }

    #[test]
    fn can_scope_self() {
        let base_provider = BaseProvider::construct();
        let base_foo: Rc<IncrementingFoo> = base_provider.generate();

        for i in 0..10 {
            let scoped_provider: BaseProvider = base_provider.generate();
            let foo: Rc<IncrementingFoo> = scoped_provider.generate();

            assert_eq!(i, base_foo.foo());
            assert_eq!(0, foo.foo());
            assert_eq!(1, foo.foo());
            assert_eq!(2, foo.foo());
        }
    }
}
