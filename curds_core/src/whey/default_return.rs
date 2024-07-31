#[cfg(test)]
mod tests {
    use super::super::*;

    fn test_delegate(value: &u32, reference: &u32) -> u32 { value * reference }

    #[whey_mock]
    trait ValueFoo {
        fn simple(&self) -> u32;

        fn input(&self, value: u32, reference: &u32) -> u32;
    }

    #[test]
    fn simple_returns_default_value() {
        let test_object = TestingValueFoo::new();
        test_object.default_return_simple(|| EXPECTED_INT);

        assert_eq!(EXPECTED_INT, test_object.simple());
    }
    
    #[test]
    fn assert_doesnt_reset_default_return() {
        let test_value = 234;
        let test_object = TestingValueFoo::new();
        test_object.default_return_simple(move || test_value);

        test_object.assert();

        assert_eq!(test_value, test_object.simple());
    }

    #[test]
    #[should_panic(expected = "a return is necessary for TestingValueFoo::simple but none have been supplied")]
    fn simple_panics_without_default() {
        let test_object = TestingValueFoo::new();

        test_object.simple();
    }
    
    #[test]
    fn default_generator_uses_inputs() {
        let test_object = TestingValueFoo::new();
        test_object.default_return_input(|value, reference| value + reference);

        for i in 1..=10 {
            assert_eq!(EXPECTED_INT + i, test_object.input(EXPECTED_INT, &i));
        }
    }
    
    #[test]
    fn default_generator_can_use_delegate() {
        let test_object = TestingValueFoo::new();
        test_object.default_return_input(test_delegate);

        for i in 1..=10 {
            assert_eq!(EXPECTED_INT * i, test_object.input(EXPECTED_INT, &i));
        }
    }

    fn reference_delegate<'a>() -> &'a u32 { &EXPECTED_INT }

    #[whey_mock]
    trait ReferenceFoo<'a> {
        fn simple(&self) -> &'a u32;
    }

    #[test]
    fn simple_returns_default_reference() {
        let test_object = TestingReferenceFoo::new();
        test_object.default_return_simple(|| &EXPECTED_INT);

        assert_eq!(&EXPECTED_INT, test_object.simple());
    }
    
    #[test]
    fn default_reference_can_use_delegate() {
        let test_object = TestingReferenceFoo::new();
        test_object.default_return_simple(reference_delegate);

        assert_eq!(&EXPECTED_INT, test_object.simple());
    }
}