mod core_call_count;
//mod no_expectations;
//mod input_compare_value;
//mod input_compare_reference;
//mod return_value;
//mod return_reference;

#[cfg(test)]
use super::*;

#[cfg(test)]
pub use simple::*;

#[cfg(test)]
mod simple {
    use super::super::*;
}