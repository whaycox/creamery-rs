#[cfg(test)]
mod tests {
    use super::super::*;

    const EXPECTED_RETURN: u32 = 654;

    #[whey_mock]
    trait ValueReturningFoo {
        fn shared_foo(&self) -> u32;
        fn exclusive_foo(&mut self) -> u32;
    }

    #[whey_context]
    #[mocks(dyn ValueReturningFoo)]
    struct ValueReturningContext {}

    #[whey]
    fn shared_returns_expected_values(context: ValueReturningContext) {
        for count in 1..10 {
            shared_returns_expected_values_helper(&context, count);
        }
    }
    fn shared_returns_expected_values_helper(context: &ValueReturningContext, count: u32) {
        for i in 0..count {
            expect!(context, dyn ValueReturningFoo.shared_foo() -> EXPECTED_RETURN - i, 1);
        }

        for i in 0..count {
            let foo: Box<dyn ValueReturningFoo> = context.generate();
            assert_eq!(EXPECTED_RETURN - i, foo.shared_foo());
        }

        context.mocked().assert();
    }

    #[whey]
    fn exclusive_returns_expected_values(context: ValueReturningContext) {
        for count in 1..10 {
            exclusive_returns_expected_values_helper(&context, count);
        }
    }
    fn exclusive_returns_expected_values_helper(context: &ValueReturningContext, count: u32) {
        for i in 0..count {
            expect!(context, dyn ValueReturningFoo.exclusive_foo() -> EXPECTED_RETURN - i, 1);
        }

        for i in 0..count {
            let mut foo: Box<dyn ValueReturningFoo> = context.generate();
            assert_eq!(EXPECTED_RETURN - i, foo.exclusive_foo());
        }

        context.mocked().assert();
    }
}