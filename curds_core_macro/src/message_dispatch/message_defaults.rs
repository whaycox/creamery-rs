use super::*;

pub struct MessageDefaults {
    pub pipeline: Option<PipelineDefault>,
}

impl MessageDefaults {
    pub fn new() -> Self {
        Self {
            pipeline: None,
        }
    }
} 