use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use tokio::io::BufReader;
use std::{pin::Pin, time::Duration};
use std::future::Future;
use std::sync::Arc;
use super::*;

use crate::{web::{HttpRequest, HttpVersion, HttpMethod, CurdsWebError, CurdsWebResult}, whey_mock};

#[whey_mock]
pub trait HttpRequestParser {
    fn parse<'a>(&self, stream: &'a mut TcpStream) -> Pin<Box<dyn Future<Output = CurdsWebResult<HttpRequest>> + Send + Sync + 'a>>;
}
pub struct CurdsWebHttpRequestParser<TRequestLineParser, THeaderParser> {
    request_line: Arc<TRequestLineParser>,
    header: Arc<THeaderParser>,
}

pub type ProductionHttpRequestParser = CurdsWebHttpRequestParser<CurdsWebRequestLineParser, CurdsWebHttpHeaderParser>;
impl ProductionHttpRequestParser {
    pub fn new() -> Self {
        ProductionHttpRequestParser {
            request_line: Arc::new(CurdsWebRequestLineParser),
            header: Arc::new(CurdsWebHttpHeaderParser),
        }
    }
}

impl<TRequestLineParser, THeaderParser> CurdsWebHttpRequestParser<TRequestLineParser, THeaderParser> {    
    const BUFFER_SIZE: usize = 1024;
    const TIMEOUT_MS: u64 = 300;
}

impl<TRequestLineParser, THeaderParser> HttpRequestParser for CurdsWebHttpRequestParser<TRequestLineParser, THeaderParser> where
TRequestLineParser : HttpRequestLineParser + Send + Sync + 'static,
THeaderParser : HttpHeaderParser + Send + Sync + 'static {
    fn parse<'a>(&self, stream: &'a mut TcpStream) -> Pin<Box<dyn Future<Output = CurdsWebResult<HttpRequest>> + Send + Sync + 'a>> {
        let request_line_parser = self.request_line.clone();
        let header_parser = self.header.clone();
        Box::pin(async move {
            let request_line_parser = request_line_parser;
            let header_parser = header_parser;
            let mut header_buffer = vec![0; Self::BUFFER_SIZE];
            let mut header_end = 0;
            let mut total_read = 0;
            loop {
                match tokio::time::timeout(Duration::from_millis(Self::TIMEOUT_MS), stream.read(&mut header_buffer[total_read..])).await {
                    Ok(read_result) => match read_result {
                        Ok(read) => {
                            if read == 0 {
                                break;
                            }
                            if let Some(end) = header_end_index(&header_buffer) {
                                header_end = end;
                                total_read += read;
                                break;
                            }
                            total_read += read;
                            header_buffer.extend(vec![0; Self::BUFFER_SIZE]);
                        },
                        Err(read_error) => return Err(CurdsWebError::Read(read_error.to_string())),
                    },
                    Err(_) => return Err(CurdsWebError::Timeout(Self::TIMEOUT_MS)),
                }
            }
            if total_read == 0 {
                return Err(CurdsWebError::NoBytesRead);
            }
    
            let mut body_buffer = header_buffer.split_off(header_end + HEADER_END_LENGTH);
            total_read -= header_buffer.len();
            body_buffer.truncate(total_read);

            let mut request = request_line_parser.parse(&mut header_buffer)?;
            header_parser.add_headers(header_buffer, &mut request)?;

            if let Some(length) = &request.headers.content_length {
                if *length < total_read {
                    return Err(CurdsWebError::RequestFormat(format!("A body length of {} was expected but at least {} have already been read", length, total_read)));
                }
                let missing = length - total_read;
                if missing > 0 {
                    let mut missing_body = vec![0; missing];
                    match tokio::time::timeout(Duration::from_millis(Self::TIMEOUT_MS), stream.read_exact(&mut missing_body)).await {
                        Ok(read_result) => match read_result {
                            Ok(_) => body_buffer.extend(missing_body),
                            Err(read_error) => return Err(CurdsWebError::Read(read_error.to_string())),
                        },
                        Err(_) => return Err(CurdsWebError::Timeout(Self::TIMEOUT_MS)),

                    }
                }
                request.body = Some(String::from_utf8(body_buffer)?);
            }
            Ok(request)
        })
    }
}

