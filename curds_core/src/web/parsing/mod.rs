mod request;
mod uri;
mod uri_authority;
mod uri_path;
mod percent_encoding;
mod rules;
mod header;
mod request_line;

pub use request::*;
pub use uri::*;
pub use uri_authority::*;
pub use uri_path::*;
pub use percent_encoding::*;
pub use rules::*;
pub use header::*;
pub use request_line::*;

use super::*;

const DELIMITER_LENGTH: usize = 1;

static SLASH: [u8; DELIMITER_LENGTH] = [0x2F];
static QUERY: [u8; DELIMITER_LENGTH] = [0x3F];
static FRAGMENT: [u8; DELIMITER_LENGTH] = [0x23];
static AT: [u8; DELIMITER_LENGTH] = [0x40];
static COLON: [u8; DELIMITER_LENGTH] = [0x3A];
static OPENING_BRACKET: [u8; DELIMITER_LENGTH] = [0x5B];
static CLOSING_BRACKET: [u8; DELIMITER_LENGTH] = [0x5D];
static PERCENT: [u8; DELIMITER_LENGTH] = [0x25];
static SPACE_SEQUENCE: [u8; DELIMITER_LENGTH] = [0x20];

const DOUBLE_SLASH_LENGTH: usize = 2;
static DOUBLE_SLASH: [u8; DOUBLE_SLASH_LENGTH] = [0x2F,0x2F];

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
fn split_on_new_line(buffer: Vec<u8>) -> Vec<Vec<u8>> { split_on_byte_sequence(buffer, &NEW_LINE) }
fn split_on_space(buffer: Vec<u8>) -> Vec<Vec<u8>> { split_on_byte_sequence(buffer, &SPACE_SEQUENCE) }

fn control_bytes() -> Vec<u8> {
    vec![
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
        0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A,
        0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x7F,
    ]
}
fn tspecial_bytes() -> Vec<u8> {
    vec![
        0x28, 0x29, 0x3C, 0x3E, 0x40, 0x2C, 0x3B, 0x3A, 0x5C,
        0x22, 0x2F, 0x5B, 0x5D, 0x3F, 0x3D, 0x7B, 0x7D, 0x20, 0x09
    ] 
}

static TOKEN_RESTRICTIONS: OnceLock<HashSet<u8>> = OnceLock::new();
fn token_restrictions() -> &'static HashSet<u8> { TOKEN_RESTRICTIONS.get_or_init(build_token_restrictions) }
fn build_token_restrictions() -> HashSet<u8> {
    let mut controls = control_bytes();
    controls.extend(tspecial_bytes());
    controls.into_iter().collect()
}
const ASCII_MAX: u8 = 0x7F;
fn parse_token(bytes: Vec<u8>) -> CurdsWebResult<String> {
    if bytes.len() == 0 {
        return Err(CurdsWebError::Read("No bytes were supplied to be parsed into a token".into()));
    }
    let restrictions = token_restrictions();
    for i in 0..bytes.len() {
        if restrictions.contains(&bytes[i]) || bytes[i] > ASCII_MAX {
            return Err(CurdsWebError::Read(format!("An invalid token character 0x{0:X} was supplied", bytes[i]).into()));
        }
    }
    Ok(String::from_utf8(bytes)?)
}

