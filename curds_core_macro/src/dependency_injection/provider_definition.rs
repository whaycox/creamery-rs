use super::*;

#[derive(Clone)]
pub struct ProviderDefinition {
    abstraction: Option<Ident>,
    provider: Ident,
    pub singleton: SingletonIdentifier,
}

impl Parse for ProviderDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Option<Token![dyn]>>()?;
        let requested: Ident = input.parse()?;
        input.parse::<Option<Token![<-]>>()?;
        let provider: Option<Ident> = input.parse()?;
        let singleton = SingletonIdentifier::new();

        Ok(match provider {
            Some(provider_name) => Self {
                abstraction: Some(requested),
                provider: provider_name,
                singleton: singleton,
            },
            None => Self {
                abstraction: None,
                provider: requested,
                singleton: singleton,
            }
        })
    }
}

impl ProviderDefinition {
    pub fn provider_name(&self) -> String {
        self.provider.to_string()
    }
    pub fn set_singleton_identifier(self, ident: &SingletonIdentifier) -> Self {
        Self {
            abstraction: self.abstraction,
            provider: self.provider,
            singleton: ident.clone(),
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