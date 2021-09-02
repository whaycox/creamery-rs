use syn::spanned::Spanned;

use super::*;

pub struct ForwardedDefinition {
    pub requested: Ident,
    pub provider_requested: Ident,
    provider: Ident,
    pub mapped: bool,
}

impl ForwardedDefinition {
    pub fn parse_transient(input: ParseStream) -> Result<Self> {
        let trait_production: Option<Token![dyn]> = input.parse()?;
        let requested: Ident = input.parse()?;
        input.parse::<Token![<-]>()?;
        let provider_requested: Ident = input.parse()?;
        input.parse::<Option<Token![<-]>>()?;
        let provider_ident: Option<Ident> = input.parse()?;
        
        Ok(match provider_ident {
            Some(provider_name) => {
                Self {
                    requested: requested,
                    provider_requested: provider_requested,
                    provider: provider_name,
                    mapped: trait_production.is_some(),
                }
            },
            None => Self {
                requested: requested.clone(),
                provider_requested: requested,
                provider: provider_requested,
                mapped: trait_production.is_some(),
            }
        })   
    }

    pub fn parse_singleton(input: ParseStream) -> Result<Self> {
        let trait_production: Option<Token![dyn]> = input.parse()?;
        let requested: Ident = input.parse()?;
        input.parse::<Token![<-]>()?;
        let provider_requested: Ident = input.parse()?;
        input.parse::<Option<Token![<-]>>()?;
        let provider_ident: Option<Ident> = input.parse()?;
        
        Ok(match provider_ident {
            Some(provider_name) => {
                if requested != provider_requested {
                    println!("Not Same");

                    Self {
                        requested: requested,
                        provider_requested: provider_requested,
                        provider: provider_name,
                        mapped: true,
                    }
                }
                else {
                    println!("Same");
                    if trait_production.is_some() {
                        return Err(Error::new(trait_production.span(), "'dyn' unexpected when generating a concrete type"))
                    }

                    Self {
                        requested: requested,
                        provider_requested: provider_requested,
                        provider: provider_name,
                        mapped: false,
                    }
                }
            },
            None => {
                if trait_production.is_some() {
                    return Err(Error::new(requested.span(), "a concrete type type must also be defined to store as singleton"))
                }

                Self {
                    requested: requested.clone(),
                    provider_requested: requested,
                    provider: provider_requested,
                    mapped: trait_production.is_some(),
                }
            }
        })   
    }
}

impl ForwardedDefinition {
    pub fn transient(self, definition: &StructDefinition) -> TokenStream {
        let requested = self.requested;
        let provider_requested = self.provider_requested;
        let provider = self.provider;
        let name = definition.name.clone();

        if self.mapped {
            quote! {
                impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<dyn #requested>> for #name {
                    fn generate(&self) -> std::rc::Rc<dyn #requested> {
                        curds_core_abstraction::dependency_injection::ServiceGenerator::<std::rc::Rc<dyn #provider_requested>>::generate(&*self.#provider)
                    }
                }
            }
        }
        else {
            quote! {
                impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#requested>> for #name {
                    fn generate(&self) -> std::rc::Rc<#requested> {
                        curds_core_abstraction::dependency_injection::ServiceGenerator::<std::rc::Rc<#provider_requested>>::generate(&*self.#provider)
                    }
                }
            }
        }
    }

    pub fn singleton(self, definition: &StructDefinition) -> TokenStream {
        let requested = self.requested;
        let provider_requested = self.provider_requested;
        let provider = self.provider;
        let name = definition.name.clone();
        let library_name = super::library_name();

        if self.mapped {
            quote! {
                impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<dyn #requested>> for #name {
                    fn generate(&self) -> std::rc::Rc<dyn #requested> {
                        let type_id = std::any::TypeId::of::<std::rc::Rc<#provider_requested>>();
                        if !self.#library_name.borrow().contains_key(&type_id) {
                            let service = curds_core_abstraction::dependency_injection::ServiceGenerator::<std::rc::Rc<#provider_requested>>::generate(&*self.#provider);

                            let mut service_update = self.#library_name.take();
                            service_update.insert(type_id, service);
                            self.#library_name.replace(service_update);
                        }
                        self.#library_name
                            .borrow()
                            .get(&type_id)
                            .unwrap()
                            .clone()
                            .downcast::<#provider_requested>()
                            .unwrap()
                    }
                }
            }
        }
        else {
            quote! {
                impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#requested>> for #name {
                    fn generate(&self) -> std::rc::Rc<#requested> {
                        let type_id = std::any::TypeId::of::<std::rc::Rc<#provider_requested>>();
                        if !self.#library_name.borrow().contains_key(&type_id) {
                            let service = curds_core_abstraction::dependency_injection::ServiceGenerator::<std::rc::Rc<#provider_requested>>::generate(&*self.#provider);
    
                            let mut service_update = self.#library_name.take();
                            service_update.insert(type_id, service);
                            self.#library_name.replace(service_update);
                        }
                        self.#library_name
                            .borrow()
                            .get(&type_id)
                            .unwrap()
                            .clone()
                            .downcast::<#provider_requested>()
                            .unwrap()                        
                    }
                }
            }
        }
    }
}