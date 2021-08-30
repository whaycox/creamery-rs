use super::*;

pub struct InjectedImplementation {
    definition: DependencyDefinition,
}
impl InjectedImplementation {
    pub fn new(definition: DependencyDefinition) -> Self {
        Self {
            definition: definition,
        }
    }

    pub fn quote(self) -> TokenStream {
        let ident = self.definition.ident;
        let field_initializers = self.definition.fields
            .clone()
            .into_iter()
            .map(|field| {
                let ident = field.ident;
                let field_type = field.ty;
                quote! {
                    #ident: curds_core_abstraction::dependency_injection::ServiceGenerator::<#field_type>::generate(provider)
                }
            });
        let provider_constraints: Vec<Field> = self.definition.fields
            .clone()
            .into_iter()
            .collect();
        let has_constraints = provider_constraints.len() != 0;
        let constraint_tokens = if !has_constraints {
            quote! { std::any::Any }
        }
        else {
            let mapped_constraints = provider_constraints
                .into_iter()
                .map(|field| {
                    let field_type = field.ty;
                    quote! { curds_core_abstraction::dependency_injection::ServiceGenerator<#field_type> }
                });

            quote! { #(#mapped_constraints)+* }
        };
        let root_inject_tokens = if has_constraints {
            quote!{}
        }
        else {
            quote! {
                impl curds_core_abstraction::dependency_injection::RootInjected for #ident {
                    fn root_inject() -> Self {
                        Self {

                        }
                    }
                }
            }
        };


        quote! {
            impl<TProvider> curds_core_abstraction::dependency_injection::Injected<TProvider> for #ident
            where TProvider : #constraint_tokens {
                fn inject(provider: &TProvider) -> Self {
                    Self {
                        #(#field_initializers),*
                    }
                }
            }
            #root_inject_tokens
        }
    }
}