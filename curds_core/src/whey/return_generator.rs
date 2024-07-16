#[cfg(test)]
mod tests {
    use super::super::*;

    #[whey_mock]
    trait ValueFoo {
        fn simple(&self) -> u32;

        fn input(&self, value: u32, reference: &u32) -> u32;

        fn custom(&self) -> CustomStruct;
    }

    fn simple_delegate() -> u32 { 1 }
    
    #[test]
    #[should_panic(expected = "not all stored returns for ValueFoo::simple have been consumed")]
    fn panics_if_returns_arent_consumed() {
        let test_object = TestingValueFoo::new();

        test_object.store_return_simple(simple_delegate, 1);
    }
    
    #[test]
    fn resets_stored_returns() {
        let test_object = TestingValueFoo::new();
        test_object.store_return_simple(simple_delegate, 1);

        test_object.reset();
    }
    
    #[test]
    fn returns_from_many_generators() {
        let test_object = TestingValueFoo::new();

        for i in 1..=10 {
            test_object.store_return_simple(move || i, i);

            for _ in 0..i {
                assert_eq!(i, test_object.simple());
            }
        }
    }
    
    #[test]
    fn returns_from_many_generators_with_input() {
        let test_object = TestingValueFoo::new();

        for i in 1..=10 {
            test_object.store_return_input(move |value, reference| value + reference + i, i);

            for j in 0..i {
                assert_eq!(EXPECTED_INT + &j + i, test_object.input(EXPECTED_INT, &j));
            }
        }
    }
}