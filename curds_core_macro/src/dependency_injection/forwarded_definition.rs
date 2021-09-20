use super::*;

#[derive(Clone)]
pub struct ForwardedDefinition {
    trait_production: bool,
    requested: Type,
    intermediate: Option<Type>,
    provider: Ident,
    pub singleton: SingletonIdentifier,
}

impl Parse for ForwardedDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let singleton = SingletonIdentifier::new();
        let trait_production: Option<Token![dyn]> = input.parse()?;
        let requested: Type = input.parse()?;
        input.parse::<Token![~]>()?;
        //CHECK FOR DYN TOKEN
        let provider_fork = input.fork();
        let intermediate: Result<Type> = input.parse();
        if input.peek(Token![~]) {
            let intermediate_type = intermediate?;
            input.parse::<Token![~]>()?;
            let provider: Ident = input.parse()?;
            
            Ok(Self {
                trait_production: trait_production.is_some(),
                requested: requested,
                intermediate: Some(intermediate_type),
                provider: provider,
                singleton: singleton,
            })
        }
        else {
            let provider: Ident = provider_fork.parse()?;

            Ok(Self {
                trait_production: trait_production.is_some(),
                requested: requested,
                intermediate: None,
                provider: provider,
                singleton: singleton,
            })
        }
    }
}

impl ForwardedDefinition {
    pub fn register(self, collection: &mut SingletonCollection) -> Self {
        let registered_type = match self.intermediate.clone()  {
            Some(intermediate) => intermediate,
            None => self.requested.clone(),
        };

        match collection.register_type(registered_type, self.singleton.clone()) {
            None => self,
            Some(replacement) => Self {
                trait_production: self.trait_production,
                requested: self.requested,
                intermediate: self.intermediate,
                provider: self.provider,
                singleton: replacement,
            }
        }
    }

    pub fn to_singleton_dependency(self) -> SingletonDependency {
        match &self.intermediate {
            Some(intermediate) => SingletonDependency::new(self.singleton, quote! { std::rc::Rc<#intermediate> }),
            None => {
                let requested = self.requested;
                if self.trait_production {
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
        if self.trait_production {
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
        if self.trait_production {
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