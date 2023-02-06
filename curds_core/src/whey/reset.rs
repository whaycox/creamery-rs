#[cfg(test)]
mod tests {
    use super::super::*;

    const FOO_VALUE: u32 = 234;

    #[whey_mock]
    trait Foo {
        fn foo(&self, value: u32);
    }

    #[whey_context]
    #[mocks(dyn Foo)]
    struct ResettingContext {}
    
    #[whey]
    fn resets_call_count(context: ResettingContext) {
        let foo: Box<dyn Foo> = context.generate();
        foo.foo(FOO_VALUE);

        context.reset();

        context.mocked().assert_foo(0);
    }
    
    #[whey]
    fn resets_expectations(context: ResettingContext) {
        expect!(context, dyn Foo.foo(FOO_VALUE));

        context.reset();
    }
}