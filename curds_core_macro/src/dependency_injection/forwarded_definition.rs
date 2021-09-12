use super::*;

#[derive(Clone)]
pub struct ForwardedDefinition {
    requested: Ident,
    intermediate: Option<Ident>,
    provider: Ident,
    mapped: bool,
    pub singleton: SingletonIdentifier,
}

impl Parse for ForwardedDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let trait_production: Option<Token![dyn]> = input.parse()?;
        let requested: Ident = input.parse()?;
        input.parse::<Token![<-]>()?;
        let intermediate: Ident = input.parse()?;
        input.parse::<Option<Token![<-]>>()?;
        let provider_ident: Option<Ident> = input.parse()?;
        let singleton = SingletonIdentifier::new();
        
        Ok(match provider_ident {
            Some(provider_name) => {
                Self {
                    requested: requested,
                    intermediate: Some(intermediate),
                    provider: provider_name,
                    mapped: trait_production.is_some(),
                    singleton: singleton,
                }
            },
            None => {
                Self {
                    requested: requested,
                    intermediate: None,
                    provider: intermediate,
                    mapped: trait_production.is_some(),
                    singleton: singleton,
                }
            }
        })
    }
}

impl ForwardedDefinition {
    pub fn singleton_type(&self) -> String {
        match &self.intermediate {
            Some(intermediate) => intermediate.to_string(),
            None => self.requested.to_string(),
        }
    }
    pub fn set_singleton_identifier(self, ident: &SingletonIdentifier) -> Self {
        Self {
            requested: self.requested,
            intermediate: self.intermediate,
            provider: self.provider,
            mapped: self.mapped,
            singleton: ident.clone(),
        }
    }
    pub fn to_singleton_dependency(self) -> SingletonDependency {
        match &self.intermediate {
            Some(intermediate) => SingletonDependency::new(self.singleton, quote! { std::rc::Rc<#intermediate> }),
            None => {
                let requested = self.requested;
                if self.mapped {
                    SingletonDependency::new(self.singleton, quote! { std::rc::Rc<dyn #requested> })
                }
                else {
                    SingletonDependency::new(self.singleton, quote! { std::rc::Rc<#requested> })
                }
            },
        }        
    }

    pub fn transient(self, definition: &StructDefinition) -> TokenStream {
        let requested = 
        if self.mapped {
            let abstraction = self.requested;
            quote! { dyn #abstraction }
        }
        else {
            self.requested.to_token_stream()
        };
        let intermediate = 
        match self.intermediate {
            Some(concrete) => quote! { #concrete },
            None => requested.clone(),
        };
        let provider = self.provider;
        let name = definition.name.clone();

        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#requested>> for #name {
                fn generate(&self) -> std::rc::Rc<#requested> {
                    curds_core_abstraction::dependency_injection::ServiceGenerator::<std::rc::Rc<#intermediate>>::generate(&self.#provider)
                }
            }
        }
    }

    pub fn singleton(self, definition: &StructDefinition) -> TokenStream {
        let requested = 
        if self.mapped {
            let abstraction = self.requested;
            quote! { dyn #abstraction }
        }
        else {
            self.requested.to_token_stream()
        };
        let intermediate = 
        match self.intermediate {
            Some(concrete) => quote! { #concrete },
            None => requested.clone(),
        };
        let provider = self.provider;
        let name = definition.name.clone();
        let singleton_ident = self.singleton.ident();

        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#requested>> for #name {
                fn generate(&self) -> std::rc::Rc<#requested> {
                    if self.#singleton_ident.borrow().is_none() {
                        let service = curds_core_abstraction::dependency_injection::ServiceGenerator::<std::rc::Rc<#intermediate>>::generate(&self.#provider);
                        self.#singleton_ident.replace(Some(service));
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