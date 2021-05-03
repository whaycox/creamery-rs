pub trait CliArgumentDefinition {
    fn parse(key: String, arguments: &mut ArgumentCollection) -> Self;
}

pub struct ArgumentCollection {
    arguments: Vec<String>,
}
impl ArgumentCollection {
    pub fn new(mut arguments: Vec<String>) -> Self {
        arguments.reverse();
        ArgumentCollection {
            arguments: arguments,
        }
    }

    pub fn has_values(&self) -> bool {
        self.arguments.len() > 0
    }

    pub fn pop(&mut self) -> String {
        self.arguments.pop().unwrap()
    }
}