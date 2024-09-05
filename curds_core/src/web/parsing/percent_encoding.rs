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
    ].into_iter().collect()
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
        let mut decoded = map.get(&sequence[1]).unwrap() << 4;
        decoded |= map.get(&sequence[2]).unwrap();
        
        encoded.insert(percent_index, decoded);
        
        if encoded.len() == percent_index + 1 {
            break;
        }
        else {
            starting_index = percent_index + 1;
        }
    }

    Ok(encoded)
}