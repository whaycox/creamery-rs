use super::*;

#[cfg_attr(test, whey_mock)]
pub trait ArgumentFactory {
    fn has_arguments(&self) -> bool;
    fn next(&self) -> String;
}