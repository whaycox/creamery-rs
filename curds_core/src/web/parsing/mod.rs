mod request;
mod uri;
mod uri_authority;
mod uri_path;
mod percent_encoding;

pub use request::*;
pub use uri::*;
pub use uri_authority::*;
pub use uri_path::*;
pub use percent_encoding::*;

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

static SPACE: [u8; DELIMITER_LENGTH] = [0x20];
fn split_on_space(buffer: Vec<u8>) -> Vec<Vec<u8>> { split_on_byte_sequence(buffer, &SPACE) }