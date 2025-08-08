use super::*;

pub trait HttpRequestLineParser {
    fn parse(&self, bytes: &mut Vec<u8>) -> CurdsWebResult<HttpRequest>;
}

pub struct CurdsWebRequestLineParser;

impl HttpRequestLineParser for CurdsWebRequestLineParser {
    fn parse(&self, bytes: &mut Vec<u8>) -> CurdsWebResult<HttpRequest> {
        match byte_sequence_index(&bytes, &NEW_LINE) {
            Some(index) => {
                let request_line: Vec<u8> = bytes
                    .drain(0..index)
                    .collect();

                let splits: Vec<Vec<u8>> = split_on_space(request_line);
                match splits.len() {
                    2 => {
                        let mut iterator = splits.into_iter();
                        let method = HttpMethod::new(String::from_utf8(iterator.next().unwrap())?);
                        let target = Uri::parse(iterator.next().unwrap())?;
                        return Ok(HttpRequest::new(method, target, HttpVersion::ZeroPointNine))
                    },
                    3 => {
                        let mut iterator = splits.into_iter();
                        let method = HttpMethod::new(String::from_utf8(iterator.next().unwrap())?);
                        let target = Uri::parse(iterator.next().unwrap())?;
                        let version = HttpVersion::new(String::from_utf8(iterator.next().unwrap())?)?;
                        return Ok(HttpRequest::new(method, target, version))
                    },
                    _ => return Err(CurdsWebError::RequestFormat(format!("Incorrect request line parts; there are {} but 3 is expected", splits.len()))),
                }
            },
            None => Err(CurdsWebError::RequestFormat("No newline delimiter found within the request line.".to_string())),
        }
    }
}