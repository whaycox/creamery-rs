use super::*;

#[derive(Clone, Debug)]
pub struct SerialRoute {
    pub stages: Vec<SerialStage>
}

impl SerialRoute {
    pub fn new(stages: Vec<SerialStage>) -> Self {
        Self {
            stages: stages,
        }
    }

    pub fn parse_explicit(input: ParseStream) -> Result<Self> {
        let pipeline_content;
        braced!(pipeline_content in input);
        let stages: Punctuated<SerialStage, Token![,]> = pipeline_content.parse_terminated(SerialStage::parse)?;
        Ok(Self {
            stages: stages
                .into_iter()
                .collect(),
        })
    }
    pub fn explicit_return(&self) -> Option<Type> {
        if self.stages.len() > 0 {
            let last = &self.stages[self.stages.len() - 1];
            last.output()
        }
        else {
            None
        }
    }
}