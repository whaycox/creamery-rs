use super::*;

#[derive(Clone)]
pub struct GeneratedDefinition {
    abstraction: Option<Ident>,
    implementation: Ident,
    pub singleton: SingletonIdentifier,
}

impl Parse for GeneratedDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let trait_production: Option<Token![dyn]> = input.parse()?;
        let requested: Ident = input.parse()?;
        input.parse::<Option<Token![<-]>>()?;
        let implementation: Option<Ident> = input.parse()?;
        if trait_production.is_some() && implementation.is_none() {
            return Err(Error::new(requested.span(), "a concrete type type must also be defined"));
        }
        let singleton = SingletonIdentifier::new();

        Ok(match implementation {
            Some(ident) => Self {
                abstraction: Some(requested),
                implementation: ident,
                singleton: singleton,
            },
            None => Self {
                abstraction: None,
                implementation: requested,
                singleton: singleton,
            }
        })
    }
}

impl GeneratedDefinition {
    pub fn singleton_type(&self) -> String {
        self.implementation.to_string()
    }
    pub fn set_singleton_identifier(self, ident: &SingletonIdentifier) -> Self {
        Self {
            abstraction: self.abstraction,
            implementation: self.implementation,
            singleton: ident.clone(),
        }
    }
    pub fn to_singleton_dependency(self) -> SingletonDependency {
        let implementation = self.implementation;
        SingletonDependency::new(self.singleton, quote! { std::rc::Rc<#implementation> })
    }

    pub fn transient(self, definition: &StructDefinition) -> TokenStream {
        let requested = match self.abstraction {
            Some(abstraction) => quote! { dyn #abstraction },
            None => self.implementation.to_token_stream(),
        };
        let implementation = self.implementation;
        let name = definition.name.clone();

        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#requested>> for #name {
                fn generate(&self) -> std::rc::Rc<#requested> {
                    <#implementation as curds_core_abstraction::dependency_injection::Injected::<#name>>::inject(self)
                }
            }
        }
    }

    pub fn singleton(self, definition: &StructDefinition) -> TokenStream {
        let requested = match self.abstraction {
            Some(abstraction) => quote! { dyn #abstraction },
            None => self.implementation.to_token_stream(),
        };
        let implementation = self.implementation;
        let name = definition.name.clone();
        let singleton_ident = self.singleton.ident();

        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#requested>> for #name {
                fn generate(&self) -> std::rc::Rc<#requested> {
                    if self.#singleton_ident.borrow().is_none() {
                        let service = <#implementation as curds_core_abstraction::dependency_injection::Injected::<#name>>::inject(self);
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