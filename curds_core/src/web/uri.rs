use super::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Uri {
    pub scheme: Option<String>,
    pub authority: Option<UriAuthority>,
    pub path: UriPath,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

impl Uri {
    pub fn new() -> UriBuilder {
        UriBuilder::new()
    }
}