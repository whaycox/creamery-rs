use super::*;

pub struct CurdsWebServer<TRequestParser, TResponder> {
    request_parser: Arc<TRequestParser>,
    responder: Arc<TResponder>,
}

impl CurdsWebServer<CurdsWebHttpRequestParser, ProductionWebHttpResponder> {
    pub async fn new() -> CurdsWebResult<Self> {
        Ok(Self {
            request_parser: Arc::new(CurdsWebHttpRequestParser),
            responder: Arc::new(ProductionWebHttpResponder::new().await?),
        })
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
    use curds_core::web::test_connection;
    use curds_core::web::Uri;

    fn test_response() -> HttpResponse { HttpResponse::new(HttpStatus::OK) }

    impl CurdsWebServer<TestingHttpRequestParser, TestingHttpResponder> {
        pub fn test_object() -> Self {
            let test = Self {
                request_parser: Arc::new(TestingHttpRequestParser::new()),
                responder: Arc::new(TestingHttpResponder::new()),
            };
            test.request_parser.default_return_parse(|_| Box::pin(async { Ok(HttpRequest::new(HttpMethod::GET, Uri::default(), HttpVersion::OnePointOne)) }));
            test.responder.default_return_create(|_, _| Box::pin(async { Some(test_response()) }));

            test
        }
    }

    #[tokio::test]
    async fn parses_request_and_creates_response() {
        let test_object = CurdsWebServer::test_object();
        test_object.request_parser.expect_calls_parse(1);
        test_object.responder.expect_calls_create(1);
        let (socket, _, server) = test_connection("localhost:41234").await;

        test_object.handle_stream(server, socket).await;
    }

    #[tokio::test]
    async fn writes_response_to_stream() {
        let test_object = CurdsWebServer::test_object();
        let (socket, mut client, server) = test_connection("localhost:41235").await;

        test_object.handle_stream(server, socket).await;

        let mut written_bytes: Vec<u8> = Vec::new();
        client.read_to_end(&mut written_bytes).await.unwrap();
        assert_eq!(test_response().to_bytes(), written_bytes);
    }

    #[tokio::test]
    async fn doesnt_write_when_no_response() {
        let test_object = CurdsWebServer::test_object();
        test_object.responder.store_return_create(|_, _| Box::pin(async { None }), 1);
        let (socket, mut client, server) = test_connection("localhost:41236").await;

        test_object.handle_stream(server, socket).await;

        let mut written_bytes: Vec<u8> = Vec::new();
        assert_eq!(0, client.read_to_end(&mut written_bytes).await.unwrap());
    }
}