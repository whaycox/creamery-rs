use super::*;

#[derive(Clone)]
pub struct PipelineStage {
    pub name: Ident,
    pub return_type: StageReturn,
}

impl Default for PipelineStage {
    fn default() -> Self {
        Self { 
            name: Ident::new(HANDLER_NAME, Span::call_site()), 
            return_type: StageReturn::None,
        }
    }
}
impl From<Type> for PipelineStage {
    fn from(value: Type) -> Self {
        Self {
            name: Ident::new(HANDLER_NAME, Span::call_site()),
            return_type: StageReturn::Explicit(value),
        }
    }
}

impl Parse for PipelineStage {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            let return_type: StageReturn = input.parse()?;
            Ok(Self {
                name,
                return_type,
            })
        }
        else {
            Ok(Self {
                name,
                return_type: StageReturn::None,
            })
        }
    }
}

impl PipelineStage {
    pub fn replace_return(self, return_type: &StageReturn) -> Self {
        Self {
            name: self.name,
            return_type: return_type.clone(),
        }
    }

    pub fn trait_name(&self, base_name: &Ident) -> Ident { format_ident!("{}{}", base_name, self.name) }
    pub fn return_type(&self) -> &StageReturn { &self.return_type }
}