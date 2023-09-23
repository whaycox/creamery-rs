#[cfg(test)]
mod tests {
    use super::super::*;

    #[whey_mock]
    trait ValueFoo {
        fn simple(&self) -> u32;

        fn input(&self, value: u32, reference: &u32) -> u32;

        fn custom(&self) -> CustomStruct;
    }

    #[whey_context(WheyValueFoo)]
    #[mocks(dyn ValueFoo)]
    struct ReturnGeneratorValueContext {}

    fn simple_delegate() -> u32 { 1 }
    
    #[whey(ReturnGeneratorValueContext ~ context)]
    #[should_panic(expected = "not all stored returns for ValueFoo::simple have been consumed")]
    fn panics_if_returns_arent_consumed() {
        mock_return!(context ~ ValueFoo ~ simple, simple_delegate, 1);
    }
    
    #[whey(ReturnGeneratorValueContext ~ context)]
    fn resets_stored_returns() {
        mock_return!(context ~ ValueFoo ~ simple, simple_delegate, 1);
        
        let core: Rc<RwLock<WheyCoreValueFoo>> = context.generate();
        core.write().unwrap().reset();
    }
    
    #[whey(ReturnGeneratorValueContext ~ context)]
    fn returns_from_many_generators() {
        for i in 1..=10 {
            mock_return!(context ~ ValueFoo ~ simple, move || i, i);
            let test = context.test_type();

            for _ in 0..i {
                assert_eq!(i, test.simple());
            }
        }
    }
    
    #[whey(ReturnGeneratorValueContext ~ context)]
    fn returns_from_many_generators_with_input() {
        for i in 1..=10 {
            mock_return!(context ~ ValueFoo ~ input, move |value, reference| value + reference + i, i);
            let test = context.test_type();

            for j in 0..i {
                assert_eq!(EXPECTED_INT + &j + i, test.input(EXPECTED_INT, &j));
            }
        }
    }
}