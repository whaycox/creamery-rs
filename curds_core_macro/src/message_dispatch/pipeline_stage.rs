use super::*;

pub struct PipelineStage {
    name: Ident,
    pub return_type: Option<Type>,
}

impl Default for PipelineStage {
    fn default() -> Self {
        Self { 
            name: Ident::new(HANDLER_NAME, Span::call_site()), 
            return_type: None,
        }
    }
}
impl From<Type> for PipelineStage {
    fn from(value: Type) -> Self {
        Self {
            name: Ident::new(HANDLER_NAME, Span::call_site()),
            return_type: Some(value),
        }
    }
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
    pub fn trait_name(&self, base_name: &Ident) -> Ident { format_ident!("{}{}", base_name, self.name) }
    pub fn return_type(&self) -> &Option<Type> { &self.return_type }
}