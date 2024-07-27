use std::fmt::Display;

#[derive(Debug)]
pub enum HttpVersion {
    One,
    OnePointOne,
    Two,
    Three,
}

impl HttpVersion {
    pub fn new(version: String) -> Self {
        match version.as_str() {
            "HTTP/1" | "HTTP/1.0" => HttpVersion::One,
            "HTTP/1.1" => HttpVersion::OnePointOne,
            "HTTP/2" | "HTTP/2.0" => HttpVersion::Two,
            "HTTP/3" | "HTTP/3.0" => HttpVersion::Three,
            _ => panic!("Unrecognized version {}", version),
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