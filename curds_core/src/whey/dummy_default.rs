#[cfg(test)]
mod tests {
    use super::super::*;
    
    const EXPECTED_VALUE: u32 = 123;

    #[derive(Default, Clone)]
    struct DerivedStruct {
        pub value: u32,
    }

    #[derive(Clone)]
    struct SomeStruct {
        pub value: u32,
    }
    impl DummyDefault for SomeStruct {
        fn dummy() -> Self { 
            SomeStruct { 
                value: EXPECTED_VALUE 
            } 
        }
    }

    #[whey_mock]
    trait Foo {
        fn returning_foo(&self) -> u32;
        fn returning_exclusive_foo(&mut self) -> u32;
    }


    #[whey_mock]
    trait DerivedFoo {
        fn returning_foo(&self) -> DerivedStruct;
        fn returning_exclusive_foo(&mut self) -> DerivedStruct;
    }

    #[whey_mock]
    trait StructFoo {
        fn returning_foo(&self) -> SomeStruct;
        fn returning_exclusive_foo(&mut self) -> SomeStruct;
    }

    #[whey_context]
    #[mocks(dyn Foo)]
    #[mocks(dyn DerivedFoo)]
    #[mocks(dyn StructFoo)]
    struct DefaultContext {}

    #[whey]
    fn shared_returns_default(context: DefaultContext) {
        let foo: Box<dyn Foo> = context.generate();
        assert_eq!(0, foo.returning_foo());
    }

    #[whey]
    fn exclusive_returns_default(context: DefaultContext) {
        let mut foo: Box<dyn Foo> = context.generate();
        assert_eq!(0, foo.returning_exclusive_foo());
    }

    #[whey]
    fn shared_returns_derived_default(context: DefaultContext) {
        let foo: Box<dyn DerivedFoo> = context.generate();
        assert_eq!(0, foo.returning_foo().value);
    }

    #[whey]
    fn exclusive_returns_derived_default(context: DefaultContext) {
        let mut foo: Box<dyn DerivedFoo> = context.generate();
        assert_eq!(0, foo.returning_exclusive_foo().value);
    }
    
    #[whey]
    fn shared_returns_dummy_default(context: DefaultContext) {
        let foo: Box<dyn StructFoo> = context.generate();
        assert_eq!(EXPECTED_VALUE, foo.returning_foo().value);
    }

    #[whey]
    fn exclusive_returns_dummy_default(context: DefaultContext) {
        let mut foo: Box<dyn StructFoo> = context.generate();
        assert_eq!(EXPECTED_VALUE, foo.returning_exclusive_foo().value);
    }
}