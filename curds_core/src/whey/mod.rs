mod call_count;
mod default_return;
mod return_generator;
mod input_compare;
mod sequence;
mod context_default_return;
mod context_return_generator;
mod context_input_compare;
mod context_sequence;

#[cfg(test)]
use super::*;

#[cfg(test)]
pub use simple::*;

#[cfg(test)]
mod simple {
    pub const EXPECTED_ITERATIONS: u32 = 10;
    pub const EXPECTED_INT: u32 = 123;
    pub const EXPECTED_LONG: u64 = 9876543210;

    pub struct CustomStruct {
        pub foo: u32,
    }
    impl Default for CustomStruct {
        fn default() -> Self {
            Self { 
                foo: EXPECTED_INT
            }
        }
    }
}