use super::*;

const HANDLER_NAME: &str = "Handler";

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

    pub fn default_message() -> Self {
        Self {
            name: Ident::new(HANDLER_NAME, Span::call_site()),
            return_type: None,
        }
    }
    pub fn default_request(return_type: Type) -> Self {
        Self {
            name: Ident::new(HANDLER_NAME, Span::call_site()),
            return_type: Some(return_type),
        }
    }
}