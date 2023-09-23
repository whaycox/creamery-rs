#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates(dyn Foo ~ IncrementingFoo)]
    #[generates(IncrementingFoo)]
    struct BaseTransientProvider {}

    #[service_provider]
    #[forwards_singleton(dyn Foo ^ base)]
    #[forwards_singleton(IncrementingFoo ^ base)]
    struct PromotedProvider {
        #[defaulted(BaseTransientProvider::construct())]
        base: BaseTransientProvider,
    }

    #[test]
    fn promotes_singleton_trait_from_base_transient() {
        let mut provider = PromotedProvider::construct();
        
        for i in 0..10 {
            let singleton: Rc<RwLock<Box<dyn Foo>>> = provider.generate();
            let mut foo = singleton.write().unwrap();

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }

    #[test]
    fn promotes_singleton_struct_from_base_transient() {
        let mut provider = PromotedProvider::construct();
        
        for i in 0..10 {
            let singleton: Rc<RwLock<IncrementingFoo>> = provider.generate();
            let mut foo = singleton.write().unwrap();

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }

    #[service_provider]
    #[generates(IncrementingFoo)]
    struct BaseIntermediateProvider {}

    #[service_provider]
    #[forwards_singleton(dyn Foo ^ IncrementingFoo ~ base)]
    struct IntermediateProvider {
        #[defaulted(BaseIntermediateProvider::construct())]
        base: BaseIntermediateProvider,
    }
    
    #[test]
    fn promotes_singleton_struct_from_base_using_intermediate() {
        let mut provider = IntermediateProvider::construct();
        
        for i in 0..10 {
            let singleton: Rc<RwLock<Box<dyn Foo>>> = provider.generate();
            let mut foo = singleton.write().unwrap();

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }
}