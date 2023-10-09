use super::*;

pub struct ChainStage {
    pub name: Ident,
}

impl Default for ChainStage {
    fn default() -> Self {
        Self { 
            name: Ident::new(HANDLER_NAME, Span::call_site()),
        }
    }
}

impl Parse for ChainStage {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        Ok(Self {
            name,
        })
    }
}

impl ChainStage {
    pub fn trait_name(&self, base_name: &Ident) -> Ident { format_ident!("{}{}", base_name, self.name) }
}