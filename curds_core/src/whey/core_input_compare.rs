#[cfg(test)]
mod tests {
    use super::super::*;

    #[whey_mock]
    trait InputFoo {
        fn value(&self, parameter: u32);
    }

    #[test]
    fn value_compare() {
        for expected_value in 1..=10 {
            println!("testing {}", expected_value);
            value_parameterless_helper(expected_value);
        }

        panic!("uh oh");
    }
    fn value_parameterless_helper(value: u32) {
        let mut core = WheyCoreInputFoo::construct();
        core.expect_input_value(Box::new(move |expected| expected == value), 1);
    }
}