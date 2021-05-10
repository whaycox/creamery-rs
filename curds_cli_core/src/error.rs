use thiserror::*;

#[derive(Debug, PartialEq, Error)]
pub enum CliParseError {
    #[error("Unsupported key {key} provided.")]
    UnsupportedKey {
        key: String,
    },
    #[error("An argument value was expected but none provided")]
    MissingValue,
}