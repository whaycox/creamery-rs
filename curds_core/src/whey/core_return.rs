#[cfg(test)]
mod tests {
    use super::super::*;
    
    const EXPECTED_VALUE: u32 = 123;

    #[whey_mock]
    trait ReturnFoo {
        fn value(&self) -> u32;
    }

    #[test]
    fn value_return() {
        let mut core = WheyCoreReturnFoo::construct();

        for times in 1..=10 {
            println!("testing {}", times);
            value_return_helper(&mut core, times);
        }

        panic!("uh oh");
    }
    fn value_return_helper(core: &mut WheyCoreReturnFoo, times: u32) {
        let expected_value = EXPECTED_VALUE + times;
        
        for _ in 0..times {
            assert_eq!(expected_value, core.generate_return_value());            
        }
    }
}