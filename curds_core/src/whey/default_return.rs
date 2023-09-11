#[cfg(test)]
mod tests {
    use super::super::*;
    
    const EXPECTED_VALUE: u32 = 123;

    #[whey_mock]
    trait ValueFoo {
        fn simple(&self) -> u32;
    }
    
    #[whey_context(WheyValueFoo)]
    #[mocks(dyn ValueFoo)]
    struct DefaultReturnContext {}
    
    #[whey(DefaultReturnContext ~ context)]
    fn simple_returns_default_value() {
        mock_default_return!(context ~ ValueFoo ~ simple, || EXPECTED_VALUE);

        assert_eq!(EXPECTED_VALUE, context.test_type().simple());
    }
    
    #[whey(DefaultReturnContext ~ context)]
    fn assert_doesnt_reset_default_return() {
        let test_value = 234;
        mock_default_return!(context ~ ValueFoo ~ simple, move || test_value);

        context.assert();

        assert_eq!(test_value, context.test_type().simple());
    }

    #[whey(DefaultReturnContext ~ context)]
    #[should_panic(expected = "a return is necessary but none have been supplied")]
    fn simple_panics_without_default() {
        context
            .test_type()
            .simple();
    }
}