use super::*;

pub trait Parseable {
    fn parse(factory: &Rc<dyn ArgumentFactory>) -> Self;
}