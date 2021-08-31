use super::*;

pub struct TransientMapDefinition {
    abstraction: Ident,
    implementation: Ident,
}
impl TransientMapDefinition {
    pub fn quote(self, definition: &DependencyDefinition) -> TokenStream {
        let abstraction = self.abstraction;
        let implementation = self.implementation;
        let ident = definition.ident.clone();

        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<dyn #abstraction>> for #ident {
                fn generate(&self) -> std::rc::Rc<dyn #abstraction> {
                    std::rc::Rc::<#implementation>::new(curds_core_abstraction::dependency_injection::Injected::<#ident>::inject(self))
                }
            }
        }
    }
}

impl Parse for TransientMapDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let abstraction: Ident = input.parse()?;
        input.parse::<Token![<-]>()?;
        let implementation: Ident = input.parse()?;

        Ok(TransientMapDefinition {
            abstraction: abstraction,
            implementation: implementation,
        })
    }
}