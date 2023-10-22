use super::*;

#[whey_mock]
pub trait ArgumentFactory {
    fn create(&self) -> Vec<String>;
}

#[injected]
pub struct CliArgumentFactory {}

impl ArgumentFactory for CliArgumentFactory {
    fn create(&self) -> Vec<String>  {
        let mut arguments: Vec<String> = std::env::args().collect();
        arguments.remove(0);

        arguments
    }
}