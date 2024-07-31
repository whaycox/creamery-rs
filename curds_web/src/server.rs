use super::*;
use curds_core::{io::AsyncFileSystem, web::{CurdsWebError, CurdsWebHttpRequestParser, HttpMethod, HttpRequest, HttpRequestParser, HttpResponse, HttpStatus, HttpVersion}};
use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use tokio::io::BufReader;
use std::{net::SocketAddr, pin::Pin};
use std::future::Future;
use std::sync::Arc;

pub struct CurdsWebServer<TRequestParser, TResponder> {
    request_parser: Arc<TRequestParser>,
    responder: Arc<TResponder>,
}

impl CurdsWebServer<CurdsWebHttpRequestParser, ProductionWebHttpResponder> {
    pub fn new() -> Self {
        Self {
            request_parser: Arc::new(CurdsWebHttpRequestParser),
            responder: Arc::new(ProductionWebHttpResponder::new()),
        }
    }
}

impl<TRequestParser, TResponder> CurdsWebServer<TRequestParser, TResponder> where 
TRequestParser : HttpRequestParser + Send + Sync + 'static,
TResponder : HttpResponder + Send + Sync + 'static {
    pub async fn start(&self) {
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
        log::info!("Listening");
        loop {
            let (stream, socket) = listener.accept().await.unwrap();
            tokio::spawn(self.handle_stream(stream, socket));
        }
    }
    fn handle_stream(&self, mut stream: TcpStream, socket: SocketAddr) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> {
        let request_parser = self.request_parser.clone();
        let responder = self.responder.clone();
        Box::pin(async move {
            log::info!("{}|Connection received", socket);
            if let Some(response) = responder.create(&socket, request_parser.parse(&mut stream).await).await {
                log::info!("{}|Sending response: {}", socket, response.status);
                stream.write_all(&response.to_bytes()).await.unwrap();
                stream.flush().await.unwrap();
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use curds_core::web::TestingHttpRequestParser;

    impl CurdsWebServer<TestingHttpRequestParser, TestingHttpResponder> {
        pub fn test_object() -> Self {
            Self {
                request_parser: Arc::new(TestingHttpRequestParser::new()),
                responder: Arc::new(TestingHttpResponder::new()),
            }
        }
    }

    // #[test]
    // fn something() {
    //     todo!("testing")
    // }
}