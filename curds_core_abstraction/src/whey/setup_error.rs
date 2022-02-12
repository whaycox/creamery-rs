use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SetupError {
    ExhaustedConsumption,
    InputComparison,
}
impl Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("data") }
}

impl Error for SetupError {}