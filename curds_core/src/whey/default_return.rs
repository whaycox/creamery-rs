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
    struct DefaultReturnValueContext {}
    
    #[whey(DefaultReturnValueContext ~ context)]
    fn simple_returns_default_value() {
        mock_default_return!(context ~ ValueFoo ~ simple, || EXPECTED_VALUE);

        assert_eq!(EXPECTED_VALUE, context.test_type().simple());
    }
    
    #[whey(DefaultReturnValueContext ~ context)]
    fn assert_doesnt_reset_default_return() {
        let test_value = 234;
        mock_default_return!(context ~ ValueFoo ~ simple, move || test_value);

        context.assert();

        assert_eq!(test_value, context.test_type().simple());
    }

    #[whey(DefaultReturnValueContext ~ context)]
    #[should_panic(expected = "a return is necessary for ValueFoo::simple but none have been supplied")]
    fn simple_panics_without_default() {
        context
            .test_type()
            .simple();
    }
    
    #[whey(DefaultReturnValueContext ~ context)]
    fn decorated_returns_default_value() {
        assert_eq!(EXPECTED_VALUE, context.test_type().decorated());
    }
    
    #[whey(DefaultReturnValueContext ~ context)]
    fn default_generator_uses_inputs() {
        mock_default_return!(context ~ ValueFoo ~ input, |value, reference| value + reference);

        for i in 1..=10 {
            assert_eq!(EXPECTED_VALUE + i, context.test_type().input(EXPECTED_VALUE, &i));
        }
    }

    // #[whey_mock]
    // trait ReferenceFoo<'a> {
    //     fn simple(&self) -> &'a u32;

    //     #[mock_default_return(|| &EXPECTED_VALUE)]
    //     fn decorated(&self) -> &'a u32;
    // }
    
    // #[whey_context(WheyReferenceFoo<'a>)]
    // #[mocks(dyn ReferenceFoo<'a>)]
    // struct DefaultReturnReferenceContext<'a> {}

    // #[whey(DefaultReturnReferenceContext ~ context)]
    // fn simple_returns_default_reference() {
    //     mock_default_return!(context ~ ReferenceFoo ~ simple, || &EXPECTED_VALUE);

    //     assert_eq!(&EXPECTED_VALUE, context.test_type().simple());
    // }
    
    // #[whey(DefaultReturnReferenceContext ~ context)]
    // fn decorated_returns_default_reference() {
    //     assert_eq!(&EXPECTED_VALUE, context.test_type().decorated());
    // }
}