const HEADER_END_LENGTH: usize = 4;
static HEADER_END: [u8; HEADER_END_LENGTH] = [0x0D,0x0A,0x0D,0x0A];
fn header_end_index(buffer: &[u8]) -> Option<usize> { byte_sequence_index(buffer, &HEADER_END) }
/* 
#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::test_connection;

    const TEST_BODY_LENGTH: usize = 43;
    fn test_request(content_length: Option<usize>) -> String {
        let mut request = String::new();
        request.push_str("GET /Testing HTTP/1.1\r\n");
        request.push_str(&format!("Content-Length: {}\r\n\r\n", content_length.unwrap_or(TEST_BODY_LENGTH)));
        request.push_str("The quick brown fox jumps over the lazy dog");

        request
    }

    fn test_target() -> Uri {
        Uri::parse("/Testing".as_bytes().to_owned()).unwrap()
    }

    #[tokio::test]
    async fn parses_test_request() {
        let test_object = CurdsWebHttpRequestParser;
        let (_, mut client, mut server) = test_connection("localhost:42123").await;
        client.write_all(test_request(None).as_bytes()).await.unwrap();

        let actual = test_object.parse(&mut server).await.unwrap();

        assert_eq!(HttpMethod::GET, actual.method);
        assert_eq!(test_target(), actual.target);
        assert_eq!(HttpVersion::OnePointOne, actual.version);
        assert_eq!(43, actual.headers.content_length.unwrap());
        assert_eq!("The quick brown fox jumps over the lazy dog", actual.body.unwrap());
    }

    #[tokio::test]
    async fn handles_no_bytes_written() {
        let test_object = CurdsWebHttpRequestParser;
        let (_, _, mut server) = test_connection("localhost:42124").await;

        match test_object.parse(&mut server).await {
            Err(CurdsWebError::NoBytesRead) => {},
            _ => panic!("Did not get the expected error"),
        }
    }

    #[tokio::test]
    async fn times_out_if_only_partially_written() {
        let test_object = CurdsWebHttpRequestParser;
        let (_, mut client, mut server) = test_connection("localhost:42125").await;
        client.write_all("GET /T".as_bytes()).await.unwrap();

        match test_object.parse(&mut server).await {
            Err(CurdsWebError::Timeout(_)) => {},
            _ => panic!("Did not get the expected error"),
        }
    }

    #[tokio::test]
    async fn content_length_too_short_errors() {
        let test_object = CurdsWebHttpRequestParser;
        let (_, mut client, mut server) = test_connection("localhost:42126").await;
        client.write_all(test_request(Some(TEST_BODY_LENGTH - 1)).as_bytes()).await.unwrap();

        match test_object.parse(&mut server).await {
            Err(CurdsWebError::RequestFormat(_)) => {},
            _ => panic!("Did not get the expected error"),
        }
    }

    #[tokio::test]
    async fn content_length_too_long_errors() {
        let test_object = CurdsWebHttpRequestParser;
        let (_, mut client, mut server) = test_connection("localhost:42127").await;
        client.write_all(test_request(Some(TEST_BODY_LENGTH + 1)).as_bytes()).await.unwrap();

        match test_object.parse(&mut server).await {
            Err(CurdsWebError::Timeout(_)) => {},
            _ => panic!("Did not get the expected error"),
        }
    }

    #[test]
    fn parses_headers() {
        let test_line = "GET /Testing HTTP/1.1\r\nOne:      Two     \r\nThree:\r\nOne:Four\r\n\r\n".as_bytes().to_owned();
        let mut expected = HttpRequest::new(HttpMethod::GET, test_target(), HttpVersion::OnePointOne);
        expected.headers.add("One".to_owned(), "Two".to_owned());
        expected.headers.add("Three".to_owned(), String::new());
        expected.headers.add("One".to_owned(), "Four".to_owned());

        assert_eq!(expected, parse_headers(test_line).unwrap());
    }

    #[test]
    fn headers_error_on_incorrect_header() {
        let test_line = "GET /Testing HTTP/1.1\r\nOne:      Two     \r\nThree\r\nOne:Four\r\n\r\n".as_bytes().to_owned();

        match parse_headers(test_line) {
            Err(CurdsWebError::RequestFormat(_)) => {},
            _ => panic!("Did not receive the expected error"),
        }
    }

    #[test]
    fn headers_error_on_incorrect_utf8() {
        let mut test_line = "GET /Testing HTTP/1.1\r\nOne:      Two     \r\nThree:\r\nOne:Four\r\n\r\n".as_bytes().to_owned();
        test_line.insert(27, 0x80);

        match parse_headers(test_line) {
            Err(CurdsWebError::Read(_)) => {},
            _ => panic!("Did not receive the expected error"),
        }
    }

    #[test]
    fn parses_request_line() {
        let test_line = "GET /Testing HTTP/1.1".as_bytes().to_owned();

        assert_eq!(HttpRequest::new(HttpMethod::GET, test_target(), HttpVersion::OnePointOne), parse_request_line(test_line).unwrap());
    }

    #[test]
    fn errors_on_incorrect_parts() {
        let test_line = "GET /Other /Testing HTTP/1.1".as_bytes().to_owned();

        match parse_request_line(test_line) {
            Err(CurdsWebError::RequestFormat(_)) => {},
            _ => panic!("Did not receive the expected error"),
        }
    }

    #[test]
    fn errors_on_invalid_utf8() {
        let mut test_line = "GET /Testing HTTP/1.1".as_bytes().to_owned();
        test_line.insert(1, 0x80);

        match parse_request_line(test_line) {
            Err(CurdsWebError::Read(_)) => {},
            _ => panic!("Did not receive the expected error"),
        }
    }

    #[test]
    fn errors_on_invalid_version() {
        let test_line = "GET /Testing HTTP/4".as_bytes().to_owned();

        match parse_request_line(test_line) {
            Err(CurdsWebError::RequestFormat(_)) => {},
            _ => panic!("Did not receive the expected error"),
        }
    }
} */