use syn::token::For;

use super::*;

pub struct ForwardProviderDefinition {
    abstraction: Ident,
    provider: Ident,
}
impl ForwardProviderDefinition {
    pub fn quote(self, dependency_definition: &DependencyDefinition) -> TokenStream {
        let abstraction = self.abstraction;
        let provider = self.provider;
        let ident = dependency_definition.ident.clone();
        let mut dependency_field: Option<Field> = None;
        for field in dependency_definition.fields.clone().into_iter() {
            if field.ident.clone().unwrap() == provider {
                dependency_field = Some(field);
                break;
            }
        }

        match dependency_field {
            Some(field) => {
                let provider_name = field.ident;
                quote! {
                    impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<dyn #abstraction>> for #ident {
                        fn generate(&self) -> std::rc::Rc<dyn #abstraction> {
                            curds_core_abstraction::dependency_injection::ServiceGenerator::<std::rc::Rc<dyn #abstraction>>::generate(&*self.#provider_name)
                        }
                    }
                }
            },
            None => {
                quote_spanned! {
                    provider.span() =>
                    compile_error!("field was not found");
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