use super::*;

pub struct UriBuilder {
    scheme: Option<String>,
    authority: Option<UriAuthorityBuilder>,
    path: Option<UriPathBuilder>,
    query: Option<String>,
    fragment: Option<String>,
}

impl UriBuilder {
    pub fn new() -> Self {
        Self {
            scheme: None,
            authority: None,
            path: None,
            query: None,
            fragment: None,
        }
    }

    pub fn from_authority(&mut self, authority: UriAuthorityBuilder) {
        self.authority = Some(authority);
    }

    pub fn from_path(&mut self, path: UriPathBuilder) {
        self.path = Some(path);
    }
    
    pub fn scheme(mut self, scheme: &str) -> Self {
        self.scheme = Some(scheme.to_owned());
        self
    }

    pub fn authority(mut self) -> UriAuthorityBuilder {
        if let Some(current) = self.authority {
            self.authority = None;
            return current.existing(self);
        }
        UriAuthorityBuilder::new(self)
    }

    pub fn path(mut self) -> UriPathBuilder {
        if let Some(current) = self.path {
            self.path = None;
            return current.existing(self);
        }
        UriPathBuilder::new(self)
    }

    pub fn query(mut self, query: &str) -> Self {
        self.query = Some(query.to_owned());
        self
    }

    pub fn fragment(mut self, fragment: &str) -> Self {
        self.fragment = Some(fragment.to_owned());
        self
    }

    pub fn build(self) -> Uri {
        let mut authority = None;
        if let Some(authority_builder) = self.authority {
            authority = authority_builder.build_authority();
        }

        let mut path = UriPath::default();
        if let Some(path_builder) = self.path {
            path = path_builder.build_path();
        }

        Uri {
            scheme: self.scheme,
            authority,
            path,
            query: self.query,
            fragment: self.fragment,
        }
    }
}