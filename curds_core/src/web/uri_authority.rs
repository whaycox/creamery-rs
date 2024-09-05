#[derive(Debug, Clone, PartialEq)]
pub struct UriAuthority {
    pub user_info: Option<String>,
    pub host: String,
    pub port: Option<u32>,
}