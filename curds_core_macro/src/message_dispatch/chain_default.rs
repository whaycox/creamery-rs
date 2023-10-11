use super::*;

pub struct ChainDefault {
    stages: Vec<ChainStage>,
}

impl Parse for ChainDefault {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed_stages: Punctuated<ChainStage, Token![,]> = input.parse_terminated(ChainStage::parse)?;

        Ok(Self {
            stages: parsed_stages.into_iter().collect(),
        })
    }
}

impl ChainDefault {
    pub fn stages(&self) -> Vec<ChainStage> { 
        self.stages.clone()
    }
}