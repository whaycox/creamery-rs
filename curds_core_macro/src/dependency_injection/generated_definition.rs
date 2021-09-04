use super::*;

#[derive(Clone)]
pub struct GeneratedDefinition {
    abstraction: Option<Ident>,
    implementation: Ident,
    singleton_identifier: String,
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

        let random_bytes = rand::thread_rng().gen::<[u8; 8]>();
        let mut singleton_identifier = String::new();
        for byte in random_bytes {
            singleton_identifier.push_str(&format!("{:X}", byte));
        }

        Ok(match implementation {
            Some(ident) => Self {
                abstraction: Some(requested),
                implementation: ident,
                singleton_identifier: singleton_identifier,
            },
            None => Self {
                abstraction: None,
                implementation: requested,
                singleton_identifier: singleton_identifier,
            }
        })
    }
}

impl GeneratedDefinition {
    pub fn singleton_dependency(self) -> InjectedDependency {
        let field_name = format!("{}{}", SINGLETON_FIELD_PREFIX, self.singleton_identifier);
        let implementation = self.implementation;
        let singleton_type = quote! { std::cell::RefCell<std::option::Option<std::rc::Rc<#implementation>>> };

        InjectedDependency {
            visibility: Visibility::Inherited,
            name: Ident::new(&field_name, Span::call_site()),
            ty: singleton_type,
            default: true,
        }
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
        let singleton_ident = Ident::new(&format!("{}{}", SINGLETON_FIELD_PREFIX, self.singleton_identifier), Span::call_site());

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