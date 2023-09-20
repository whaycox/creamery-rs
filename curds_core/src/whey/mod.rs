mod call_count;
mod default_return;
mod return_generator;
mod input_compare;
mod sequence;

#[cfg(test)]
use super::*;

#[cfg(test)]
pub use simple::*;

#[cfg(test)]
mod simple {
    use super::super::*;
}