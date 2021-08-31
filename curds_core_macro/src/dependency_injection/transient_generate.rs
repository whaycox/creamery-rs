use super::*;

pub struct TransientGenerateDefinition {
    implementation: Ident,
}
impl TransientGenerateDefinition {
    pub fn quote(self, definition: &DependencyDefinition) -> TokenStream {
        let implementation = self.implementation;
        let ident = definition.ident.clone();

        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#implementation>> for #ident {
                fn generate(&self) -> std::rc::Rc<#implementation> {
                    std::rc::Rc::<#implementation>::new(curds_core_abstraction::dependency_injection::Injected::<#ident>::inject(self))
                }
            }
        }
    }
}

impl Parse for TransientGenerateDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(TransientGenerateDefinition {
            implementation: input.parse()?,
        })
    }
}