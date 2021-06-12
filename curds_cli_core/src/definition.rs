use super::*;

pub trait CliArgumentDefinition {
    fn parse(key: String, arguments: &mut ArgumentCollection) -> CliParseResult<Self>
    where Self : Sized;

    fn usage(detailed: bool);
}
