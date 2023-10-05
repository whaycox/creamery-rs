use super::*;

pub struct GeneratedDefinition {
    abstraction: Option<Type>,
    implementation: Type,
}

impl From<Type> for GeneratedDefinition {
    fn from(value: Type) -> Self {
        Self {
            abstraction: None,
            implementation: value,
        }
    }
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
    pub fn singleton_description(&self) -> SingletonDescription {
        SingletonDescription {
            requested: self.requested_type(),
            stored: self.stored_type(),
        }
    }
    fn requested_type(&self) -> Type {
        if self.abstraction.is_some() {
            self.abstraction.clone().unwrap()
        }
        else {
            self.implementation.clone()
        }
    }
    pub fn stored_type(&self) -> Type {
        syn::parse2(match &self.abstraction {
            Some(trait_type) => quote! {
                std::cell::RefCell<
                    std::option::Option<
                        std::rc::Rc<
                            std::sync::RwLock<
                                std::boxed::Box<dyn #trait_type>
                            >
                        >
                    >
                >
            },
            None => {
                let concrete_type = &self.implementation;
                quote! {
                    std::cell::RefCell<
                        std::option::Option<
                            std::rc::Rc<
                                std::sync::RwLock<
                                    #concrete_type
                                >
                            >
                        >
                    >
                }
            }
        }).unwrap()
    }

    pub fn quote_transient(&self, provider: &ServiceProviderDefinition) -> TokenStream {
        let producing_abstraction = self.abstraction.is_some();
        let implementation = &self.implementation;
        let requested = match &self.abstraction {
            Some(abstraction) => {
                let lifetimes = provider.lifetimes();
                quote! { std::boxed::Box<dyn #abstraction + #(#lifetimes)+*> }
            },
            None => implementation.to_token_stream(),
        };
        let name = provider.name();
        let (impl_generics, type_generics, where_clause) = provider.generics().split_for_impl();
        let mut generation = quote! { <#implementation as curds_core_abstraction::dependency_injection::Injected::<#name #type_generics>>::inject(self) };
        if producing_abstraction {
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
    
    pub fn quote_singleton(&self, provider: &ServiceProviderDefinition) -> TokenStream {
        let requested_key = match &self.abstraction {
            Some(abstraction) => abstraction,
            None => &self.implementation,
        };
        let mut requested = match &self.abstraction {
            Some(abstraction) => {
                let lifetimes = provider.lifetimes();
                quote! { std::boxed::Box<dyn #abstraction + #(#lifetimes)+*> }
            },
            None => self.implementation.to_token_stream(),
        };
        requested = quote! { std::rc::Rc<std::sync::RwLock<#requested>> };
        let name = provider.name();
        let (impl_generics, type_generics, where_clause) = provider.generics().split_for_impl();
        let singleton_ident = provider.singleton(requested_key);
        let implementation = &self.implementation;
        let mut generation = quote! { <#implementation as curds_core_abstraction::dependency_injection::Injected::<#name #type_generics>>::inject(self) };
        if self.abstraction.is_some() {
            generation = quote! { std::boxed::Box::new(#generation) };
        }
        generation = quote! { std::rc::Rc::new(std::sync::RwLock::new(#generation)) };

        quote! {
            impl #impl_generics curds_core_abstraction::dependency_injection::ServiceGenerator<#requested> for #name #type_generics #where_clause {
                fn generate(&self) -> #requested {
                    if self.#singleton_ident.borrow().is_none() {
                        let mut singleton = self.#singleton_ident.borrow_mut();
                        *singleton = Some(#generation);
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