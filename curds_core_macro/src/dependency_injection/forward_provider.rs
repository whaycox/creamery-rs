use syn::token::For;

use super::*;

pub struct ForwardProviderDefinition {
    abstraction: Ident,
    provider: Ident,
}
impl ForwardProviderDefinition {
    pub fn quote(self, definition: &DependencyDefinition) -> TokenStream {
        let abstraction = self.abstraction;
        let provider = self.provider;
        let ident = definition.ident.clone();

        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<dyn #abstraction>> for #ident {
                fn generate(&self) -> std::rc::Rc<dyn #abstraction> {
                    curds_core_abstraction::dependency_injection::ServiceGenerator::<std::rc::Rc<dyn #abstraction>>::generate(&*self.#provider)
                }
            }
        }
    }
}

impl Parse for ForwardProviderDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let abstraction: Ident = input.parse()?;
        input.parse::<Token![<-]>()?;
        let provider: Ident = input.parse()?;

        Ok(ForwardProviderDefinition {
            abstraction: abstraction,
            provider: provider,
        })
    }
}