const DOUBLE_QUOTE_MARK: u8 = 0x22;
static QUOTED_STRING_RESTRICTIONS: OnceLock<HashSet<u8>> = OnceLock::new();
fn quoted_string_restrictions() -> &'static HashSet<u8> { QUOTED_STRING_RESTRICTIONS.get_or_init(build_quoted_string_restrictions) }
fn build_quoted_string_restrictions() -> HashSet<u8> {
    let mut controls = control_bytes();
    controls.insert(0, DOUBLE_QUOTE_MARK);

    controls.into_iter().collect()
}
fn parse_quoted_string(mut bytes: Vec<u8>) -> CurdsWebResult<String> {
    let mut length = bytes.len();
    if length < 2 || bytes[0] != DOUBLE_QUOTE_MARK || bytes[length - 1] != DOUBLE_QUOTE_MARK {
        return Err(CurdsWebError::Read("A quoted string must be properly enclosed by double quotes".into()))
    }
    bytes.remove(length - 1);
    bytes.remove(0);
    length = bytes.len();

    let restrictions = quoted_string_restrictions();
    let mut index = 0;
    loop {
        skip_lws(&bytes, &mut index);
        if index == length {
            break;
        }

        if restrictions.contains(&bytes[index]) || bytes[index] > ASCII_MAX {
            return Err(CurdsWebError::Read(format!("An invalid quoted string character 0x{0:X} was supplied", bytes[index]).into()));
        }

        index += 1;
        if index == length {
            break;
        }
    }

    Ok(String::from_utf8(bytes)?)
}

const CARRIAGE_RETURN: u8 = 0x0D;
const LINE_FEED: u8 = 0x0A;
const SPACE: u8 = 0x20;
const HORIZONTAL_TAB: u8 = 0x32;
fn skip_lws(bytes: &Vec<u8>, index: &mut usize) {
    let length = bytes.len();
    if length > 0 {
        if length >= 3 && *index <= length - 3 && bytes[*index] == CARRIAGE_RETURN && bytes[*index + 1] == LINE_FEED {
            println!("advancing");
            *index += 2;
        }

        loop {
            if bytes[*index] != SPACE && bytes[*index] != HORIZONTAL_TAB {
                break;
            }
    
            println!("advancing 2");
            *index += 1;
            if *index == length {
                break;
            }
        }
    }
}

fn delimiter_index(bytes: &Vec<u8>) -> Option<usize> {
    vec![
        byte_sequence_index(bytes, &SLASH),
        byte_sequence_index(bytes, &QUERY),
        byte_sequence_index(bytes, &FRAGMENT),
    ]
    .into_iter()
    .flatten()
    .min()
}

