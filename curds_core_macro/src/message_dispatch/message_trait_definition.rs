use super::*;

pub struct MessageTraitDefinition {
    pub name: Ident,
    pub error_type: Type,
}

impl Parse for MessageTraitDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![!]>()?;
        let error_type: Type = input.parse()?;

        Ok(Self {
            name,
            error_type,
        })
    }
}
