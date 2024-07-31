use std::net::SocketAddr;
use curds_core::{web::{CurdsWebResult, CurdsWebError, HttpRequest, HttpResponse, HttpStatus, HttpMethod}, whey_mock};

#[whey_mock]
pub trait HttpResponder {
    fn create(&self, socket: &SocketAddr, parse_result: CurdsWebResult<HttpRequest>) -> Option<HttpResponse>;
}

pub struct CurdsWebHttpResponder;

impl HttpResponder for CurdsWebHttpResponder {
    fn create(&self, socket: &SocketAddr, parse_result: CurdsWebResult<HttpRequest>) -> Option<HttpResponse> {
        match parse_result {
            Ok(request) => match request.method {
                HttpMethod::GET => {
                    log::info!("{}|Request received: {}", socket, request.target);
                    let mut success = HttpResponse::new(HttpStatus::OK);
                    success.body = Some("Hi there this is a response".to_owned());
            
                    Some(success)
                },
                _ => {
                    log::error!("{}|Received a {} request", socket, request.method);
                    Some(HttpResponse::new(HttpStatus::MethodNotAllowed))
                },
            },
            Err(error) => {
                log::error!("{}|Failed to parse HTTP request: {}", socket, error);
                match error {
                    CurdsWebError::NoBytesRead => None,
                    CurdsWebError::Timeout(_) => Some(HttpResponse::new(HttpStatus::RequestTimeout)),
                    _ => Some(HttpResponse::new(HttpStatus::BadRequest)),
                }
            },
        }




    }
}