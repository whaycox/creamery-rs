#[cfg(test)]
mod tests {
    use super::super::*;

    const TEST_VALUE: u32 = 1234;

    #[derive(Default)]
    struct TestStruct {
        value: u32,
    }
    // impl TestStruct {
    //     fn test() -> Self {
    //         Self {
    //             value: TEST_VALUE,
    //         }
    //     }
    // }
    #[whey_mock]
    trait DefaultReturnFoo {
        fn basic(&self) -> TestStruct;

        #[mocked_return]
        fn decorated(&self) -> TestStruct;
    }
/*

    #[whey_context]
    //#[mocks(dyn DefaultReturnFoo => test)]
    #[mocks(dyn DefaultReturnFoo)]
    struct DefaultReturnContext {
        #[defaulted(TestStruct::test())]
        test: TestStruct,
    }

    #[whey]
    fn returns_basic_default_with_no_expectations(context: DefaultReturnContext) {
        let foo: Box<dyn DefaultReturnFoo> = context.generate();
        let expected: TestStruct = Default::default();

        assert_eq!(expected, foo.basic());
    }

    #[whey]
    fn returns_decorated_default_with_no_expectations(context: DefaultReturnContext) {
        let foo: Box<dyn DefaultReturnFoo> = context.generate();
        let expected = context.test;

        assert_eq!(expected, foo.decorated());
    }
*/
}