use super::*;

pub trait CliTerminal {
    fn write_out(message: &str);
    fn write_error(message: &str);

    fn exit(code: i32);
}

pub struct StandardTerminal;
impl CliTerminal for StandardTerminal {
    fn write_out(message: &str) { println!("{}", message) }
    fn write_error(message: &str) { eprintln!("{}", message) }

    fn exit(code: i32) { std::process::exit(code) }
}