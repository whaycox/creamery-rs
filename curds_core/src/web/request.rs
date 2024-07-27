use super::{HttpHeaders, HttpMethod, HttpVersion};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub target: String,
    pub version: HttpVersion,

    pub headers: HttpHeaders,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn new(method: HttpMethod, target: String, version: HttpVersion) -> Self {
        Self {
            method,
            target,
            version,

            headers: HttpHeaders::new(),
            body: None,
        }
    }
}