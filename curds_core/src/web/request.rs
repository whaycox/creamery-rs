use super::{HttpHeaders, HttpMethod, HttpVersion, Uri};

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub target: Uri,
    pub version: HttpVersion,

    pub headers: HttpHeaders,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn new(method: HttpMethod, target: Uri, version: HttpVersion) -> Self {
        Self {
            method,
            target,
            version,

            headers: HttpHeaders::new(),
            body: None,
        }
    }
}