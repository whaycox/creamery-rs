#[cfg(test)]
mod tests {
    use super::super::*;
    
    const EXPECTED_INT: u32 = 123;
    const EXPECTED_LONG: u64 = 9876543210;

    #[whey_mock]
    trait VoidFoo {
        fn value(&mut self, input: u32);
        fn reference(&mut self, one: &u32, two: &u64);
    }

    #[whey_context(WheyVoidFoo)]
    #[mocks(dyn VoidFoo)]
    struct VoidFooContext {}

    fn value_comparison(input: u32) -> bool { input == EXPECTED_INT }

    #[whey(VoidFooContext ~ context)]
    #[should_panic(expected = "not all stored input comparisons for VoidFoo::value have been consumed")]
    fn panics_if_inputs_arent_consumed() {
        mock_input!(context ~ VoidFoo ~ value, value_comparison, 1);
    }
    
    #[whey(VoidFooContext ~ context)]
    fn resets_stored_returns() {
        mock_input!(context ~ VoidFoo ~ value, value_comparison, 1);        
        let core: Rc<RwLock<WheyCoreVoidFoo>> = context.generate();

        core.write().unwrap().reset();
    }

    #[whey(VoidFooContext ~ context)]
    #[should_panic(expected = "the expected inputs for VoidFoo::value were not supplied")]
    fn panics_if_inputs_arent_expected() {
        mock_input!(context ~ VoidFoo ~ value, value_comparison, 1);
        let mut test = context.test_type();

        test.value(EXPECTED_INT + 1);
    }
    
    #[whey(VoidFooContext ~ context)]
    fn compares_against_many_comparisons() {
        for i in 1..=10 {
            mock_input!(context ~ VoidFoo ~ value, move |input| input == i, i);
            let mut test = context.test_type();

            for _ in 0..i {
                test.value(i);
            }
        }
    }
    
    #[whey(VoidFooContext ~ context)]
    fn compares_against_multiple_inputs() {
        for i in 1..=10 {
            mock_input!(context ~ VoidFoo ~ reference, move |one, two| *one == EXPECTED_INT + i && *two == EXPECTED_LONG, i);
            let mut test = context.test_type();

            for _ in 0..i {
                test.reference(&(EXPECTED_INT + i), &EXPECTED_LONG);
            }
        }
    }
}