#[cfg(test)]
mod tests {
    use super::super::*;

    const EXPECTED_RETURN: u32 = 654;
    
    #[whey_mock]
    trait ReferenceReturningFoo {
        fn shared_foo(&self) -> &u32;
        fn exclusive_foo(&mut self) -> &mut u32;
    }

    #[whey_context]
    #[mocks(dyn ReferenceReturningFoo)]
    struct ReferenceReturningContext {}

    #[whey]
    fn shared_returns_expected_values(context: ReferenceReturningContext) {
        for count in 1..10 {
            shared_returns_expected_values_helper(&mut context, count);
        }
    }
    fn shared_returns_expected_values_helper(context: &mut ReferenceReturningContext, count: u32) {
        for i in 0..count {
            expect!(context, dyn ReferenceReturningFoo.shared_foo() -> EXPECTED_RETURN - i, 1);
            let foo: Box<dyn ReferenceReturningFoo> = context.generate();
            
            assert_eq!(&(EXPECTED_RETURN - i), foo.shared_foo());
        }
    }

    #[whey]
    fn exclusive_returns_expected_values(context: ReferenceReturningContext) {
        for count in 1..10 {
            exclusive_returns_expected_values_helper(&mut context, count);
        }
    }
    fn exclusive_returns_expected_values_helper(context: &mut ReferenceReturningContext, count: u32) {
        for i in 0..count {
            expect!(context, dyn ReferenceReturningFoo.exclusive_foo() -> EXPECTED_RETURN - i, 1);
            let mut foo: Box<dyn ReferenceReturningFoo> = context.generate();

            assert_eq!(&mut (EXPECTED_RETURN - i), foo.exclusive_foo());
        }
    }
}