use std::fmt::Display;
use super::{CurdsWebResult, CurdsWebError};

#[derive(Debug, PartialEq)]
pub enum HttpVersion {
    One,
    OnePointOne,
    Two,
    Three,
}

impl HttpVersion {
    pub fn new(version: String) -> CurdsWebResult<Self> {
        match version.as_str() {
            "HTTP/1" | "HTTP/1.0" => Ok(HttpVersion::One),
            "HTTP/1.1" => Ok(HttpVersion::OnePointOne),
            "HTTP/2" | "HTTP/2.0" => Ok(HttpVersion::Two),
            "HTTP/3" | "HTTP/3.0" => Ok(HttpVersion::Three),
            _ => Err(CurdsWebError::RequestFormat(format!("Unrecognized HTTP version: {}", version)),)
        }
    }
}

impl Display for HttpVersion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::One => write!(formatter, "HTTP/1.0"),
            HttpVersion::OnePointOne => write!(formatter, "HTTP/1.1"),
            HttpVersion::Two => write!(formatter, "HTTP/2"),
            HttpVersion::Three => write!(formatter, "HTTP/3"),
        }
    }
}