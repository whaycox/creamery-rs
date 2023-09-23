#[cfg(test)]
mod tests {
    use super::super::*;

    fn test_delegate(value: u32, reference: &u32) -> u32 { value * reference }

    #[whey_mock]
    trait ValueFoo {
        fn simple(&self) -> u32;
        fn input(&self, value: u32, reference: &u32) -> u32;
    }
    
    #[whey_context(WheyValueFoo)]
    #[mocks(dyn ValueFoo)]
    #[mock_default_return(ValueFoo ~ simple, || EXPECTED_INT)]
    #[mock_default_return(ValueFoo ~ input, test_delegate)]
    struct DefaultReturnValueContext {}
    
    #[whey(DefaultReturnValueContext ~ context)]
    fn simple_returns_default_value() {
        assert_eq!(EXPECTED_INT, context.test_type().simple());
    }

    #[whey(DefaultReturnValueContext ~ context)]
    fn assert_doesnt_reset_default_return() {
        context.assert();

        assert_eq!(EXPECTED_INT, context.test_type().simple());
    }
    
    #[whey(DefaultReturnValueContext ~ context)]
    fn default_generator_can_use_delegate() {
        for i in 1..=10 {
            assert_eq!(EXPECTED_INT * i, context.test_type().input(EXPECTED_INT, &i));
        }
    }

    fn reference_delegate<'a>() -> &'a u32 { &EXPECTED_INT }

    #[whey_mock]
    trait ReferenceFoo<'a> {
        fn closure(&self) -> &'a u32;
        fn delegate(&self) -> &'a u32;
    }
    
    #[whey_context(WheyReferenceFoo<'a>)]
    #[mocks(dyn ReferenceFoo<'a>)]
    #[mock_default_return(ReferenceFoo<'a> ~ closure, || &EXPECTED_INT)]
    #[mock_default_return(ReferenceFoo<'a> ~ delegate, reference_delegate)]
    struct DefaultReturnReferenceContext<'a> {}

    #[whey(DefaultReturnReferenceContext ~ context)]
    fn closure_returns_default_reference() {
        assert_eq!(&EXPECTED_INT, context.test_type().closure());
    }

    #[whey(DefaultReturnReferenceContext ~ context)]
    fn delegate_returns_default_reference() {
        assert_eq!(&EXPECTED_INT, context.test_type().delegate());
    }
}