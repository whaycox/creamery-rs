#[cfg(test)]
mod tests {
    use super::super::*;

    const EXPECTED_VALUE: u32 = 987;
    const EXPECTED_LONG: u64 = 9876543210;

    #[whey_mock]
    trait ReferenceFoo {
        fn shared_foo(&self, value: &u32);
        fn exclusive_foo(&mut self, value: &u32);
    }
 
    #[whey_context]
    #[mocks(dyn ReferenceFoo)]
    struct ReferenceContext {}

    #[whey]
    fn shared_compares_provided(context: ReferenceContext) {
        for count in 1..10 {
            shared_compares_provided_helper(&mut context, count);
        }
    }
    fn shared_compares_provided_helper(context: &mut ReferenceContext, count: u32) {
        expect!(context, dyn ReferenceFoo.shared_foo(EXPECTED_VALUE), count);
        let foo: Box<dyn ReferenceFoo> = context.generate();

        for _ in 0..count {
            foo.shared_foo(&EXPECTED_VALUE);
        }
    }

    #[whey]
    fn exclusive_compares_provided(context: ReferenceContext) {
        for count in 1..10 {
            exclusive_compares_provided_helper(&mut context, count);
        }
    }
    fn exclusive_compares_provided_helper(context: &mut ReferenceContext, count: u32) {
        expect!(context, dyn ReferenceFoo.exclusive_foo(EXPECTED_VALUE), count);
        let mut foo: Box<dyn ReferenceFoo> = context.generate();

        for _ in 0..count {
            foo.exclusive_foo(&EXPECTED_VALUE);
        }
    }

    #[whey_mock]
    trait MultiInputReferenceFoo {
        fn shared_foo(&self, one: &u32, two: &mut u64);
        fn exclusive_foo(&mut self, one: &u32, two: &mut u64);
    }
    
    #[whey_context]
    #[mocks(dyn MultiInputReferenceFoo)]
    struct MultiInputReferenceContext {}

    #[whey]
    fn multi_shared_compares_provided(context: MultiInputReferenceContext) {
        for count in 1..10 {
            multi_shared_compares_provided_helper(&mut context, count);
        }
    }
    fn multi_shared_compares_provided_helper(context: &mut MultiInputReferenceContext, count: u32) {
        expect!(context, dyn MultiInputReferenceFoo.shared_foo(EXPECTED_VALUE, EXPECTED_LONG), count);
        let mut test_long = EXPECTED_LONG;
        let foo: Box<dyn MultiInputReferenceFoo> = context.generate();

        for _ in 0..count {
            foo.shared_foo(&EXPECTED_VALUE, &mut test_long);
        }
    }

    #[whey]
    fn multi_exclusive_compares_provided(context: MultiInputReferenceContext) {
        for count in 1..10 {
            multi_exclusive_compares_provided_helper(&mut context, count);
        }
    }
    fn multi_exclusive_compares_provided_helper(context: &mut MultiInputReferenceContext, count: u32) {
        expect!(context, dyn MultiInputReferenceFoo.exclusive_foo(EXPECTED_VALUE, EXPECTED_LONG), count);
        let mut test_long = EXPECTED_LONG;
        let mut foo: Box<dyn MultiInputReferenceFoo> = context.generate();

        for _ in 0..count {
            foo.exclusive_foo(&EXPECTED_VALUE, &mut test_long);
        }
    }
}