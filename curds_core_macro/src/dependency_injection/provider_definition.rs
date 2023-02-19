use super::*;

pub struct ProviderDefinition {
    provider: Ident,
}

impl Parse for ProviderDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let provider: Ident = input.parse()?;
        Ok(Self {
            provider: provider,
        })
        
    }
}

impl ProviderDefinition {
    pub fn quote_clone(&self, definition: &ServiceProviderDefinition) -> TokenStream {
        let provider = &self.provider;
        let name = definition.name();
        let provider_type = definition.provider(&provider);
        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<#provider_type> for #name {
                fn generate(&mut self) -> #provider_type {
                    self.#provider.clone()
                }
            }
        }
    }

    pub fn quote_scope(&self, definition: &ServiceProviderDefinition) -> TokenStream {
        let provider = &self.provider;
        let name = definition.name();
        let provider_type = definition.provider(&provider);
        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<#provider_type> for #name {
                fn generate(&mut self) -> #provider_type {
                    self.#provider.scope()
                }
            }
        }
    }
}