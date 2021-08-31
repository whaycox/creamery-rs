use super::*;

pub struct CloneProviderDefinition {
    implementation: Ident,
    provider: Ident,
}
impl CloneProviderDefinition {
    pub fn quote(self, definition: &DependencyDefinition) -> TokenStream {
        let implementation = self.implementation;
        let provider = self.provider;
        let ident = definition.ident.clone();

        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#implementation>> for #ident {
                fn generate(&self) -> std::rc::Rc<#implementation> {
                    self.#provider.clone()
                }
            }
        }
    }
}

impl Parse for CloneProviderDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let implementation: Ident = input.parse()?;
        input.parse::<Token![<-]>()?;
        let provider: Ident = input.parse()?;

        Ok(CloneProviderDefinition {
            implementation: implementation,
            provider: provider,
        })
    }
}