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
            verifies_shared_counts_helper(&mut context, count);
            context.reset();
        }        
    }
    fn verifies_shared_counts_helper(context: &mut TransientContext, count: u32) {
        for _ in 0..count {
            let foo: Box<dyn Foo> = context.generate();
            foo.shared_foo();
        }

        context.mocked().assert_shared_foo(count);
    }

    #[whey]
    fn verifies_exclusive_counts(context: TransientContext) {
        for count in 1..10 {
            verifies_exclusive_counts_helper(&mut context, count);
            context.reset();
        }        
    }
    fn verifies_exclusive_counts_helper(context: &mut TransientContext, count: u32) {
        for _ in 0..count {
            let mut foo: Box<dyn Foo> = context.generate();
            foo.exclusive_foo();
        }

        context.mocked().assert_exclusive_foo(count);
    }

    #[whey_context]
    #[mocks_singleton(dyn Foo)]
    struct SingletonContext {}

    #[whey]
    fn verifies_singleton_shared_counts(context: SingletonContext) {
        for count in 1..10 {
            verifies_singleton_shared_counts_helper(&mut context, count);
            context.reset();
        }        
    }
    fn verifies_singleton_shared_counts_helper(context: &mut SingletonContext, count: u32) {
        for _ in 0..count {
            let singleton: Rc<RwLock<Box<dyn Foo>>> = context.generate();
            let foo = singleton.read().unwrap();
            foo.shared_foo();
        }

        context.mocked().assert_shared_foo(count);
    }

    #[whey]
    fn verifies_singleton_exclusive_counts(context: SingletonContext) {
        for count in 1..10 {
            verifies_singleton_exclusive_counts_helper(&mut context, count);
            context.reset();
        }        
    }
    fn verifies_singleton_exclusive_counts_helper(context: &mut SingletonContext, count: u32) {
        for _ in 0..count {
            let singleton: Rc<RwLock<Box<dyn Foo>>> = context.generate();
            let mut foo = singleton.write().unwrap();
            foo.exclusive_foo();
        }

        context.mocked().assert_exclusive_foo(count);
    }
}