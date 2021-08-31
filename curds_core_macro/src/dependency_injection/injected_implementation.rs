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

    pub fn quote(self, defaults: Vec<DefaultedFields>) -> TokenStream {
        let ident = self.definition.ident;

        let default_fields: HashSet<Ident> = defaults
            .into_iter()
            .map(|field| field.ident)
            .collect();
        let required_fields: Vec<Field> = self.definition.fields
            .clone()
            .into_iter()
            .filter(|field| !default_fields.contains(&field.ident.clone().unwrap()))
            .collect();    

        let default_field_generators: Vec<TokenStream> = default_fields
            .into_iter()
            .map(|field|  {
                quote! {
                    #field: std::default::Default::default()
                }
            })
            .collect();
        let mut required_field_generators: Vec<TokenStream> = required_fields
            .clone()
            .into_iter()
            .map(|field| {
                let ident = field.ident.unwrap();
                let field_type = field.ty;

                quote! {
                    #ident: curds_core_abstraction::dependency_injection::ServiceGenerator::<#field_type>::generate(provider)
                }
            })
            .collect();
        let mut required_field_arguments: Vec<TokenStream> = required_fields
            .clone()
            .into_iter()
            .map(|field| {
                let ident = field.ident.unwrap();
                quote! {
                    #ident: #ident
                }
            })
            .collect();
        for field_tokens in default_field_generators.clone().into_iter() {
            required_field_generators.push(field_tokens.clone());
            required_field_arguments.push(field_tokens.clone());
        }

        let has_constraints = required_fields.len() != 0;
        let constraint_tokens = if !has_constraints {
            quote! { }
        }
        else {
            let mapped_constraints = required_fields
                .clone()
                .into_iter()
                .map(|field| {
                    let field_type = field.ty;
                    quote! { curds_core_abstraction::dependency_injection::ServiceGenerator<#field_type> }
                });

            quote! { where TProvider : #(#mapped_constraints)+* }
        };
        let argument_tokens = if !has_constraints {
            quote!{}
        }
        else {
            let mapped_arguments = required_fields.clone();
            quote! { #(#mapped_arguments),* }
        };

        quote! {
            impl<TProvider> curds_core_abstraction::dependency_injection::Injected<TProvider> for #ident
            #constraint_tokens {
                fn inject(provider: &TProvider) -> Self {
                    Self {
                        #(#required_field_generators),*
                    }
                }
            }
            
            impl #ident {
                fn construct(#argument_tokens) -> Self {
                    Self {
                        #(#required_field_arguments),*
                    }
                }
            }
        }
    }
}