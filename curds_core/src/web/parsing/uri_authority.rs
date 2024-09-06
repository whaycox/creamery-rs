use super::*;

impl UriAuthority {
    pub fn parse(bytes: &mut Vec<u8>) -> CurdsWebResult<Option<Self>> {
        let mut authority: Option<Vec<u8>> = None;
        if let Some(double_slash) = byte_sequence_index(&bytes, &DOUBLE_SLASH) {
            if double_slash == 0 {
                bytes.drain(0..DOUBLE_SLASH_LENGTH);
                if let Some(delimiter_index) = delimiter_index(&bytes) {
                    if delimiter_index > 0 {
                        authority = Some(bytes.drain(0..delimiter_index).collect());
                    }
                }
                else if bytes.len() > 0 {
                    authority = Some(bytes.drain(0..bytes.len()).collect());
                }
            }
        }

        if let Some(mut authority) = authority {
            let user_info = Self::parse_user_info(&mut authority)?;
            let host = Self::parse_host(&mut authority)?;
            let port = Self::parse_port(&mut authority)?;

            return Ok(Some(Self {
                user_info,
                host,
                port,
            }))
        }
        Ok(None)
    }
    fn parse_user_info(bytes: &mut Vec<u8>) -> CurdsWebResult<Option<String>> {
        let mut result = Ok(None);
        if let Some(at_index) = byte_sequence_index(bytes, &AT) {
            result = Ok(Some(String::from_utf8(bytes.drain(0..at_index).collect())?));
            bytes.drain(0..DELIMITER_LENGTH);
        }
        
        result
    }
    fn parse_host(bytes: &mut Vec<u8>) -> CurdsWebResult<String> {
        if let Some(opening_bracket_index) = byte_sequence_index(bytes, &OPENING_BRACKET) {
            if opening_bracket_index != 0 {
                Err(CurdsWebError::RequestFormat(format!("There is unrecognized content in the host prior to the declaration of an IP literal, \"{}\"", String::from_utf8(bytes.drain(0..opening_bracket_index).collect())?)))
            }
            else {
                match byte_sequence_index(bytes, &CLOSING_BRACKET) { 
                    Some(closing_bracket_index) => Ok(String::from_utf8(bytes.drain(0..=closing_bracket_index).collect())?),
                    None => Err(CurdsWebError::RequestFormat("The host indicates an IP literal but does not conclude it".to_owned()))
                }
            }
        }
        else if let Some(colon_index) = byte_sequence_index(bytes, &COLON) {
            let result = Ok(String::from_utf8(bytes.drain(0..colon_index).collect())?);
            bytes.drain(0..DELIMITER_LENGTH);

            result
        }
        else if bytes.len() == 0 {
            Err(CurdsWebError::RequestFormat("No bytes were left for the URI host".to_owned()))
        }
        else {
            Ok(String::from_utf8(bytes.drain(0..bytes.len()).collect())?)
        }
    }
    fn parse_port(bytes: &mut Vec<u8>) -> CurdsWebResult<Option<u32>> {
        if bytes.len() > 0 {
            return Ok(Some(String::from_utf8(bytes.drain(0..bytes.len()).collect())?.parse::<u32>()?))
        }
        Ok(None)
    }
}