fn query_or_fragment_index(bytes: &Vec<u8>) -> Option<usize> {
    vec![
        byte_sequence_index(bytes, &QUERY),
        byte_sequence_index(bytes, &FRAGMENT),
    ]
    .into_iter()
    .flatten()
    .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRING: &str = "TheQuickBrownFoxJumpsOverTheLazyDog";
    
    fn test_token_bytes() -> Vec<u8> {
        TEST_STRING.as_bytes().to_owned()
    }

    #[test]
    fn parses_test_token() {
        assert_eq!(TEST_STRING, parse_token(test_token_bytes()).unwrap());
    }

    #[test]
    fn parse_token_errors_if_empty() {
        parse_token(vec![]).expect_err("expected an error parsing token");
    }

    #[test]
    fn parse_token_errors_with_controls() {
        for control in control_bytes() {
            let mut bytes = test_token_bytes();
            bytes.insert(1, control);

            parse_token(bytes).expect_err("expected an error parsing token");
        }
    }

    #[test]
    fn parse_token_errors_with_tspecials() {
        for tspecial in tspecial_bytes() {
            let mut bytes = test_token_bytes();
            bytes.insert(1, tspecial);

            parse_token(bytes).expect_err("expected an error parsing token");
        }
    }

    #[test]
    fn parse_token_errors_beyond_ascii() {
        for non_ascii in ASCII_MAX + 1..=0xFF {
            let mut bytes = test_token_bytes();
            bytes.insert(1, non_ascii);

            parse_token(bytes).expect_err("expected an error parsing token");
        }
    }
        
    fn test_quoted_string_bytes() -> Vec<u8> {
        format!("\"{0}\"", TEST_STRING).as_bytes().to_owned()
    }
    
    #[test]
    fn parses_test_quoted_string() {
        assert_eq!(TEST_STRING, parse_quoted_string(test_quoted_string_bytes()).unwrap());
    }

    #[test]
    fn parses_quoted_string_with_lws() {
        let mut test_bytes: Vec<u8> = Vec::new();
        test_bytes.push(DOUBLE_QUOTE_MARK);
        test_bytes.extend(test_token_bytes());
        test_bytes.push(CARRIAGE_RETURN);
        test_bytes.push(LINE_FEED);
        test_bytes.extend(test_token_bytes());
        test_bytes.push(DOUBLE_QUOTE_MARK);

        assert_eq!(format!("{0}\r\n{0}", TEST_STRING), parse_quoted_string(test_bytes).unwrap());
    }

    #[test]
    fn parse_quoted_string_errors_if_too_short() {
        parse_quoted_string("\"".as_bytes().to_owned()).expect_err("expected an error parsing quoted string");
    }

    #[test]
    fn parse_quoted_string_errors_if_no_opening_quote() {
        let mut bytes = test_quoted_string_bytes();
        bytes.remove(0);
        
        parse_quoted_string(bytes).expect_err("expected an error parsing quoted string");
    }

    #[test]
    fn parse_quoted_string_errors_if_no_closing_quote() {
        let mut bytes = test_quoted_string_bytes();
        bytes.remove(bytes.len() - 1);
        
        parse_quoted_string(bytes).expect_err("expected an error parsing quoted string");
    }
    
    #[test]
    fn parse_quoted_string_errors_with_contained_quote() {
        let mut bytes = test_quoted_string_bytes();
        bytes.insert(1, DOUBLE_QUOTE_MARK);

        parse_quoted_string(bytes).expect_err("expected an error parsing quoted string");
    }
    
    #[test]
    fn parse_quoted_string_errors_with_controls() {
        for control in control_bytes() {
            let mut bytes = test_quoted_string_bytes();
            bytes.insert(1, control);

            parse_quoted_string(bytes).expect_err("expected an error parsing quoted string");
        }
    }

    #[test]
    fn parse_quoted_string_errors_beyond_ascii() {
        for non_ascii in ASCII_MAX + 1..=0xFF {
            let mut bytes = test_quoted_string_bytes();
            bytes.insert(1, non_ascii);

            parse_quoted_string(bytes).expect_err("expected an error parsing quoted string");
        }
    }

    fn test_lws(preceding_line: bool, spaces: u32, tabs: u32) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        if preceding_line {
            bytes.push(CARRIAGE_RETURN);
            bytes.push(LINE_FEED);
        }
        for _ in 0..spaces {
            bytes.push(SPACE);
        }
        for _ in 0..tabs {
            bytes.push(HORIZONTAL_TAB);
        }

        bytes
    }

    #[test]
    fn skip_lws_skips_spaces() {
        for i in 0..15 {
            let test_bytes = test_lws(false, i, 0);
            let mut index = 0;
            
            skip_lws(&test_bytes, &mut index);

            assert_eq!(index, test_bytes.len());
        }
    }

    #[test]
    fn skip_lws_skips_tabs() {
        for i in 0..15 {
            let test_bytes = test_lws(false, 0, i);
            let mut index = 0;
            
            skip_lws(&test_bytes, &mut index);

            assert_eq!(index, test_bytes.len());
        }
    }

    #[test]
    fn skip_lws_skips_leading_newline() {
        let test_bytes = test_lws(true, 1, 1);
        let mut index = 0;
        
        skip_lws(&test_bytes, &mut index);

        assert_eq!(index, test_bytes.len());
    }

    #[test]
    fn skip_lws_doesnt_skip_trailing_newline() {
        let mut test_bytes = test_lws(true, 1, 1);
        test_bytes.push(CARRIAGE_RETURN);
        test_bytes.push(LINE_FEED);
        let mut index = 0;
        
        skip_lws(&test_bytes, &mut index);

        assert_eq!(4, index);
    }
}