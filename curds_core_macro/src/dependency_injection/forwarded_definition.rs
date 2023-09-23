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
    pub fn is_promoted(&self) -> bool { self.promoted }
    pub fn singleton_description(&self) -> SingletonDescription {
        SingletonDescription {
            requested: self.requested_type(),
            stored: self.stored_type(),
        }
    }
    fn requested_type(&self) -> Type { self.requested.clone() }
    fn stored_type(&self) -> Type { 
        let requested = &self.requested;
        syn::parse2(if self.trait_production {
            quote! {
                std::option::Option<
                    std::rc::Rc<
                        std::sync::RwLock<
                            std::boxed::Box<dyn #requested>
                        >
                    >
                >
            }
        }
        else {
            quote! {
                std::option::Option<
                    std::rc::Rc<
                        std::sync::RwLock<
                            #requested
                        >
                    >
                >
            }
        }).unwrap()
    }

    pub fn quote_transient(&self, provider: &ServiceProviderDefinition) -> TokenStream {
        let requested = 
        if self.trait_production {
            let abstraction = &self.requested;
            let lifetimes = provider.lifetimes();
            quote! { std::boxed::Box<dyn #abstraction + #(#lifetimes)+*> }
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
        let mut generation = quote! { curds_core_abstraction::dependency_injection::ServiceGenerator::<#intermediate>::generate(&mut self.#forwarded_provider) };
        if self.trait_production && self.intermediate.is_some() {
            generation = quote! { std::boxed::Box::new(#generation) };
        }

        quote! {
            impl #impl_generics curds_core_abstraction::dependency_injection::ServiceGenerator<#requested> for #name #type_generics #where_clause {
                fn generate(&mut self) -> #requested {
                    #generation
                }
            }
        }
    }

    pub fn quote_singleton(&self, definition: &ServiceProviderDefinition) -> TokenStream {
        let mut requested = if self.trait_production {
            let abstraction = &self.requested;
            let lifetimes = definition.lifetimes();
            quote! { std::boxed::Box<dyn #abstraction + #(#lifetimes)+*> }
        }
        else {
            self.requested.to_token_stream()
        };
        let intermediate = match &self.intermediate {
            Some(concrete) => quote! { #concrete },
            None => requested.clone(),
        };
        requested = quote! { std::rc::Rc<std::sync::RwLock<#requested>> };
        let provider = &self.provider;
        let name = definition.name();
        let (impl_generics, type_generics, where_clause) = definition.generics().split_for_impl();
        let mut singleton_generation = quote! { self.#provider.generate() };
        if self.promoted {
            let singleton_ident = definition.singleton(&self.requested);

            let mut generation = quote! { curds_core_abstraction::dependency_injection::ServiceGenerator::<#intermediate>::generate(&mut self.#provider) };
            if self.trait_production && self.intermediate.is_some() {
                generation = quote! { std::boxed::Box::new(#generation) };
            }
            generation = quote! { std::rc::Rc::new(std::sync::RwLock::new(#generation)) };
            singleton_generation = quote! {
                if self.#singleton_ident.is_none() {
                    self.#singleton_ident = Some(#generation);
                }
                self.#singleton_ident
                    .as_ref()
                    .unwrap()
                    .clone()
            };
        }

        quote! {
            impl #impl_generics curds_core_abstraction::dependency_injection::ServiceGenerator<#requested> for #name #type_generics #where_clause {
                fn generate(&mut self) -> #requested {
                    #singleton_generation
                }
            }
        }
    }
}