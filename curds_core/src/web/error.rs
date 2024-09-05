use std::{num::ParseIntError, string::FromUtf8Error};
use thiserror::Error;

use super::UriPath;

#[derive(Debug, Error)]
pub enum CurdsWebError {
    #[error("The stream closed before any bytes were read")]
    NoBytesRead,
    #[error("There was an error reading from the stream: {0}")]
    Read(String),
    #[error("No bytes were read after {0} ms")]
    Timeout(u64),
    #[error("The structure of the request read is incorrect: {0}")]
    RequestFormat(String),
    #[error("The target \"{0}\" was not found")]
    FileNotFound(UriPath),
}

impl From<FromUtf8Error> for CurdsWebError {
    fn from(value: FromUtf8Error) -> Self { Self::Read(value.to_string()) }
}

impl From<ParseIntError> for CurdsWebError {
    fn from(value: ParseIntError) -> Self { Self::Read(value.to_string()) }
}