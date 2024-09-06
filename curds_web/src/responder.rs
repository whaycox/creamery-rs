use std::net::SocketAddr;
use curds_core::{io::{AsyncFileSystem, FileSystem}, web::{CurdsWebError, CurdsWebResult, HttpMethod, HttpRequest, HttpResponse, HttpStatus}, whey_mock};
use std::pin::Pin;
use std::future::Future;
use std::sync::Arc;
use super::*;

#[whey_mock]
pub trait HttpResponder {
    fn create<'a>(&self, socket: &'a SocketAddr, parse_result: CurdsWebResult<HttpRequest>) -> Pin<Box<dyn Future<Output = Option<HttpResponse>> + Send + Sync + 'a>>;
}

pub struct CurdsWebHttpResponder<TRouter> {
    router: Arc<TRouter>,
}

pub type ProductionWebHttpResponder = CurdsWebHttpResponder<ProductionFileRouter>;
impl ProductionWebHttpResponder {
    pub async fn new() -> CurdsWebResult<Self> {
        Ok(Self {
            router: Arc::new(ProductionFileRouter::new().await?),
        })
    }
}

impl<TRouter> HttpResponder for CurdsWebHttpResponder<TRouter> where
TRouter : FileRouter + Send + Sync + 'static {
    fn create<'a>(&self, socket: &'a SocketAddr, parse_result: CurdsWebResult<HttpRequest>) -> Pin<Box<dyn Future<Output = Option<HttpResponse>> + Send + Sync + 'a>> {
        let router = self.router.clone();
        Box::pin(async move {
            match parse_result {
                Ok(request) => match request.method {
                    HttpMethod::GET => {
                        let condensed_path = request.target.path.condense();
                        log::info!("{}|Request received: {}", socket, condensed_path);
                        match router.route_request(&condensed_path).await {
                            Ok(file) => {
                                let mut response = HttpResponse::new(HttpStatus::OK);
                                response.body = Some(file);
        
                                Some(response)
                            },
                            Err(_) => Some(HttpResponse::new(HttpStatus::NotFound)),
                        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use curds_core::{io::TestingFileSystem, web::HttpVersion};
    use std::net::{Ipv4Addr, SocketAddrV4};

    // impl CurdsWebHttpResponder<TestingFileSystem> {
    //     pub fn test_object() -> Self {
    //         let test_object = Self {
    //             router: Arc::new(TestingFileSystem::new()),
    //         };
    //         test_object.file_system.default_return_read_bytes(|_| Box::pin(async { Ok(test_file()) }));

    //         test_object
    //     }
    // }

    // fn test_file() -> Vec<u8> { "The quick brown fox jumps over the lazy dog".as_bytes().to_owned() }

    // fn test_socket() -> SocketAddr {
    //     SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 80))
    // }

    // #[tokio::test]
    // async fn returns_ok_response() {
    //     let test_object = CurdsWebHttpResponder::test_object();

    //     let actual = test_object.create(&test_socket(), Ok(HttpRequest::new(HttpMethod::GET, "/Testing".to_owned(), HttpVersion::OnePointOne))).await.unwrap();

    //     assert_eq!(HttpStatus::OK, actual.status);
    //     assert_eq!(test_file(), actual.body.unwrap());
    // }
}