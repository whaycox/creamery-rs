use std::fmt::Display;

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct UriPath {
    pub absolute: bool,
    pub segments: Vec<UriPathSegment>,
}

impl UriPath {
    pub fn condense(self) -> Self {
        let mut condensed_segments = Vec::new();
        for segment in self.segments {
            match segment {
                UriPathSegment::Current => {},
                UriPathSegment::Parent => {
                    if !self.absolute && condensed_segments.len() == 0 {
                        condensed_segments.push(segment);
                    }
                    else {
                        condensed_segments.pop();
                    }
                },
                UriPathSegment::Named(_) => { condensed_segments.push(segment); },
            }
        }

        Self {
            absolute: self.absolute,
            segments: condensed_segments,
        }
    }
}

impl Display for UriPath {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.absolute {
            write!(formatter, "/")?;
        }

        let length = self.segments.len();
        for i in 0..length {
            write!(formatter, "{}", self.segments[i])?;
            if i != length - 1 {
                write!(formatter, "/")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum UriPathSegment {
    Named(String),
    Current,
    Parent,
}

impl UriPathSegment {
    pub fn new(segment: String) -> Self {
        match &*segment {
            "." => UriPathSegment::Current,
            ".." => UriPathSegment::Parent,
            _ => UriPathSegment::Named(segment),
        }
    }
}

impl Display for UriPathSegment {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UriPathSegment::Current => write!(formatter, "."),
            UriPathSegment::Parent => write!(formatter, ".."),
            UriPathSegment::Named(name) => write!(formatter, "{name}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_segment(id: u32) -> UriPathSegment { UriPathSegment::Named(format!("testing{id}").to_owned()) }

    #[test]
    fn condense_removes_current() {
        let test_path = UriPath {
            absolute: true,
            segments: vec![
                test_segment(1),
                UriPathSegment::Current,
                test_segment(2)
            ],
        };

        let expected = UriPath {
            absolute: true,
            segments: vec![
                test_segment(1),
                test_segment(2)
            ],
        };
        assert_eq!(expected, test_path.condense());
    }

    #[test]
    fn condense_removes_segment_before_parent() {
        let test_path = UriPath {
            absolute: true,
            segments: vec![
                test_segment(1),
                UriPathSegment::Parent,
                test_segment(2)
            ],
        };

        let expected = UriPath {
            absolute: true,
            segments: vec![test_segment(2)],
        };
        assert_eq!(expected, test_path.condense());
    }

    #[test]
    fn condense_doesnt_remove_parent_at_relative_start() {
        let test_path = UriPath {
            absolute: false,
            segments: vec![
                UriPathSegment::Parent,
                test_segment(1),
                test_segment(2)
            ],
        };

        let expected = UriPath {
            absolute: false,
            segments: vec![
                UriPathSegment::Parent,
                test_segment(1),
                test_segment(2)
            ],
        };
        assert_eq!(expected, test_path.condense());
    }
}