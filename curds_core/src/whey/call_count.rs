#[cfg(test)]
mod tests {
    use super::super::*;

    #[whey_mock]
    trait VoidFoo {
        fn parameterless(&self);
    }

    #[whey_context(WheyVoidFoo)]
    #[mocks(dyn VoidFoo)]
    struct CallCountContext {}

    #[whey(CallCountContext ~ context)]
    fn void_parameterless_no_expectation() {
        context
            .test_type()
            .parameterless();
    }

    #[whey(CallCountContext ~ context)]
    fn void_parameterless_expectations() {
        for expected_counts in 1..=10 {
            expect_calls!(context ~ VoidFoo ~ parameterless, expected_counts);
            
            for _ in 0..expected_counts {
                context
                    .test_type()
                    .parameterless();
            }

            context.assert();
        }
    }

    #[whey(CallCountContext ~ context)]
    #[should_panic(expected = "expected 2 calls to VoidFoo::parameterless but recorded 1 instead")]
    fn void_parameterless_unmet_expectation() {
        expect_calls!(context ~ VoidFoo ~ parameterless, 2);

        context
            .test_type()
            .parameterless();
    }
}