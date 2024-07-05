use std::cell::OnceCell;
use std::path::Path;

use super::*;

pub trait Terminal {
    fn write(&self, message: &str);
}

pub struct CliTerminal {}

impl Terminal for CliTerminal {
    fn write(&self,message: &str) {
        println!("{}", message);
    }
}