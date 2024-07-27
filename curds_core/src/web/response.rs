use super::HttpStatus;
use std::fmt::Write;

#[derive(Debug)]
pub struct HttpResponse {
    pub status: HttpStatus,

    pub body: Option<String>,
}

impl HttpResponse {
    pub fn new(status: HttpStatus) -> Self {
        Self {
            status,
            
            body: None,
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut response: String = String::new();
        write!(response, "HTTP/1.1 {}\r\n", self.status).unwrap();
        write!(response, "Connection: close\r\n").unwrap();
        if let Some(body) = self.body {
            write!(response, "Content-Length: {}\r\n", body.len()).unwrap();
            write!(response, "Content-Type: text/plain\r\n\r\n").unwrap();
            write!(response, "{}", body).unwrap();
        }
        else {
            write!(response, "\r\n").unwrap();
        }

        response.as_bytes().to_owned()
    }
}