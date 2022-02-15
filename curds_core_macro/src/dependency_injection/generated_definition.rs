use super::*;

#[derive(Clone)]
pub struct GeneratedDefinition {
    abstraction: Option<Type>,
    implementation: Type,
    pub singleton: SingletonIdentifier,
}

impl Parse for GeneratedDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let singleton = SingletonIdentifier::new();
        let trait_production: Option<Token![dyn]> = input.parse()?;
        let requested: Type = input.parse()?;
        if input.peek(Token![~]) {
            input.parse::<Token![~]>()?;
            let implementation: Type = input.parse()?;
            Ok(Self {
                abstraction: Some(requested),
                implementation: implementation,
                singleton: singleton,
            })
        }
        else if trait_production.is_some() {
            Err(Error::new(requested.span(), "a concrete type type must also be defined"))
        }
        else {
            Ok(Self {
                abstraction: None,
                implementation: requested,
                singleton: singleton,
            })
        }
    }
}

impl GeneratedDefinition {
    pub fn new(generated_type: Type) -> Self {
        Self {
            abstraction: None,
            implementation: generated_type,
            singleton: SingletonIdentifier::new(),
        }
    }

    pub fn register(self, collection: &mut SingletonCollection) -> Self {
        match collection.register_type(self.implementation.clone(), self.singleton.clone()) {
            None => self,
            Some(replacement) => Self {
                abstraction: self.abstraction,
                implementation: self.implementation,
                singleton: replacement,
            }
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
        let (impl_generics, type_generics, where_clause) = definition.generics.split_for_impl();

        quote! {
            impl #impl_generics curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#requested>> for #name #type_generics #where_clause {
                fn generate(&self) -> std::rc::Rc<#requested> {
                    <#implementation as curds_core_abstraction::dependency_injection::Injected::<#name #type_generics>>::inject(self)
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
        let (impl_generics, type_generics, where_clause) = definition.generics.split_for_impl();
        let singleton_ident = self.singleton.ident();

        quote! {
            impl #impl_generics curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#requested>> for #name #type_generics #where_clause {
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