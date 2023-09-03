#[cfg(test)]
mod tests {
    use super::super::*;

    #[whey_mock]
    trait VoidFoo {
        fn parameterless(&self);
    }

    #[test]
    fn parameterless() {
        for expected_counts in 1..=10 {
            parameterless_helper(expected_counts);
        }
    }
    fn parameterless_helper(count: u32) {
        let mut core = WheyCoreVoidFoo::construct();
        core.expect_calls_parameterless(count);

        for _ in 0..count {
            core.record_call_parameterless();
        }
    }
}