use super::*;

pub struct ForwardedDefinition {
    trait_production: bool,
    promoted: bool,
    requested: Type,
    intermediate: Option<Type>,
    provider: Ident,
}

impl Parse for ForwardedDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let trait_production: Option<Token![dyn]> = input.parse()?;
        let requested: Type = input.parse()?;
        let promoted: Option<Token![^]> = input.parse()?;
        if promoted.is_none() {
            input.parse::<Token![~]>()?;
        }
        let provider_fork = input.fork();
        let intermediate: Result<Type> = input.parse();
        if input.peek(Token![~]) {
            let intermediate_type = intermediate?;
            input.parse::<Token![~]>()?;
            let provider: Ident = input.parse()?;
            
            Ok(Self {
                trait_production: trait_production.is_some(),
                promoted: promoted.is_some(),
                requested: requested,
                intermediate: Some(intermediate_type),
                provider: provider,
            })
        }
        else {
            let provider: Ident = provider_fork.parse()?;

            Ok(Self {
                trait_production: trait_production.is_some(),
                promoted: promoted.is_some(),
                requested: requested,
                intermediate: None,
                provider: provider,
            })
        }
    }
}

impl ForwardedDefinition {
    pub fn trait_storage(&self) -> bool { self.intermediate.is_none() && self.trait_production }
    pub fn singleton_type(&self) -> Type { 
        match &self.intermediate {
            Some(intermediate) => intermediate.clone(),
            None => self.requested.clone(),
        }
     }

    pub fn quote_transient(&self, provider: &ServiceProviderDefinition) -> TokenStream {
        let requested = 
        if self.trait_production {
            let abstraction = &self.requested;
            quote! { std::boxed::Box<dyn #abstraction> }
        }
        else {
            self.requested.to_token_stream()
        };
        let intermediate = 
        match &self.intermediate {
            Some(concrete) => quote! { #concrete },
            None => requested.clone(),
        };
        let name = provider.name();
        let forwarded_provider = &self.provider;
        let (impl_generics, type_generics, where_clause) = provider.generics().split_for_impl();
        let mut generation = quote! { curds_core_abstraction::dependency_injection::ServiceGenerator::<#intermediate>::generate(&self.#forwarded_provider) };
        if self.trait_production && self.intermediate.is_some() {
            generation = quote! { std::boxed::Box::new(#generation) };
        }

        quote! {
            impl #impl_generics curds_core_abstraction::dependency_injection::ServiceGenerator<#requested> for #name #type_generics #where_clause {
                fn generate(&self) -> #requested {
                    #generation
                }
            }
        }
    }

    pub fn quote_singleton(&self, definition: &ServiceProviderDefinition) -> TokenStream {
        let mut stored: &Type;
        let requested = 
        if self.trait_production {
            let abstraction = &self.requested;
            stored = abstraction;
            quote! { dyn #abstraction }
        }
        else {
            stored = &self.requested;
            self.requested.to_token_stream()
        };
        let mut intermediate = 
        match &self.intermediate {
            Some(concrete) => {
                stored = concrete;
                quote! { #concrete }
            },
            None => requested.clone(),
        };
        if self.promoted {
            if self.intermediate.is_none() && self.trait_production {
                intermediate = quote! { std::boxed::Box<#intermediate> };
            }
        }
        else {
            intermediate = quote! { std::rc::Rc<#intermediate> };
        }
        let provider = &self.provider;
        let mut generation = quote! { curds_core_abstraction::dependency_injection::ServiceGenerator::<#intermediate>::generate(&self.#provider) };
        if self.promoted {
            if self.intermediate.is_none() && self.trait_production {
                generation = quote! { #generation.into() };
            }
            else {
                generation = quote! { std::rc::Rc::new(#generation) };
            }
        }
        let name = definition.name();
        let singleton_ident = definition.singleton(stored);

        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#requested>> for #name {
                fn generate(&self) -> std::rc::Rc<#requested> {
                    if self.#singleton_ident.borrow().is_none() {
                        self.#singleton_ident.replace(Some(#generation));
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