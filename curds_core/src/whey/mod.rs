mod call_count;
mod dummy_default;

#[cfg(test)]
use super::*;

#[cfg(test)]
pub use simple::*;

#[cfg(test)]
mod simple {
    use super::super::*;
}