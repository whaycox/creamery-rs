#[cfg(test)]
mod tests {
    use super::super::*;

    #[whey_mock]
    trait VoidFoo {
        fn parameterless(&self);
    }

    #[test]
    fn can_call_with_no_expectation() {
        let test_object = TestingVoidFoo::new();
        
        test_object.parameterless();
    }

    #[test]
    fn can_call_and_meet_expectations() {
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
    #[should_panic(expected = "expected 2 calls to TestingVoidFoo::parameterless but recorded 1 instead")]
    fn panics_with_unmet_expectation() {
        let test_object = TestingVoidFoo::new();
        test_object.expect_calls_parameterless(2);

        test_object.parameterless();
    }
    
    #[test]
    fn can_expect_no_calls() {
        let test_object = TestingVoidFoo::new();
        test_object.expect_calls_parameterless(0);
    }
    
    #[test]
    #[should_panic(expected = "expected 0 calls to TestingVoidFoo::parameterless but recorded 1 instead")]
    fn panics_if_doesnt_meet_no_calls() {
        let test_object = TestingVoidFoo::new();
        test_object.expect_calls_parameterless(0);

        test_object.parameterless();
    }
}