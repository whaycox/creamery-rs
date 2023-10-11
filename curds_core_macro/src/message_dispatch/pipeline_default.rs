use super::*;

pub struct PipelineDefault {
    stages: Vec<PipelineStage>,
}

impl Parse for PipelineDefault {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed_stages: Punctuated<PipelineStage, Token![,]> = input.parse_terminated(PipelineStage::parse)?;

        Ok(Self {
            stages: parsed_stages.into_iter().collect(),
        })
    }
}

impl PipelineDefault {
    pub fn stages(&self, pipeline_return: &StageReturn) -> Vec<PipelineStage> { 
        let mut stages = self.stages.clone();
        let last_stage = stages.pop().unwrap();
        stages.push(last_stage.replace_return(pipeline_return));

        stages
    }
}