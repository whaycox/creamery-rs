use super::*;

pub struct GeneratedDefinition {
    abstraction: Option<Type>,
    implementation: Type,
}

impl Parse for GeneratedDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let trait_production: Option<Token![dyn]> = input.parse()?;
        let requested: Type = input.parse()?;
        if input.peek(Token![~]) {
            input.parse::<Token![~]>()?;
            let implementation: Type = input.parse()?;
            Ok(Self {
                abstraction: Some(requested),
                implementation: implementation,
            })
        }
        else if trait_production.is_some() {
            Err(Error::new(requested.span(), "a concrete implementation type type must also be defined"))
        }
        else {
            Ok(Self {
                abstraction: None,
                implementation: requested,
            })
        }
    }
}

impl GeneratedDefinition {
    pub fn singleton_type(&self) -> Type { self.implementation.clone() }

    pub fn quote_transient(&self, provider: &ServiceProviderDefinition) -> TokenStream {
        let producing_abstraction = self.abstraction.is_some();
        let implementation = &self.implementation;
        let requested = match &self.abstraction {
            Some(abstraction) => quote! { std::rc::Rc<dyn #abstraction> },
            None => implementation.to_token_stream(),
        };
        let name = provider.name();
        let (impl_generics, type_generics, where_clause) = provider.generics().split_for_impl();
        let mut generation = quote! { <#implementation as curds_core_abstraction::dependency_injection::Injected::<#name #type_generics>>::inject(self) };
        if producing_abstraction {
            generation = quote! { std::rc::Rc::new(#generation) };
        }

        quote! {
            impl #impl_generics curds_core_abstraction::dependency_injection::ServiceGenerator<#requested> for #name #type_generics #where_clause {
                fn generate(&self) -> #requested {
                    #generation
                }
            }
        }
    }
    
    pub fn quote_singleton(&self, provider: &ServiceProviderDefinition) -> TokenStream {
        let mut requested = match &self.abstraction {
            Some(abstraction) => quote! { dyn #abstraction },
            None => self.implementation.to_token_stream(),
        };
        requested = quote! { std::rc::Rc<#requested> };
        let implementation = &self.implementation;
        let name = provider.name();
        let (impl_generics, type_generics, where_clause) = provider.generics().split_for_impl();
        let singleton_ident = provider.singleton(&self.implementation);

        quote! {
            impl #impl_generics curds_core_abstraction::dependency_injection::ServiceGenerator<#requested> for #name #type_generics #where_clause {
                fn generate(&self) -> #requested {
                    if self.#singleton_ident.borrow().is_none() {
                        let service = <#implementation as curds_core_abstraction::dependency_injection::Injected::<#name>>::inject(self);
                        self.#singleton_ident.replace(Some(std::rc::Rc::new(service)));
                    }
                    self.#singleton_ident
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .clone()
                }
            }
        }
    }
}