#[cfg(test)]
mod tests {
    use super::super::*;

    #[whey_mock]
    trait DependencyA {
        fn generate_value(&self) -> u32;
        fn finalize(&self);
    }

    #[whey_mock]
    trait DependencyB {
        fn process_value(&self, input: u32) -> u32;
    }

    #[injected]
    struct Consumer {
        a: Box<dyn DependencyA>,
        b: Box<dyn DependencyB>,
    }

    impl Consumer {
        fn value(&self) -> u32 {
            let intermediate = self.a.generate_value();
            let value = self.b.process_value(intermediate);
            self.a.finalize();

            return value;
        }
    }

    #[whey_context(Consumer)]
    #[mocks(dyn DependencyA)]
    #[mocks(dyn DependencyB)]
    struct SequenceContext {}

    fn test_comparison(input: &u32) -> bool { *input == EXPECTED_INT }
    fn test_generator() -> u32 { EXPECTED_INT }

    #[whey(SequenceContext ~ context)]
    fn calls_in_correct_order_is_expected() {
        let test_object = context.test_type();

        for i in 0..=10 {
            mock_sequence!(context ~ [
                DependencyA ~ generate_value() -> test_generator,
                DependencyB ~ process_value(test_comparison) -> move |input| input + i,
                DependencyA ~ finalize(),
            ]);
            
            assert_eq!(EXPECTED_INT + i, test_object.value());
            context.assert();
        }
    }

    #[whey(SequenceContext ~ context)]
    #[should_panic(expected = "sequence expected a method of generate_value but was provided finalize")]
    fn calls_in_wrong_order_is_expected() {
        mock_sequence!(context ~ [
            DependencyA ~ generate_value(),
            DependencyA ~ finalize(),
        ]);
        let a: Box<dyn DependencyA> = context.generate();

        a.finalize();
    }

    #[whey(SequenceContext ~ context)]
    #[should_panic(expected = "the expected inputs for DependencyB::process_value were not supplied")]
    fn sequence_performs_input_comparison() {
        let test_object = context.test_type();
        mock_sequence!(context ~ [
            DependencyA ~ generate_value() -> test_generator,
            DependencyB ~ process_value(|input| *input != EXPECTED_INT),
        ]);

        test_object.value();
    }
}