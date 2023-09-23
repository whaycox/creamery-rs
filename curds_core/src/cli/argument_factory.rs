use super::*;

#[whey_mock]
#[expect(has_arguments() -> true)]
pub trait ArgumentFactory {
    fn has_arguments(&self) -> bool;
    fn next(&self) -> String;
}