use super::*;

pub trait CliTerminal {
    fn exit(code: i32);
}

pub struct StandardTerminal;
impl CliTerminal for StandardTerminal {  
    fn exit(code: i32) { std::process::exit(code) }
}