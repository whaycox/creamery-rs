use super::*;

impl Uri {
    pub fn parse(mut bytes: Vec<u8>) -> CurdsWebResult<Self> {
        let scheme = Self::parse_scheme(&mut bytes)?;
        let authority = UriAuthority::parse(&mut bytes)?;
        let path = UriPath::parse(&mut bytes)?;
        let query = Self::parse_query(&mut bytes)?;
        let fragment = Self::parse_fragment(&mut bytes)?;

        Ok(Self{
            scheme,
            authority,
            path,
            query,
            fragment,
        })
    }
    fn parse_scheme(bytes: &mut Vec<u8>) -> CurdsWebResult<Option<String>> {
        let mut scheme = None;
        if let Some(potential_scheme_delimiter) = byte_sequence_index(&bytes, &COLON) {
            if let Some(next_delimiter) = delimiter_index(bytes) {
                if potential_scheme_delimiter < next_delimiter {
                    scheme = Some(String::from_utf8(bytes.drain(0..potential_scheme_delimiter).collect())?);
                    bytes.drain(0..DELIMITER_LENGTH);
                }
            }
            else {
                scheme = Some(String::from_utf8(bytes.drain(0..potential_scheme_delimiter).collect())?);
                bytes.drain(0..DELIMITER_LENGTH);
            }
        }

        Ok(scheme)
    }
    fn parse_query(bytes: &mut Vec<u8>) -> CurdsWebResult<Option<String>> {
        let mut query = None;
        if let Some(query_index) = byte_sequence_index(bytes, &QUERY) {
            if query_index == 0 {
                bytes.drain(0..DELIMITER_LENGTH);
                if let Some(fragment_index) = byte_sequence_index(bytes, &FRAGMENT) {
                    let query_bytes = percent_decode(bytes.drain(0..fragment_index).collect())?;
                    query = Some(String::from_utf8(query_bytes)?);
                }
                else if bytes.len() > 0 {
                    let query_bytes = percent_decode(bytes.drain(0..bytes.len()).collect())?;
                    query = Some(String::from_utf8(query_bytes)?);
                }
            }
        }

        Ok(query)
    }
    fn parse_fragment(bytes: &mut Vec<u8>) -> CurdsWebResult<Option<String>> {
        let mut fragment = None;
        if let Some(fragment_index) = byte_sequence_index(bytes, &FRAGMENT) {
            if fragment_index == 0 {
                bytes.drain(0..DELIMITER_LENGTH);
                if bytes.len() > 0 {
                    fragment = Some(String::from_utf8(bytes.drain(0..bytes.len()).collect())?);
                }
            }
        }

        Ok(fragment)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_bytes(input: &str) -> Vec<u8> { input.as_bytes().to_owned() }

    #[test]
    fn parse_rfc_example() {
        let expected = Uri::new()
            .scheme("foo")
            .authority()
                .host("example.com")
                .port(8042)
                .uri()
            .path()
                .segment("/over")
                .segment("there")
                .uri()
            .query("name=ferret")
            .fragment("nose")
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("foo://example.com:8042/over/there?name=ferret#nose")).unwrap());
    }

    #[test]
    fn parse_rfc_urn_example() {
        let expected = Uri::new()
            .scheme("urn")
            .path()
                .segment("example:animal:ferret:nose")
                .uri()
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("urn:example:animal:ferret:nose")).unwrap());
    }

    #[test]
    fn parse_random_example_1() {
        let expected = Uri::new()
            .scheme("https")
            .authority()
                .host("example.com")
                .uri()
            .path()
                .segment("/path")
                .segment("to")
                .segment("resource")
                .uri()
            .query("query=some value")
            .fragment("fragment")
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("https://example.com/path/to/resource?query=some%20value#fragment")).unwrap());
    }

    #[test]
    fn parse_random_example_2() {
        let expected = Uri::new()
            .scheme("mailto")
            .path()
                .segment("user.name+alias@sub.example.com")
                .uri()
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("mailto:user.name+alias@sub.example.com")).unwrap());
    }

    #[test]
    fn parse_random_example_3() {
        let expected = Uri::new()
            .scheme("ftp")
            .authority()
                .user_info("user:pass")
                .host("ftp.example.com")
                .uri()
            .path()
                .segment("//file name.txt")
                .uri()
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("ftp://user:pass@ftp.example.com/%2Ffile%20name.txt")).unwrap());
    }

    #[test]
    fn parse_random_example_4() {
        let expected = Uri::new()
            .scheme("http")
            .authority()
                .host("example.com")
                .port(8080)
                .uri()
            .path()
                .segment("/some")
                .segment("path;params")
                .uri()
            .query("query=example")
            .fragment("frag")
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("http://example.com:8080/some/path;params?query=example#frag")).unwrap());
    }

    #[test]
    fn parse_random_example_5() {
        let expected = Uri::new()
            .scheme("urn")
            .path()
                .segment("ietf:rfc:3986")
                .uri()
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("urn:ietf:rfc:3986")).unwrap());
    }

    #[test]
    fn parse_random_example_6() {
        let expected = Uri::new()
            .scheme("file")
            .path()
                .segment("/C:")
                .segment("Program Files")
                .segment("Example App")
                .segment("config.ini")
                .uri()
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("file:///C:/Program%20Files/Example%20App/config.ini")).unwrap());
    }

    #[test]
    fn parse_random_example_7() {
        let expected = Uri::new()
            .scheme("tel")
            .path()
                .segment("+1-800-555-1212")
                .uri()
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("tel:+1-800-555-1212")).unwrap());
    }

    #[test]
    fn parse_random_example_8() {
        let expected = Uri::new()
            .scheme("news")
            .path()
                .segment("comp.infosystems.www.servers.unix")
                .uri()
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("news:comp.infosystems.www.servers.unix")).unwrap());
    }

    #[test]
    fn parse_random_example_9() {
        let expected = Uri::new()
            .scheme("https")
            .authority()
                .user_info("user:pass")
                .host("host.example.com")
                .port(8443)
                .uri()
            .path()
                .segment("/some")
                .segment("path")
                .uri()
            .query("arg1=value1&arg2={encoded}")
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("https://user:pass@host.example.com:8443/some/path?arg1=value1&arg2=%7Bencoded%7D")).unwrap());
    }

    #[test]
    fn parse_random_example_10() {
        let expected = Uri::new()
            .authority()
                .host("example.com")
                .uri()
            .path()
                .segment("/path")
                .segment("with")
                .segment("no")
                .segment("scheme")
                .uri()
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("//example.com/path/with/no/scheme")).unwrap());
    }

    #[test]
    fn parse_random_example_11() {
        let expected = Uri::new()
            .path()
                .segment("data")
                .segment("resource.txt")
                .uri()
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("data/resource.txt")).unwrap());
    }

    #[test]
    fn parse_random_example_12() {
        let expected = Uri::new()
            .path()
                .segment("..")
                .segment("relative")
                .segment("path")
                .segment("to")
                .segment("resource")
                .uri()
            .query("arg=value")
            .fragment("section-2")
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("../relative/path/to/resource?arg=value#section-2")).unwrap());
    }

    #[test]
    fn parse_random_example_13() {
        let expected = Uri::new()
            .authority()
                .user_info("user")
                .host("host")
                .port(80)
                .uri()
            .path()
                .segment("/")
                .uri()
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("//user@host:80/")).unwrap());
    }

    #[test]
    fn parse_random_example_14() {
        let expected = Uri::new()
            .path()
                .segment("path")
                .segment("with:special;characters")
                .uri()
            .query("query=value&another=value%escaped")
            .fragment("part")
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("path/with:special;characters?query=value&another=value%25escaped#part")).unwrap());
    }

    #[test]
    fn parse_random_example_15() {
        let expected = Uri::new()
            .scheme("ldap")
            .authority()
                .host("[2001:db8::7]")
                .uri()
            .path()
                .segment("/c=GB")
                .uri()
            .query("objectClass?one")
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("ldap://[2001:db8::7]/c=GB?objectClass?one")).unwrap());
    }

    #[test]
    fn parse_random_example_16() {
        let expected = Uri::new()
            .scheme("sip")
            .path()
                .segment("test@example.com;transport=udp")
                .uri()
            .build();

        assert_eq!(expected, Uri::parse(generate_bytes("sip:test@example.com;transport=udp")).unwrap());
    }
}