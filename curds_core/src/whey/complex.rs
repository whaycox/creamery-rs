#[cfg(test)]
mod tests {
    use super::super::*;

    const TEST_CUSTOM: CustomStruct = CustomStruct {
        foo: 12345,
    };
    
    #[whey_mock]
    trait ComplexFoo {
        fn value(&self, custom: CustomStruct) -> CustomStruct;
        fn reference(&mut self, custom: &CustomStruct);
    }

    #[whey_context(WheyComplexFoo)]
    #[mocks(dyn ComplexFoo)]
    struct ComplexFooContext {}

    #[whey(ComplexFooContext ~ context)]
    fn complex_expectations() {
        mock_sequence!(context ~ [
            ComplexFoo ~ value(|input| input == &TEST_CUSTOM) -> |_| Default::default(),
            ComplexFoo ~ reference(|input| *input == Default::default())
        ]);
        let mut test_object = context.test_type();

        let generated = test_object.value(TEST_CUSTOM);
        test_object.reference(&generated)
    }
}