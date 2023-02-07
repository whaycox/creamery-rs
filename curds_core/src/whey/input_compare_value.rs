#[cfg(test)]
mod tests {
    use super::super::*;

    const EXPECTED_VALUE: u32 = 987;
    const EXPECTED_LONG: u64 = 9876543210;

    #[whey_mock]
    trait ValueFoo {
        fn shared_foo(&self, value: u32);
        fn exclusive_foo(&mut self, value: u32);
    }

    #[whey_context]
    #[mocks(dyn ValueFoo)]
    struct ValueContext {}

    #[whey]
    fn shared_compares_provided(context: ValueContext) {
        for count in 1..10 {
            shared_compares_provided_helper(&context, count);
        }
    }
    fn shared_compares_provided_helper(context: &ValueContext, count: u32) {
        expect!(context, dyn ValueFoo.shared_foo(EXPECTED_VALUE), count);

        for _ in 0..count {
            let foo: Box<dyn ValueFoo> = context.generate();
            foo.shared_foo(EXPECTED_VALUE);
        }

        context.mocked().assert();
    }

    #[whey]
    fn exclusive_compares_provided(context: ValueContext) {
        for count in 1..10 {
            exclusive_compares_provided_helper(&context, count);
        }
    }
    fn exclusive_compares_provided_helper(context: &ValueContext, count: u32) {
        expect!(context, dyn ValueFoo.exclusive_foo(EXPECTED_VALUE), count);

        for _ in 0..count {
            let mut foo: Box<dyn ValueFoo> = context.generate();
            foo.exclusive_foo(EXPECTED_VALUE);
        }

        context.mocked().assert();
    }

    #[whey_mock]
    trait MultiInputValueFoo {
        fn shared_foo(&self, one: u32, two: u64);
        fn exclusive_foo(&mut self, one: u32, two: u64);
    }
    
    #[whey_context]
    #[mocks(dyn MultiInputValueFoo)]
    struct MultiInputValueContext {}

    #[whey]
    fn multi_shared_compares_provided(context: MultiInputValueContext) {
        for count in 1..10 {
            multi_shared_compares_provided_helper(&context, count);
        }
    }
    fn multi_shared_compares_provided_helper(context: &MultiInputValueContext, count: u32) {
        expect!(context, dyn MultiInputValueFoo.shared_foo(EXPECTED_VALUE, EXPECTED_LONG), count);

        for _ in 0..count {
            let foo: Box<dyn MultiInputValueFoo> = context.generate();
            foo.shared_foo(EXPECTED_VALUE, EXPECTED_LONG);
        }
        
        context.mocked().assert();
    }

    #[whey]
    fn multi_exclusive_compares_provided(context: MultiInputValueContext) {
        for count in 1..10 {
            multi_exclusive_compares_provided_helper(&context, count);
        }
    }
    fn multi_exclusive_compares_provided_helper(context: &MultiInputValueContext, count: u32) {
        expect!(context, dyn MultiInputValueFoo.exclusive_foo(EXPECTED_VALUE, EXPECTED_LONG), count);

        for _ in 0..count {
            let mut foo: Box<dyn MultiInputValueFoo> = context.generate();
            foo.exclusive_foo(EXPECTED_VALUE, EXPECTED_LONG);
        }
        
        context.mocked().assert();
    }
}