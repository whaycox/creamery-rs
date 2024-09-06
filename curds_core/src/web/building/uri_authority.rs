use super::*;

pub struct UriAuthorityBuilder {
    root: Option<Box<UriBuilder>>,

    user_info: Option<String>,
    host: Option<String>,
    port: Option<u32>,
}

impl UriAuthorityBuilder {
    pub fn new(builder: UriBuilder) -> Self {
        Self {
            root: Some(Box::new(builder)),

            user_info: None,
            host: None,
            port: None,
        }
    }

    pub fn existing(mut self, builder: UriBuilder) -> Self {
        self.root = Some(Box::new(builder));

        self
    }
    
    pub fn user_info(mut self, user_info: &str) -> UriAuthorityBuilder {
        self.user_info = Some(user_info.to_owned());
        self
    }
    
    pub fn host(mut self, host: &str) -> UriAuthorityBuilder {
        self.host = Some(host.to_owned());
        self
    }
    
    pub fn port(mut self, port: u32) -> UriAuthorityBuilder {
        self.port = Some(port);
        self
    }

    pub fn uri(mut self) -> UriBuilder {
        let mut root = *self.root.unwrap();
        self.root = None;
        root.from_authority(self);

        root
    }

    pub fn build_authority(self) -> Option<UriAuthority> {
        if let Some(host) = self.host {
            return Some(UriAuthority {
                user_info: self.user_info,
                host,
                port: self.port,
            })
        }
        None
    }
}