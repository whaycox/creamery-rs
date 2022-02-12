use curds_core_macro::injected;

use super::*;

#[injected]
pub struct CliParser {
    factory: Rc<dyn ArgumentFactory>,
    terminal: Rc<dyn Terminal>,
}

impl CliParser {
    pub fn parse<TParseableOperations>(&self) -> Vec<TParseableOperations>
    where TParseableOperations : Parseable {
        let mut parsed: Vec<TParseableOperations> = Vec::new();
        while self.factory.has_arguments() {
            parsed.push(TParseableOperations::parse(&self.factory))
        }
        
        parsed
    }
}