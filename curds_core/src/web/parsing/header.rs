use super::*;

pub trait HttpHeaderParser {
    fn add_headers(&self, bytes: Vec<u8>, request: &mut HttpRequest) -> CurdsWebResult<()>;
}

pub struct CurdsWebHttpHeaderParser;

impl HttpHeaderParser for CurdsWebHttpHeaderParser {
    fn add_headers(&self, bytes: Vec<u8>, request: &mut HttpRequest) -> CurdsWebResult<()> {
        let mut split_headers = split_on_new_line(bytes);
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

        Ok(())
    }
}
fn split_header_key_and_value(mut buffer: Vec<u8>) -> Vec<Vec<u8>> {
    let mut splits: Vec<Vec<u8>> = Vec::new();
    if let Some(split) = byte_sequence_index(&buffer, &COLON) {
        let mut tail = buffer.split_off(split);
        if buffer.len() > 0 {
            splits.push(buffer);
        }
        buffer = tail.split_off(DELIMITER_LENGTH);
    }
    splits.push(buffer);

    splits
 }