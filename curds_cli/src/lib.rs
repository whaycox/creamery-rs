mod cli;

use curds_cli_core::*;

pub use cli::Cli;
pub use curds_cli_derive::CliArguments;

#[cfg(test)]
use mockall::{predicate, Sequence};