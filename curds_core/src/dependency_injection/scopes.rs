#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates_singleton(IncrementingFoo)]
    #[scopes_self]
    struct ScopedSelfProvider {}

    #[test]
    fn scoped_provider_doesnt_keep_clone_singletons() {
        let mut base_provider = ScopedSelfProvider::construct();

        for i in 0..10 {
            let mut scoped_provider: ScopedSelfProvider = base_provider.generate();
            let base_foo: &mut IncrementingFoo = base_provider.lend_mut();
            let scoped_foo: &mut IncrementingFoo = scoped_provider.lend_mut();

            assert_eq!(i, base_foo.foo());
            assert_eq!(0, scoped_foo.foo());
            assert_eq!(1, scoped_foo.foo());
            assert_eq!(2, scoped_foo.foo());
        }
    }

    #[service_provider]
    #[generates_singleton(IncrementingFoo)]
    struct BaseProvider {}

    // #[service_provider]
    // #[forwards(IncrementingFoo ~ base)]
    // struct ScopedDependencyProvider {
    //     base: BaseProvider,
    // }

    #[test]
    fn todo() {
        todo!()
    }
}
