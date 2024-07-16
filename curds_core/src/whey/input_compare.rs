#[cfg(test)]
mod tests {
    use super::super::*;

    #[whey_mock]
    trait VoidFoo {
        fn value(&mut self, input: u32);
        fn reference(&mut self, one: &u32, two: &u64);
    }

    fn value_comparison(input: &u32) -> bool { *input == EXPECTED_INT }

    #[test]
    #[should_panic(expected = "not all stored input comparisons for VoidFoo::value have been consumed")]
    fn panics_if_inputs_arent_consumed() {
        let test_object = TestingVoidFoo::new();

        test_object.store_expected_input_value(value_comparison, 1);
    }
    
    #[test]
    fn resets_stored_returns() {
        let test_object = TestingVoidFoo::new();
        test_object.store_expected_input_value(value_comparison, 1);

        test_object.reset();
    }

    #[test]
    #[should_panic(expected = "the expected inputs for VoidFoo::value were not supplied")]
    fn panics_if_inputs_arent_expected() {
        let mut test_object = TestingVoidFoo::new();
        test_object.store_expected_input_value(value_comparison, 1);

        test_object.value(EXPECTED_INT + 1);
    }
    
    #[test]
    fn compares_against_many_comparisons() {
        let mut test_object = TestingVoidFoo::new();

        for i in 1..=10 {
            test_object.store_expected_input_value(move |input| *input == i, i);

            for _ in 0..i {
                test_object.value(i);
            }
        }
    }
    
    #[test]
    fn compares_against_multiple_inputs() {
        let mut test_object = TestingVoidFoo::new();

        for i in 1..=10 {
            test_object.store_expected_input_reference(move |one, two| *one == EXPECTED_INT + i && *two == EXPECTED_LONG, i);

            for _ in 0..i {
                test_object.reference(&(EXPECTED_INT + i), &EXPECTED_LONG);
            }
        }
    }
}