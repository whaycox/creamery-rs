use super::*;

#[whey_mock]
pub trait ArgumentFactory {
    fn create(&self) -> Vec<String>;
}

#[injected]
pub struct CliArgumentFactory {}

impl ArgumentFactory for CliArgumentFactory {
    fn create(&self) -> Vec<String>  {
        todo!("argument factory create")
    }
}