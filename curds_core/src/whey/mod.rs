mod call_count;
mod default_return;
mod return_generator;
mod input_compare;

#[cfg(test)]
use super::*;

#[cfg(test)]
pub use simple::*;

#[cfg(test)]
mod simple {
    pub const EXPECTED_INT: u32 = 123;
    pub const EXPECTED_LONG: u64 = 9876543210;

    #[derive(PartialEq)]
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