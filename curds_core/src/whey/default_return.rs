#[cfg(test)]
mod tests {
    use super::super::*;
    
    const EXPECTED_VALUE: u32 = 123;

    #[whey_mock]
    trait ValueFoo {
        fn simple(&self) -> u32;

        #[mock_default_return(|| EXPECTED_VALUE)]
        fn decorated(&self) -> u32;

        fn input(&self, value: u32, reference: &u32) -> u32;
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
    
    #[whey(DefaultReturnContext ~ context)]
    fn decorated_returns_default_value() {
        assert_eq!(EXPECTED_VALUE, context.test_type().decorated());
    }
    
    #[whey(DefaultReturnContext ~ context)]
    fn default_generator_uses_inputs() {
        mock_default_return!(context ~ ValueFoo ~ input, |value, reference| value + reference);

        for i in 1..=10 {
            assert_eq!(EXPECTED_VALUE + i, context.test_type().input(EXPECTED_VALUE, &i));
        }
    }
}