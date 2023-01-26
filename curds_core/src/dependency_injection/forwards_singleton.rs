#[cfg(test)]
mod tests {
    use super::super::*;

    #[service_provider]
    #[generates_singleton(dyn Foo ~ IncrementingFoo)]
    #[generates_singleton(IncrementingFoo)]
    #[generates(UnpromotedProvider)]
    #[generates(IntermediateUnpromotedProvider)]
    #[clones_self]
    #[derive(Clone)]
    struct BaseSingletonProvider {}

    #[service_provider]
    #[forwards_singleton(dyn Foo ~ base)]
    #[forwards_singleton(IncrementingFoo ~ base)]
    struct UnpromotedProvider {
        base: BaseSingletonProvider,
    }
    
    #[test]
    fn forwards_trait_to_base_singleton() {
        let base_provider: BaseSingletonProvider = BaseSingletonProvider::construct();
        let base_foo: Rc<dyn Foo> = base_provider.generate();
        let provider: UnpromotedProvider = base_provider.generate();

        for i in 0..10 {
            let foo: Rc<dyn Foo> = provider.generate();

            assert_eq!(i * 2, base_foo.foo());
            assert_eq!(i * 2 + 1, foo.foo());
        }
    }

    #[test]
    fn forwards_struct_to_base_singleton() {
        let base_provider: BaseSingletonProvider = BaseSingletonProvider::construct();
        let base_foo: Rc<IncrementingFoo> = base_provider.generate();
        let provider: UnpromotedProvider = base_provider.generate();

        for i in 0..10 {
            let foo: Rc<IncrementingFoo> = provider.generate();

            assert_eq!(i * 2, base_foo.foo());
            assert_eq!(i * 2 + 1, foo.foo());
        }
    }

    #[service_provider]
    #[forwards_singleton(dyn Foo ~ IncrementingFoo ~ base)]
    struct IntermediateUnpromotedProvider {
        base: BaseSingletonProvider,
    }

    #[test]
    fn produces_trait_but_forwards_for_struct() {
        let base_provider: BaseSingletonProvider = BaseSingletonProvider::construct();
        let base_foo: Rc<IncrementingFoo> = base_provider.generate();
        let provider: IntermediateUnpromotedProvider = base_provider.generate();

        for i in 0..10 {
            let foo: Rc<dyn Foo> = provider.generate();

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, base_foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }
    
    #[service_provider]
    #[generates(IncrementingFoo)]
    #[generates(dyn Foo ~ IncrementingFoo)]
    #[generates(PromotedProvider)]
    #[generates(IntermediatePromotedProvider)]
    #[clones_self]
    #[derive(Clone)]
    struct BaseTransientProvider {}

    #[service_provider]
    #[forwards_singleton(dyn Foo ^ base)]
    #[forwards_singleton(IncrementingFoo ^ base)]
    struct PromotedProvider {
        base: BaseTransientProvider,
    }

    #[test]
    fn promotes_forwarded_box_for_trait() {
        let base_provider: BaseTransientProvider = BaseTransientProvider::construct();
        let provider: PromotedProvider = base_provider.generate();

        for i in 0..10 {
            let foo: Rc<dyn Foo> = provider.generate();

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }

    #[test]
    fn promotes_forwarded_struct_to_rc() {
        let base_provider: BaseTransientProvider = BaseTransientProvider::construct();
        let provider: PromotedProvider = base_provider.generate();

        for i in 0..10 {
            let foo: Rc<IncrementingFoo> = provider.generate();

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }

    #[service_provider]
    #[forwards_singleton(dyn Foo ^ IncrementingFoo ~ base)]
    struct IntermediatePromotedProvider {
        base: BaseTransientProvider,
    }

    #[test]
    fn promotes_intermediate_struct_to_trait() {
        let base_provider: BaseTransientProvider = BaseTransientProvider::construct();
        let provider: IntermediatePromotedProvider = base_provider.generate();

        for i in 0..10 {
            let foo: Rc<dyn Foo> = provider.generate();

            assert_eq!(i * 3, foo.foo());
            assert_eq!(i * 3 + 1, foo.foo());
            assert_eq!(i * 3 + 2, foo.foo());
        }
    }
}
