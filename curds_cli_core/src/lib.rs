mod argument_collection;
mod argument_factory;
mod definition;
mod error;
mod terminal;

use thiserror::*;
use std::fmt::format;

pub use argument_collection::*;
pub use argument_factory::*;
pub use definition::*;
pub use error::*;
pub use terminal::*;

pub type CliParseResult<TDefinition> = Result<TDefinition, CliParseError>;

#[cfg(test)]
use mockall::*;