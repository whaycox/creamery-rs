use std::collections::HashMap;
use std::sync::OnceLock;
use super::*;

fn hex_map() -> &'static HashMap<u8, u8> { HEX_BYTE_MAP.get_or_init(populate_hex_byte_map) }
static HEX_BYTE_MAP: OnceLock<HashMap<u8, u8>> = OnceLock::new();
fn populate_hex_byte_map() -> HashMap<u8, u8> { 
    vec![
        (0x30, 0x00), (0x31, 0x01), (0x32, 0x02), (0x33, 0x03), (0x34, 0x04),
        (0x35, 0x05), (0x36, 0x06), (0x37, 0x07), (0x38, 0x08), (0x39, 0x09),
        
        (0x41, 0x0A), (0x42, 0x0B), (0x43, 0x0C), (0x44, 0x0D), (0x45, 0x0E), (0x46, 0x0F),
        (0x61, 0x0A), (0x62, 0x0B), (0x63, 0x0C), (0x64, 0x0D), (0x65, 0x0E), (0x66, 0x0F),
    ]
    .into_iter()
    .collect()
 }

pub fn percent_encode() { todo!("percent encode") }
pub fn percent_decode(mut encoded: Vec<u8>) -> CurdsWebResult<Vec<u8>> {
    let map = hex_map();
    let mut starting_index = 0;
    while let Some(percent_index) = byte_sequence_index(&encoded[starting_index..encoded.len()], &PERCENT) {
        let percent_index = starting_index + percent_index;
        if encoded.len() < percent_index + 3 {
            return Err(CurdsWebError::RequestFormat("Invalid percent encoding encountered".to_owned()));
        }
        
        let sequence: Vec<u8> = encoded.drain(percent_index..percent_index + 3).collect();
        match map.get(&sequence[1]) {
            Some(first_byte) => match map.get(&sequence[2]) {
                Some(second_byte) => {
                    let mut decoded = first_byte << 4;
                    decoded |= second_byte;
                    
                    encoded.insert(percent_index, decoded);
                    
                    if encoded.len() == percent_index + 1 {
                        break;
                    }
                    else {
                        starting_index = percent_index + 1;
                    }
                },
                None => return Err(CurdsWebError::RequestFormat(format!("Invalid percent encoding byte {:X}", sequence[2]))),
            },
            None => return Err(CurdsWebError::RequestFormat(format!("Invalid percent encoding byte {:X}", sequence[1])))
        }
    }

    Ok(encoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaves_unencoded_bytes() {
        let test_bytes = "test bytes".as_bytes().to_owned();

        assert_eq!("test bytes".as_bytes(), percent_decode(test_bytes).unwrap());
    }

    #[test]
    fn decodes_bytes() {
        let test_bytes = "%25".as_bytes().to_owned();

        assert_eq!("%".as_bytes(), percent_decode(test_bytes).unwrap());
    }

    #[test]
    fn decodes_multiple_bytes() {
        let test_bytes = "%25hello%20world%21".as_bytes().to_owned();

        assert_eq!("%hello world!".as_bytes(), percent_decode(test_bytes).unwrap());
    }

    #[test]
    fn decodes_case_insensitive_bytes() {
        let test_bytes = "%7bTesting%7D".as_bytes().to_owned();

        assert_eq!("{Testing}".as_bytes(), percent_decode(test_bytes).unwrap());
    }
    
    #[test]
    fn decodes_complex_codepoint() {
        let test_bytes = "%E2%9D%A4".as_bytes().to_owned();

        assert_eq!("❤".as_bytes(), percent_decode(test_bytes).unwrap());
    }
    
    #[test]
    fn decode_errors_with_unfinished() {
        match percent_decode("%2".as_bytes().to_owned()) {
            Err(CurdsWebError::RequestFormat(message)) => assert_eq!("Invalid percent encoding encountered", message),
            _ => panic!("Did not receive expected error"),
        }
    }
    
    #[test]
    fn decode_errors_with_invalid_first_byte() {
        match percent_decode("%G0".as_bytes().to_owned()) {
            Err(CurdsWebError::RequestFormat(message)) => assert_eq!("Invalid percent encoding byte 47", message),
            _ => panic!("Did not receive expected error"),
        }
    }
    
    #[test]
    fn decode_errors_with_invalid_second_byte() {
        match percent_decode("%0z".as_bytes().to_owned()) {
            Err(CurdsWebError::RequestFormat(message)) => assert_eq!("Invalid percent encoding byte 7A", message),
            _ => panic!("Did not receive expected error"),
        }
    }
}