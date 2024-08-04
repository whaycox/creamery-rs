use std::net::SocketAddr;
use curds_core::{io::{AsyncFileSystem, FileSystem}, web::{CurdsWebError, CurdsWebResult, HttpMethod, HttpRequest, HttpResponse, HttpStatus}, whey_mock};
use std::pin::Pin;
use std::future::Future;
use std::sync::Arc;

#[whey_mock]
pub trait HttpResponder {
    fn create<'a>(&self, socket: &'a SocketAddr, parse_result: CurdsWebResult<HttpRequest>) -> Pin<Box<dyn Future<Output = Option<HttpResponse>> + Send + Sync + 'a>>;
}

pub struct CurdsWebHttpResponder<TFileSystem> {
    file_system: Arc<TFileSystem>,
}

pub type ProductionWebHttpResponder = CurdsWebHttpResponder<AsyncFileSystem>;
impl ProductionWebHttpResponder {
    pub fn new() -> Self {
        Self {
            file_system: Arc::new(AsyncFileSystem),
        }
    }
}

impl<TFileSystem> HttpResponder for CurdsWebHttpResponder<TFileSystem> where
TFileSystem : FileSystem + Send + Sync + 'static {
    fn create<'a>(&self, socket: &'a SocketAddr, parse_result: CurdsWebResult<HttpRequest>) -> Pin<Box<dyn Future<Output = Option<HttpResponse>> + Send + Sync + 'a>> {
        let file_system = self.file_system.clone();
        Box::pin(async move {
            match parse_result {
                Ok(request) => match request.method {
                    HttpMethod::GET => {
                        log::info!("{}|Request received: {}", socket, request.target);
                        let mut response = HttpResponse::new(HttpStatus::OK);
                        match file_system.read_bytes("testing.txt").await {
                            Ok(body) => response.body = Some(body),
                            Err(read_error) => {
                                log::error!("{}|Failed to read file: {}", socket, read_error);
                                response = HttpResponse::new(HttpStatus::NotFound);
                            },
                        }

                        Some(response)
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
        })
    }
}