use super::*;

#[whey_mock]
pub trait Terminal {
    fn write(&self, message: &str);
}

pub struct CliTerminal;

impl Terminal for CliTerminal {
    fn write(&self,message: &str) {
        println!("{}", message);
    }
}