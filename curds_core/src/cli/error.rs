use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliArgumentParseError {
    #[error("There are no more arguments but more are expected")]
    ArgumentExpected,
    #[error("Value \"{0}\" not recognized as an operation key")]
    UnrecognizedKey(String),
    #[error("The value \"{0}\" could not be properly parsed into the expected value: {1}")]
    Parse(String, String),
}
