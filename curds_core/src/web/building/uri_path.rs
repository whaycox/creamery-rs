use super::*;

pub struct UriPathBuilder {
    root: Option<Box<UriBuilder>>,

    absolute: bool,
    segments: Vec<UriPathSegment>,
}

impl UriPathBuilder {
    pub fn new(builder: UriBuilder) -> Self {
        Self {
            root: Some(Box::new(builder)),

            absolute: false,
            segments: Vec::new(),
        }
    }

    pub fn existing(mut self, builder: UriBuilder) -> Self {
        self.root = Some(Box::new(builder));

        self
    }

    pub fn segment(mut self, mut segment: &str) -> Self {
        if self.segments.len() == 0 && &segment[0..1] == "/" {
            segment = &segment[1..segment.len()];
            self.absolute = true;
        }
        if segment.len() > 0 {
            self.segments.push(UriPathSegment::new(segment.to_owned()));
        }

        self
    }

    pub fn uri(mut self) -> UriBuilder {
        let mut root = *self.root.unwrap();
        self.root = None;
        root.from_path(self);

        root
    }

    pub fn build_path(self) -> UriPath {
        return UriPath {
            absolute: self.absolute,
            segments: self.segments,
        }
    }
}