use super::*;

#[derive(Clone)]
pub struct PipelineStage {
    pub name: Ident,
    pub return_type: Option<Type>,
}

impl Parse for PipelineStage {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            let return_type: Type = input.parse()?;
            Ok(Self {
                name: name,
                return_type: Some(return_type),
            })
        }
        else {
            Ok(Self {
                name: name,
                return_type: None,
            })
        }
    }
}

impl PipelineStage {
    pub fn return_type(&self) -> Option<Type> { self.return_type.clone() }
}