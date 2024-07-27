use curds_core::web::{CurdsWebError, CurdsWebHttpRequestParser, HttpMethod, HttpRequest, HttpRequestParser, HttpResponse, HttpStatus, HttpVersion};
use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use tokio::io::BufReader;
use std::{net::SocketAddr, pin::Pin};
use std::future::Future;
use std::sync::Arc;

pub struct CurdsWebServer<TRequestParser> {
    request_parser: Arc<TRequestParser>,
}

impl CurdsWebServer<CurdsWebHttpRequestParser> {
    pub fn new() -> Self {
        Self {
            request_parser: Arc::new(CurdsWebHttpRequestParser),
        }
    }
}

impl<TRequestParser> CurdsWebServer<TRequestParser> where 
TRequestParser : HttpRequestParser + Send + Sync + 'static {
    pub async fn start(&self) {
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
        log::info!("Listening");
        loop {
            let (stream, socket) = listener.accept().await.unwrap();
            tokio::spawn(self.handle_stream(stream, socket));
        }
    }
    fn handle_stream(&self, mut stream: TcpStream, socket: SocketAddr) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> {
        let cloned = self.request_parser.clone();
        Box::pin(async move {
            let request_parser = cloned;
            log::info!("{}|Connection received", socket);
            let mut response = None;
            match request_parser.parse(&mut stream).await {
                Ok(request) => match request.method {
                    HttpMethod::GET => {
                        log::info!("{}|Request received: {}", socket, request.target);
            
                        let mut success = HttpResponse::new(HttpStatus::OK);
                        success.body = Some("Hi there this is a response".to_owned());
                        response = Some(success);
                    },
                    _ => {
                        log::error!("{}|Received a {} request", socket, request.method);
                        response = Some(HttpResponse::new(HttpStatus::MethodNotAllowed));
                    },
                },
                Err(error) => {
                    log::error!("{}|Failed to parse HTTP request: {}", socket, error);
                    match error {
                        CurdsWebError::NoBytesRead => {},
                        CurdsWebError::Timeout(_) => { response = Some(HttpResponse::new(HttpStatus::RequestTimeout)); },
                        _ => { response = Some(HttpResponse::new(HttpStatus::BadRequest)); },
                    }
                },
            }
            if let Some(content) = response {
                log::info!("{}|Response sending: {}", socket, content.status);
                stream.write_all(&content.to_bytes()).await.unwrap();
                stream.flush().await.unwrap();
            }
        })
    }
}