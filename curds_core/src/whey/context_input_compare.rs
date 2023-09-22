#[cfg(test)]
mod tests {
    use super::super::*;
    
    const EXPECTED_ITERATIONS: u32 = 10;
    const EXPECTED_INT: u32 = 123;
    const EXPECTED_LONG: u64 = 9876543210;

    #[whey_mock]
    trait VoidFoo {
        fn value(&mut self, input: u32);
        fn reference(&mut self, one: &u32, two: &u64);
    }

    #[whey_context(WheyVoidFoo)]
    #[mocks(dyn VoidFoo)]
    #[mock_input(VoidFoo ~ value, value_comparison, EXPECTED_ITERATIONS)]
    #[mock_input(VoidFoo ~ reference, move |one, two| *one == EXPECTED_INT && *two == EXPECTED_LONG, EXPECTED_ITERATIONS)]
    struct VoidFooContext {}

    fn value_comparison(input: u32) -> bool { input == EXPECTED_INT }

    #[whey(VoidFooContext ~ context)]
    #[should_panic(expected = "not all stored input comparisons for VoidFoo::value have been consumed")]
    fn panics_if_inputs_arent_consumed() {}
    
    #[whey(VoidFooContext ~ context)]
    fn resets_stored_returns() {       
        let core: Rc<RwLock<WheyCoreVoidFoo>> = context.generate();

        core.write().unwrap().reset();
    }

    #[whey(VoidFooContext ~ context)]
    #[should_panic(expected = "the expected inputs for VoidFoo::value were not supplied")]
    fn panics_if_inputs_arent_expected() {
        let mut test = context.test_type();

        test.value(EXPECTED_INT + 1);
    }
    
    #[whey(VoidFooContext ~ context)]
    fn compares_against_many_comparisons() {
        let mut test = context.test_type();

        for _ in 1..=EXPECTED_ITERATIONS {
            test.value(EXPECTED_INT);
            test.reference(&(EXPECTED_INT), &EXPECTED_LONG);
        }
    }
}