use super::*;

#[derive(Clone)]
pub struct ForwardedDefinition {
    requested: Ident,
    intermediate: Option<Ident>,
    provider: Ident,
    mapped: bool,
    singleton_identifier: String,
}

impl Parse for ForwardedDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let trait_production: Option<Token![dyn]> = input.parse()?;
        let requested: Ident = input.parse()?;
        input.parse::<Token![<-]>()?;
        let intermediate: Ident = input.parse()?;
        input.parse::<Option<Token![<-]>>()?;
        let provider_ident: Option<Ident> = input.parse()?;

        let random_bytes = rand::thread_rng().gen::<[u8; 8]>();
        let mut singleton_identifier = String::new();
        for byte in random_bytes {
            singleton_identifier.push_str(&format!("{:X}", byte));
        }
        
        Ok(match provider_ident {
            Some(provider_name) => {
                Self {
                    requested: requested,
                    intermediate: Some(intermediate),
                    provider: provider_name,
                    mapped: trait_production.is_some(),
                    singleton_identifier: singleton_identifier,
                }
            },
            None => {
                Self {
                    requested: requested,
                    intermediate: None,
                    provider: intermediate,
                    mapped: trait_production.is_some(),
                    singleton_identifier: singleton_identifier,
                }
            }
        })
    }
}

impl ForwardedDefinition {
    pub fn singleton_dependency(self) -> InjectedDependency {
        let field_name = format!("{}{}", SINGLETON_FIELD_PREFIX, self.singleton_identifier);
        let singleton_type = match self.intermediate {
            Some(intermediate) => quote! { std::cell::RefCell<std::option::Option<std::rc::Rc<#intermediate>>> },
            None => if self.mapped {
                let abstraction = self.requested;
                quote! { std::cell::RefCell<std::option::Option<std::rc::Rc<dyn #abstraction>>> }
            }
            else {
                let implementation = self.requested;
                quote! { std::cell::RefCell<std::option::Option<std::rc::Rc<#implementation>>> }
            },
        };

        InjectedDependency {
            visibility: Visibility::Inherited,
            name: Ident::new(&field_name, Span::call_site()),
            ty: singleton_type,
            default: true,
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
                    curds_core_abstraction::dependency_injection::ServiceGenerator::<std::rc::Rc<#intermediate>>::generate(&*self.#provider)
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
        let singleton_ident = Ident::new(&format!("{}{}", SINGLETON_FIELD_PREFIX, self.singleton_identifier), Span::call_site());

        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#requested>> for #name {
                fn generate(&self) -> std::rc::Rc<#requested> {
                    if self.#singleton_ident.borrow().is_none() {
                        let service = curds_core_abstraction::dependency_injection::ServiceGenerator::<std::rc::Rc<#intermediate>>::generate(&*self.#provider);
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