mod cli;

pub use curds_cli_core::*;
pub use curds_cli_derive::CliArguments;
pub use cli::Cli;

use curds_cli_core::ArgumentCollection;

#[cfg(test)]
use mockall::*;
#[cfg(test)]
use mockall::predicate::*;