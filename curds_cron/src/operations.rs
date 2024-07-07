use super::*;

#[cli_arguments]
enum TestOperations {
    FirstBoolean,
    SecondBool,
    Message(String, u32),
    Point { x: u32, y: u32 },
}