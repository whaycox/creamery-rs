use super::*;

#[derive(Clone)]
pub struct ServiceProviderDefinition {
    library: Vec<ServiceProduction>,
    pub definition: StructDefinition,
    singletons: Vec<SingletonDependency>,
}
impl ServiceProviderDefinition {
    pub fn visibility(&self) -> Visibility { self.definition.visibility.clone() }
    pub fn name(&self) -> Ident { self.definition.name.clone() }

    pub fn generates(&mut self, generated_type: Type) {
        self.library.push(
            ServiceProduction::GenerateTransient(
                GeneratedDefinition::new(generated_type)))
    }

    pub fn quote(self) -> TokenStream {
        let struct_tokens = self.definition.clone().quote(self.singletons.clone());
        let definition = self.definition.clone();
        let scope_tokens = self.definition.scope_tokens(self.singletons);
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
        let library = ServiceProduction::parse_library(input)?;
        let definition: StructDefinition = StructDefinition::parse(input)?;
        let singletons: Vec<SingletonDependency> = ServiceProduction::singleton_fields(library.clone(), &definition);

        Ok(Self {
            library: library,
            definition: definition,
            singletons: singletons,
        })
    }
}