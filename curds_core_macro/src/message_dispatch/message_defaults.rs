use super::*;

pub struct MessageDefaults {
    pub pipeline: Option<PipelineDefault>,
    pub chain: Option<ChainDefault>,
}

impl MessageDefaults {
    pub fn new() -> Self {
        Self {
            pipeline: None,
            chain: None,
        }
    }
} 