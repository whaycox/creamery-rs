#[cfg(test)]
mod tests {
    use super::super::*;

    #[whey_mock]
    trait VoidFoo {
        fn parameterless(&self);
    }

    #[test]
    fn void_parameterless_no_expectation() {
        let test_object = TestingVoidFoo::new();
        
        test_object.parameterless();
    }

    #[test]
    fn void_parameterless_expectations() {
        let test_object = TestingVoidFoo::new();

        for expected_counts in 1..=10 {
            test_object.expect_calls_parameterless(expected_counts);
            
            for _ in 0..expected_counts {
                test_object.parameterless();
            }

            test_object.assert();
        }
    }

    #[test]
    #[should_panic(expected = "expected 2 calls to VoidFoo::parameterless but recorded 1 instead")]
    fn void_parameterless_unmet_expectation() {
        let test_object = TestingVoidFoo::new();
        test_object.expect_calls_parameterless(2);

        test_object.parameterless();
    }
}