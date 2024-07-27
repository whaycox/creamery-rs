use std::fmt::Display;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    Custom(String),
}

impl HttpMethod {
    pub fn new(method: String) -> Self {
        match method.as_str() {
            "GET" => HttpMethod::GET,
            _ => HttpMethod::Custom(method),
        }
    }
}

impl Display for HttpMethod {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(formatter, "GET"),
            HttpMethod::Custom(custom) => write!(formatter, "{}", custom),
        }
    }
}