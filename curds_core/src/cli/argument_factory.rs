use super::*;
use std::path::Path;

#[whey_mock]
pub trait ArgumentFactory {
    fn application_name(&self) -> String;
    fn create(&self) -> Vec<String>;
}

pub struct CliArgumentFactory;

impl ArgumentFactory for CliArgumentFactory {
    fn application_name(&self) -> String {
        let mut arguments: Vec<String> = std::env::args().collect();
        let application = arguments.remove(0);
        Path::new(&application)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    fn create(&self) -> Vec<String>  {
        let mut arguments: Vec<String> = std::env::args().collect();
        arguments.remove(0);

        arguments
    }
}