mod parser;
mod argument_factory;
mod terminal;
mod parseable;

use super::*;
use parser::*;
use argument_factory::*;
use terminal::*;
use parseable::*;

pub struct Cli {}
impl Cli {
    pub fn parse<TParseableOperations>() -> Vec<TParseableOperations>
    where TParseableOperations : Parseable {
        todo!("parse")
    }
    fn parse_internal<TParseableOperations, TGenerator>(generator: TGenerator) -> Vec<TParseableOperations>
    where TParseableOperations : Parseable,
    TGenerator : ServiceGenerator<Rc<CliParser>> {
        generator
            .generate()
            .parse::<TParseableOperations>()
    }
}

#[cfg(test)]
mod tests {
    // use std::{convert::TryInto};

    // use super::*;

    // struct TestOperations {}
    // impl Parseable for TestOperations {
    //     fn parse(factory: Rc<dyn ArgumentFactory>) -> Self { Self {} }
    // }

    // #[service_provider]
    // #[generates_singleton(dyn ArgumentFactory ~ WheyArgumentFactory)]
    // #[generates_singleton(WheyArgumentFactorySetup)]
    // #[generates_singleton(dyn Terminal ~ WheyTerminal)]
    // #[generates_singleton(CliParser)]
    // struct TestingContext {}

    // impl TestingContext {
    //     pub fn setup_argument_factory(&self) -> Rc<WheyArgumentFactorySetup> { Self::generate(&self) }
    // }

    // #[test]
    // fn parsing_operations() {
    //     test_parsing_n_operations(1);
    //     test_parsing_n_operations(5);
    //     test_parsing_n_operations(10);
    //     test_parsing_n_operations(15);
    //     test_parsing_n_operations(100);
    // }
    // fn test_parsing_n_operations(operations: usize) {
    //     let context = TestingContext::construct();
    //     let trues: Box<dyn Setup<(), bool>> = Box::new(ValueSetup::new(Box::new(AnyCompare::new()),true));
    //     trues.set_times(operations.try_into().unwrap());
    //     context
    //         .setup_argument_factory()
    //         .has_arguments(trues);
    //     let falses: Box<dyn Setup<(), bool>> = Box::new(ValueSetup::new(Box::new(AnyCompare::new()),false));
    //     context
    //         .setup_argument_factory()
    //         .has_arguments(falses);

    //     let actual: Vec<TestOperations> = Cli::parse_internal(context);

    //     assert_eq!(operations, actual.len())
    // }
}