use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use tokio::io::BufReader;
use std::{pin::Pin, time::Duration};
use std::future::Future;

use crate::{web::{HttpRequest, HttpVersion, HttpMethod, CurdsWebError, CurdsWebResult}, whey_mock};


pub trait HttpRequestParser {
    fn parse<'a>(&self, stream: &'a mut TcpStream) -> Pin<Box<dyn Future<Output = CurdsWebResult<HttpRequest>> + Send + Sync + 'a>>;
}

pub struct CurdsWebHttpRequestParser;

impl CurdsWebHttpRequestParser {    
    const BUFFER_SIZE: usize = 1024;
    const TIMEOUT_MS: u64 = 300;
}

impl HttpRequestParser for CurdsWebHttpRequestParser {
    fn parse<'a>(&self, stream: &'a mut TcpStream) -> Pin<Box<dyn Future<Output = CurdsWebResult<HttpRequest>> + Send + Sync + 'a>> {
        Box::pin(async move {
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
            let mut request = parse_headers(header_buffer)?;
            if let Some(length) = &request.headers.content_length {
                let missing = length - total_read;
                if missing > 0 {
                    let mut missing_body = vec![0; missing];
                    match stream.read_exact(&mut missing_body).await {
                        Ok(_) => body_buffer.extend(missing_body),
                        Err(read_error) => return Err(CurdsWebError::Read(read_error.to_string())),
                    }
                }
                request.body = Some(String::from_utf8(body_buffer)?);
            }
            Ok(request)
        })
    }
}

fn byte_sequence_index(buffer: &[u8], sequence: &[u8]) -> Option<usize> {
    let buffer_length = buffer.len();
    let sequence_length = sequence.len();
    if buffer_length >= sequence_length {
        for i in 0..=buffer_length - sequence_length {
            if &buffer[i..i + sequence_length] == sequence {
                return Some(i);
            }
        }
    }
    None
}

const HEADER_END_LENGTH: usize = 4;
static HEADER_END: [u8; HEADER_END_LENGTH] = [0x0D,0x0A,0x0D,0x0A];
fn header_end_index(buffer: &[u8]) -> Option<usize> { byte_sequence_index(buffer, &HEADER_END) }

fn split_on_byte_sequence(mut buffer: Vec<u8>, sequence: &[u8]) -> Vec<Vec<u8>> {
    let mut splits: Vec<Vec<u8>> = Vec::new();
    let sequence_length = sequence.len();
    while let Some(split) = byte_sequence_index(&buffer, sequence) {
        let mut tail = buffer.split_off(split);
        if buffer.len() > 0 {
            splits.push(buffer);
        }
        buffer = tail.split_off(sequence_length);
    }
    if buffer.len() > 0 {
        splits.push(buffer);
    }

    splits
}

const NEW_LINE_LENGTH: usize = 2;
static NEW_LINE: [u8; NEW_LINE_LENGTH] = [0x0D,0x0A];
fn split_on_new_line(buffer: Vec<u8>) -> Vec<Vec<u8>> { split_on_byte_sequence(buffer, &NEW_LINE) }

const SPACE_LENGTH: usize = 1;
static SPACE: [u8; SPACE_LENGTH] = [0x20];
fn split_on_space(buffer: Vec<u8>) -> Vec<Vec<u8>> { split_on_byte_sequence(buffer, &SPACE) }

const HEADER_SEPARATOR_LENGTH: usize = 1;
static HEADER_SEPARATOR: [u8; HEADER_SEPARATOR_LENGTH] = [0x3A];
fn split_header_key_and_value(mut buffer: Vec<u8>) -> Vec<Vec<u8>> {
    let mut splits: Vec<Vec<u8>> = Vec::new();
    if let Some(split) = byte_sequence_index(&buffer, &HEADER_SEPARATOR) {
        let mut tail = buffer.split_off(split);
        if buffer.len() > 0 {
            splits.push(buffer);
        }
        buffer = tail.split_off(HEADER_SEPARATOR_LENGTH);
    }
    splits.push(buffer);

    splits
 }

fn parse_headers(headers: Vec<u8>) -> CurdsWebResult<HttpRequest> {
    let mut split_headers = split_on_new_line(headers);
    let mut request = parse_request_line(split_headers.remove(0))?;
    for header in split_headers {
        let mut splits: Vec<String> = Vec::new();
        for split in split_header_key_and_value(header) {
            splits.push(String::from_utf8(split)?);
        }
        if splits.len() != 2 {
            return Err(CurdsWebError::RequestFormat(format!("Incorrect header: {}", splits.join(", "))));
        }
        let mut value = splits.pop().unwrap();
        let trimmed_value = value.trim();
        let trimmed_length = trimmed_value.len();
        if trimmed_length != value.len() {
            let start = trimmed_value.as_ptr() as usize - value.as_ptr() as usize;
            let end = start + trimmed_length;

            value.drain(..start);
            value.drain(end - start..);
        }
        let key = splits.pop().unwrap();

        request.headers.add(key, value);
    }

    Ok(request)
}
fn parse_request_line(request_line: Vec<u8>) -> CurdsWebResult<HttpRequest> {
    let mut splits: Vec<String> = Vec::new();
    for split in split_on_space(request_line) {
        splits.push(String::from_utf8(split)?);
    }
    if splits.len() != 3 {
        return Err(CurdsWebError::RequestFormat(format!("Incorrect request line: {}", splits.join(", "))));
    }

    let mut iterator = splits.into_iter();
    let method = HttpMethod::new(iterator.next().unwrap());
    let target = iterator.next().unwrap();
    let version = HttpVersion::new(iterator.next().unwrap());

    Ok(HttpRequest::new(method, target, version))
}