#[cfg(test)]
mod tests {
    use super::super::*;

    const EXPECTED_VALUE: u32 = 987;

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
            context.reset();
        }
    }
    fn shared_compares_provided_helper(context: &ValueContext, count: u32) {
        expect!(context, dyn ValueFoo.shared_foo(EXPECTED_VALUE), count);

        for _ in 0..count {
            let foo: Box<dyn ValueFoo> = context.generate();
            foo.shared_foo(EXPECTED_VALUE);
        }
    }

    #[whey]
    fn exclusive_compares_provided(context: ValueContext) {
        for count in 1..10 {
            exclusive_compares_provided_helper(&context, count);
            context.reset();
        }
    }
    fn exclusive_compares_provided_helper(context: &ValueContext, count: u32) {
        expect!(context, dyn ValueFoo.exclusive_foo(EXPECTED_VALUE), count);

        for _ in 0..count {
            let mut foo: Box<dyn ValueFoo> = context.generate();
            foo.exclusive_foo(EXPECTED_VALUE);
        }
    }
}