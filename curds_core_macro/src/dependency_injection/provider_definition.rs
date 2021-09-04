use super::*;

#[derive(Clone)]
pub struct ProviderDefinition {
    abstraction: Option<Ident>,
    provider: Ident,
    singleton_identifier: String,
}

impl Parse for ProviderDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Option<Token![dyn]>>()?;
        let requested: Ident = input.parse()?;
        input.parse::<Option<Token![<-]>>()?;
        let provider: Option<Ident> = input.parse()?;

        let random_bytes = rand::thread_rng().gen::<[u8; 8]>();
        let mut singleton_identifier = String::new();
        for byte in random_bytes {
            singleton_identifier.push_str(&format!("{:}", byte));
        }

        Ok(match provider {
            Some(provider_name) => Self {
                abstraction: Some(requested),
                provider: provider_name,
                singleton_identifier: singleton_identifier,
            },
            None => Self {
                abstraction: None,
                provider: requested,
                singleton_identifier: singleton_identifier,
            }
        })
    }
}

impl ProviderDefinition {
    pub fn singleton_dependency(self, definition: &StructDefinition) -> InjectedDependency {
        let field_name = format!("{}{}", SINGLETON_FIELD_PREFIX, self.singleton_identifier);
        let singleton_type = match self.abstraction {
            Some(abstraction) => quote! { std::cell::RefCell<std::option::Option<std::rc::Rc<dyn #abstraction>>> },
            None => {
                let provider_type = definition.dependency_type(&self.provider);
                quote! { std::cell::RefCell<std::option::Option<#provider_type>> }
            },
        };

        InjectedDependency {
            visibility: Visibility::Inherited,
            name: Ident::new(&field_name, Span::call_site()),
            ty: singleton_type,
            default: true,
        }
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
        let singleton_ident = Ident::new(&format!("{}{}", SINGLETON_FIELD_PREFIX, self.singleton_identifier), Span::call_site());

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