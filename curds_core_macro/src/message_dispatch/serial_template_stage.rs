use super::*;

pub struct SerialTemplateStage {
    name: Ident,
    output: StageReturn,
}
impl SerialTemplateStage {
    pub fn new(name: Ident, output: StageReturn) -> Self {
        Self {
            name: name,
            output: output,
        }
    }

    pub fn message(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let mut output = StageReturn::None;
        if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            output = StageReturn::Explicit(input.parse()?);
        }
        
        Ok(Self {
            name: name,
            output: output,
        })
    }

    pub fn expand(&self, parameters: &RoutingParameters) -> SerialStage {
        let output_type: Option<Type> = match self.output.clone() {
            StageReturn::None => None,
            StageReturn::Explicit(response) => Some(response),
            StageReturn::Message | StageReturn::Request => Some(parameters.message_type.clone()),
            StageReturn::Response => parameters.response_type.clone(),
        };

        SerialStage::new(self.name.clone(), output_type)
    }
}