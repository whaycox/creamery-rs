use super::*;

#[derive(Clone)]
pub struct ProviderDefinition {
    abstraction: Option<Type>,
    provider: Ident,
    pub singleton: SingletonIdentifier,
}

impl Parse for ProviderDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let singleton = SingletonIdentifier::new();
        let trait_production: Option<Token![dyn]> = input.parse()?;
        let provider_fork = input.fork();
        let requested: Result<Type> = input.parse();
        if input.peek(Token![~]) {
            let requested_type = requested?;
            input.parse::<Token![~]>()?;
            let provider: Ident = input.parse()?;

            Ok(Self {
                abstraction: Some(requested_type),
                provider: provider,
                singleton: singleton,
            })
        }
        else {
            let provider: Ident = provider_fork.parse()?;
            if trait_production.is_some() {
                return Err(Error::new(trait_production.span(), "unexpected token"))
            }

            Ok(Self {
                abstraction: None,
                provider: provider,
                singleton: singleton,
            })
        }
    }
}

impl ProviderDefinition {
    pub fn register(self, collection: &mut SingletonCollection) -> Self {
        match collection.register_provider(self.provider.clone(), self.singleton.clone()) {
            None => self,
            Some(replacement) => Self {
                abstraction: self.abstraction,
                provider: self.provider,
                singleton: replacement,
            }
        }
    }

    pub fn to_singleton_dependency(self, struct_definition: &StructDefinition) -> SingletonDependency {
        let ty = struct_definition.dependency_type(&self.provider);
        SingletonDependency::new(self.singleton, quote! { #ty })
    }

    pub fn clone_tokens(self, definition: &StructDefinition) -> TokenStream {
        let provider = self.provider;
        let name = definition.name.clone();
        match self.abstraction {
            Some(abstraction) => quote! {
                impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<dyn #abstraction>> for #name {
                    fn generate(&self) -> std::rc::Rc<dyn #abstraction> {
                        self.#provider.clone()
                    }
                }
            },
            None => {
                let provider_type = definition.dependency_type(&provider);
                quote! {
                    impl curds_core_abstraction::dependency_injection::ServiceGenerator<#provider_type> for #name {
                        fn generate(&self) -> #provider_type {
                            self.#provider.clone()
                        }
                    }
                }
            }
        }
    }

    pub fn scope_transient(self, definition: &StructDefinition) -> TokenStream {
        let provider = self.provider;
        let name = definition.name.clone();
        match self.abstraction {
            Some(abstraction) => quote! {
                impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<dyn #abstraction>> for #name {
                    fn generate(&self) -> std::rc::Rc<dyn #abstraction> {
                        self.#provider.scope()
                    }
                }
            },
            None => {
                let provider_type = definition.dependency_type(&provider);
                quote! {
                    impl curds_core_abstraction::dependency_injection::ServiceGenerator<#provider_type> for #name {
                        fn generate(&self) -> #provider_type {
                            self.#provider.scope()
                        }
                    }
                }
            }
        }
    }

    pub fn scope_singleton(self, definition: &StructDefinition) -> TokenStream {
        let provider = self.provider;
        let name = definition.name.clone();
        let provider_type = definition.dependency_type(&provider);
        let singleton_ident = self.singleton.ident();

        match self.abstraction {
            Some(abstraction) => quote! {
                impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<dyn #abstraction>> for #name {
                    fn generate(&self) -> std::rc::Rc<dyn #abstraction> {
                        if self.#singleton_ident.borrow().is_none() {
                            let service = self.#provider.scope();
                            self.#singleton_ident.replace(Some(service));
                        }
                        self.#singleton_ident
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .clone()
                    }
                }
            },
            None => {
                quote! {
                    impl curds_core_abstraction::dependency_injection::ServiceGenerator<#provider_type> for #name {
                        fn generate(&self) -> #provider_type {
                            if self.#singleton_ident.borrow().is_none() {
                                let service = self.#provider.scope();
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
    }
}