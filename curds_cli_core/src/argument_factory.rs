use super::*;

pub trait CliArgumentFactory {
    fn arguments() -> ArgumentCollection;
}

pub struct EnvArgumentFactory;
impl CliArgumentFactory for EnvArgumentFactory {
    fn arguments() -> ArgumentCollection { 
        ArgumentCollection::new(std::env::args().collect())
    }
}