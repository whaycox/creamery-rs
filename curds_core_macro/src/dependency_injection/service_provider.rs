use super::*;

pub const SINGLETON_FIELD_PREFIX: &str = "_curds_core_singleton_";

pub struct ServiceProviderDefinition {
    library: Vec<ServiceProduction>,
    definition: StructDefinition,
}
impl ServiceProviderDefinition {
    pub fn quote(self) -> TokenStream {
        let struct_tokens = self.definition.clone().quote();
        let definition = self.definition.clone();
        let scope_tokens = self.definition.scope_tokens();
        let library = self.library
            .into_iter()
            .map(|production| production.quote(&definition));
        
        quote! {
            #struct_tokens
            #scope_tokens
            #(#library)*
        }
    }
}

impl Parse for ServiceProviderDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let library = ServiceProduction::parse(&input.fork())?;
        let mut definition: StructDefinition = StructDefinition::parse(input)?;
        let singleton_fields: Vec<InjectedDependency> = library.clone()
            .into_iter()
            .filter_map(|production| {
                if production.is_singleton() {
                    Some(production.singleton_dependency(&definition))
                }
                else {
                    None
                }
            })
            .collect();
        definition.add_dependencies(singleton_fields);

        Ok(Self {
            library: library,
            definition: definition,
        })
    }
}