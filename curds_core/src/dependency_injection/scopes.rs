#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates_singleton(IncrementingFoo)]
    #[scopes_self]
    #[derive(Scoped)]
    struct ScopedSelfProvider {}

    #[test]
    fn scoped_provider_doesnt_keep_singletons() {
        let base_provider = ScopedSelfProvider::construct();

        for i in 0..10 {
            let base_singleton: Rc<RwLock<IncrementingFoo>> = base_provider.generate();
            let mut base_foo = base_singleton.write().unwrap();
            let scoped_provider: ScopedSelfProvider = base_provider.generate();
            let scoped_singleton: Rc<RwLock<IncrementingFoo>> = scoped_provider.generate();
            let mut scoped_foo = scoped_singleton.write().unwrap();

            assert_eq!(i, base_foo.foo());
            assert_eq!(0, scoped_foo.foo());
            assert_eq!(1, scoped_foo.foo());
            assert_eq!(2, scoped_foo.foo());
        }
    }

    #[service_provider]
    #[generates_singleton(IncrementingFoo)]
    #[derive(Scoped)]
    struct BaseProvider {}

    #[service_provider]
    #[forwards_singleton(IncrementingFoo ~ base)]
    #[scopes(base)]
    struct ScopedDependencyProvider {
        #[defaulted(BaseProvider::construct())]
        base: BaseProvider,
    }

    #[test]
    fn scoped_base_doesnt_keep_singletons() {
        let provider = ScopedDependencyProvider::construct();

        for i in 0..10 {
            let scoped_base: BaseProvider = provider.generate();
            let singleton: Rc<RwLock<IncrementingFoo>> = provider.generate();
            let mut foo = singleton.write().unwrap();
            let scoped_singleton: Rc<RwLock<IncrementingFoo>> = scoped_base.generate();
            let mut scoped_foo = scoped_singleton.write().unwrap();

            assert_eq!(i, foo.foo());
            assert_eq!(0, scoped_foo.foo());
            assert_eq!(1, scoped_foo.foo());
            assert_eq!(2, scoped_foo.foo());
        }
    }
}
