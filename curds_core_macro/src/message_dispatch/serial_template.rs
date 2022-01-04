use super::*;

pub struct SerialTemplate {
    stages: Vec<SerialTemplateStage>,
}
impl SerialTemplate {
    pub fn message() -> Self {
        Self {
            stages: vec![ SerialTemplateStage::new(Ident::new(HANDLER_NAME, Span::call_site()), StageReturn::None) ],
        }
    }
    pub fn request() -> Self {
        Self {
            stages: vec![ SerialTemplateStage::new(Ident::new(HANDLER_NAME, Span::call_site()), StageReturn::Response) ],
        }
    }

    pub fn parse_message(input: ParseStream) -> Result<Self> {
        let stages: Punctuated<SerialTemplateStage, Token![,]> = input.parse_terminated(SerialTemplateStage::message)?;
        Ok(Self {
            stages: stages
                .into_iter()
                .collect(),
        })
    }

    pub fn parse_request(input: ParseStream) -> Result<Self> {
        let stages: Punctuated<SerialTemplateStage, Token![,]> = input.parse_terminated(SerialTemplateStage::request)?;
        let mut parsed_stages: Vec<SerialTemplateStage> = stages
            .into_iter()
            .collect();
        let last = parsed_stages.pop();
        if last.is_some() {
            parsed_stages.push(last.unwrap().to_response());
        }
        Ok(Self {
            stages: parsed_stages,
        })
    }

    pub fn expand_with(&self, parameters: RoutingParameters) -> SerialRoute {
        let mut expanded_stages: Vec<SerialStage> = Vec::new();
        for stage in &self.stages {
            expanded_stages.push(stage.expand(&parameters))
        }
        SerialRoute::new(expanded_stages)
    }
}