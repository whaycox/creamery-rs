#[cfg(test)]
mod tests {
    use super::super::*;

    #[whey_mock]
    trait ValueFoo {
        fn simple(&self) -> u32;
        fn input(&self, value: u32, reference: &u32) -> u32;
    }

    #[whey_context(WheyValueFoo)]
    #[mocks(dyn ValueFoo)]
    #[mock_return(ValueFoo ~ simple, simple_delegate, EXPECTED_ITERATIONS)]
    #[mock_return(ValueFoo ~ input, |value, reference| value + reference, EXPECTED_ITERATIONS)]
    struct ReturnGeneratorValueContext {}

    fn simple_delegate() -> u32 { EXPECTED_INT }
    
    #[whey(ReturnGeneratorValueContext ~ context)]
    #[should_panic(expected = "not all stored returns for ValueFoo::simple have been consumed")]
    fn panics_if_returns_arent_consumed() {}
    
    #[whey(ReturnGeneratorValueContext ~ context)]
    fn resets_stored_returns() {
        let core: Rc<RwLock<WheyCoreValueFoo>> = context.generate();

        core.write().unwrap().reset();
    }
    
    #[whey(ReturnGeneratorValueContext ~ context)]
    fn returns_from_many_generators() {
        let test = context.test_type();

        for i in 1..=EXPECTED_ITERATIONS {
            assert_eq!(EXPECTED_INT, test.simple());
            assert_eq!(EXPECTED_INT + i, test.input(EXPECTED_INT, &i));
        }
    }
}