use super::*;

impl UriPath {
    pub fn parse(bytes: &mut Vec<u8>) -> CurdsWebResult<Self> {
        let mut path: Option<Vec<u8>> = None;
        if let Some(delimiter_index) = query_or_fragment_index(bytes) {
            if delimiter_index > 0 {
                path = Some(bytes.drain(0..delimiter_index).collect());
            }
        }
        else if bytes.len() > 0 {
            path = Some(bytes.drain(0..bytes.len()).collect());
        }

        let mut parsed = UriPath::default();
        if let Some(mut path) = path {
            let mut absolute = false;
            if byte_sequence_index(&path, &SLASH) == Some(0) {
                path.drain(0..DELIMITER_LENGTH);
                absolute = true;
            }
            let mut segments = Vec::new();
            for mut segment in split_on_byte_sequence(path, &SLASH) {
                segment = percent_decode(segment)?;
                segments.push(UriPathSegment::new(String::from_utf8(segment)?));
            }

            parsed = Self {
                absolute,
                segments,
            };
        }
        Ok(parsed)
    }
}