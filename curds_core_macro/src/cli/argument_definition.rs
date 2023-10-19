use super::*;

pub enum CliArgumentDefinition {
    Enumeration(CliArgumentEnumerationDefinition),
    Struct(CliArgumentStructDefinition),
}

impl Parse for CliArgumentDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(definition) = input.parse::<CliArgumentEnumerationDefinition>() {
            Ok(Self::Enumeration(definition))
        }
        else if let Ok(definition) = input.parse::<CliArgumentStructDefinition>() {
            Ok(Self::Struct(definition))
        }
        else {
            Err(syn::Error::new(Span::call_site(), "cli_arguments requires an enum or struct"))
        }
    }
}

impl CliArgumentDefinition {
    pub fn quote(self) -> TokenStream {
        match self {
            Self::Enumeration(definition) => definition.quote(),
            Self::Struct(definition) => definition.quote(),
        }
    }
}