#[cfg(test)]
mod tests {
    use super::super::*;

    #[whey_mock]
    trait Foo {
        fn shared_foo(&self);
        fn exclusive_foo(&mut self);
    }

    #[whey_context]
    #[mocks(dyn Foo)]
    struct TransientContext {}

    #[whey]
    fn verifies_shared_counts(context: TransientContext) {
        for count in 1..10 {
            verifies_shared_counts_helper(&context, count);
            context.reset();
        }
        
    }
    fn verifies_shared_counts_helper(context: &TransientContext, count: u32) {
        for _ in 0..count {
            let foo: Box<dyn Foo> = context.generate();
            foo.shared_foo();
        }

        context.mocked().assert_shared_foo(count);
    }
/*
    #[test]
    fn verifies_exclusive_counts() {
        for count in 1..10 {
            verifies_exclusive_counts_helper(count);
        }
        
    }
    fn verifies_exclusive_counts_helper(count: u32) {
        let mut foo = WheyFoo::construct();

        for _ in 0..count {
            foo.exclusive_foo();
        }

        foo.assert_exclusive_foo(count);
    }*/
}