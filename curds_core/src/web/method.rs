use std::fmt::Display;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    Custom(String),
}

impl HttpMethod {
    pub fn new(method: String) -> Self {
        match method.as_str() {
            "GET" => HttpMethod::GET,
            "HEAD" => HttpMethod::HEAD,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "CONNECT" => HttpMethod::CONNECT,
            "OPTIONS" => HttpMethod::OPTIONS,
            "TRACE" => HttpMethod::TRACE,
            _ => HttpMethod::Custom(method),
        }
    }
}

impl Display for HttpMethod {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(formatter, "GET"),
            HttpMethod::HEAD => write!(formatter, "HEAD"),
            HttpMethod::POST => write!(formatter, "POST"),
            HttpMethod::PUT => write!(formatter, "PUT"),
            HttpMethod::DELETE => write!(formatter, "DELETE"),
            HttpMethod::CONNECT => write!(formatter, "CONNECT"),
            HttpMethod::OPTIONS => write!(formatter, "OPTIONS"),
            HttpMethod::TRACE => write!(formatter, "TRACE"),
            HttpMethod::Custom(custom) => write!(formatter, "{}", custom),
        }
    }
}