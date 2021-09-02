use super::*;

pub fn library_name() -> Ident { Ident::new(SERVICES_LIBRARY_NAME, Span::call_site()) }
const SERVICES_LIBRARY_NAME: &str = "_curds_core_services";

pub struct ServiceProviderDefinition {
    library: Vec<ServiceProduction>,
    definition: StructDefinition,
}
impl ServiceProviderDefinition {
    pub fn quote(self) -> TokenStream {
        let struct_tokens = self.definition.clone().quote();
        let definition = self.definition;
        let library = self.library
            .into_iter()
            .map(|production| production.quote(&definition));

        quote! {
            #struct_tokens
            #(#library)*
        }
    }
}

impl Parse for ServiceProviderDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let library = ServiceProduction::parse(&input.fork())?;
        let definition: StructDefinition = StructDefinition::parse(input, true)?;

        Ok(Self {
            library: library,
            definition: definition,
        })
    }
}