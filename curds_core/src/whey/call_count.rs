#[cfg(test)]
mod tests {
    use super::super::*;

    #[whey_mock]
    trait VoidFoo {
        fn parameterless(&self);
    }

    #[whey_context]
    #[mocks(VoidFoo)]
    struct CallCountContext {}

    #[test]
    fn void_parameterless_no_expectation() {
        let mut context = CallCountContext::construct();
        let mock: Box<dyn VoidFoo> = context.generate();

        mock.parameterless();
    }

    #[test]
    fn void_parameterless_expectations() {
        let mut context = CallCountContext::construct();
        let core: Rc<RwLock<WheyCoreVoidFoo>> = context.generate();

        for expected_counts in 1..=10 {
            core.write().unwrap().expect_calls_parameterless(expected_counts);

            void_parameterless_expectations_helper(context.generate(), expected_counts);

            context.assert();
        }
    }
    fn void_parameterless_expectations_helper(mock: Box<dyn VoidFoo>, count: u32) {
        for _ in 0..count {
            mock.parameterless();
        }
    }

    #[test]
    #[should_panic(expected = "expected 2 calls to VoidFoo::parameterless but recorded 1 instead")]
    fn void_parameterless_unmet_expectation() {
        let mut context = CallCountContext::construct();
        let core: Rc<RwLock<WheyCoreVoidFoo>> = context.generate();
        core.write().unwrap().expect_calls_parameterless(2);
        let mock: Box<dyn VoidFoo> = context.generate();

        mock.parameterless();
    }
}