use super::*;
use std::fmt::{Display, Formatter};

pub type Result<TResponse> = core::result::Result<TResponse, DispatchError>;

#[derive(Debug)]
pub enum DispatchError {
    PipelineError(String),
}

impl Display for DispatchError {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        match self {
            DispatchError::PipelineError(name) => write!(formatter, "an error occurred while executing the pipeline for {}", name),
        }
    }
}

impl Error for DispatchError {}