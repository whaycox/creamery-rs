use super::*;

pub struct InjectedDefinition {
    definition: StructDefinition,
}
impl InjectedDefinition {
    pub fn quote(self) -> TokenStream {
        self.definition.quote()
    }
}

impl Parse for InjectedDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            definition: StructDefinition::parse(input, false)?,
        })
    }
}