use super::*;

pub struct ServiceProviderDefinition {
    item: ItemStruct,
    library: Vec<ServiceProduction>,
    singletons: SingletonCollection,
}
impl Parse for ServiceProviderDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut item: ItemStruct = input.parse()?;
        let library = Self::parse_productions(&mut item)?;
        let mut singletons = SingletonCollection::new();
        singletons.register_singletons(&item, &library)?;
        singletons.add_singletons(&mut item)?;

        Ok(Self {
            item: item,
            library: library,
            singletons: singletons,
        })
    }
}

impl ServiceProviderDefinition {
    pub fn name(&self) -> &Ident { &self.item.ident }
    pub fn generics(&self) -> &Generics { &self.item.generics }
    pub fn singleton(&self, ty: &Type) -> Ident { self.singletons.singleton(ty) }
    pub fn provider(&self, name: &Ident) -> Type {
        for field in &self.item.fields {
            if field.ident.clone().unwrap().to_string() == name.to_string() {
                return field.ty.clone()
            }
        }
        panic!("no provider found");
    }

    fn parse_productions(item: &mut ItemStruct) -> Result<Vec<ServiceProduction>> {
        let mut parsed: Vec<ServiceProduction> = Vec::new();
        let length = item.attrs.len();
        if length > 0 {
            let mut attribute_index = length - 1;
            loop {
                let attribute = &item.attrs[attribute_index];
                if attribute.path.is_ident(GENERATES_IDENTIFIER) {
                    parsed.push(ServiceProduction::GenerateTransient(attribute.parse_args::<GeneratedDefinition>()?));
                    item.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(GENERATES_SINGLETON_IDENTIFIER) {
                    parsed.push(ServiceProduction::GenerateSingleton(attribute.parse_args::<GeneratedDefinition>()?));
                    item.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(CLONES_SELF_IDENTIFIER) {
                    parsed.push(ServiceProduction::CloneSelf());
                    item.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(CLONES_IDENTIFIER) {
                    parsed.push(ServiceProduction::Clone(attribute.parse_args::<ProviderDefinition>()?));
                    item.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(FORWARDS_IDENTIFIER) {
                    parsed.push(ServiceProduction::ForwardTransient(attribute.parse_args::<ForwardedDefinition>()?));
                    item.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(FORWARDS_SINGLETON_IDENTIFIER) {
                    parsed.push(ServiceProduction::ForwardSingleton(attribute.parse_args::<ForwardedDefinition>()?));
                    item.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(SCOPES_SELF_IDENTIFIER) {
                    parsed.push(ServiceProduction::ScopeSelf());
                    item.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(SCOPES_IDENTIFIER) {
                    parsed.push(ServiceProduction::ScopeTransient(attribute.parse_args::<ProviderDefinition>()?));
                    item.attrs.remove(attribute_index);
                }
    
                if attribute_index == 0 {
                    break;
                }
                attribute_index = attribute_index - 1;
            }
            parsed.reverse();
        }

        Ok(parsed)
    }

    pub fn quote(self) -> TokenStream {
        let scope_tokens = self.quote_scope();
        let mut library: Vec<TokenStream> = Vec::new();
        for production in &self.library {
            library.push(production.quote(&self));
        }
        let singleton_initializers = self.singletons.quote_initializer_attributes();
        let item = self.item;

        quote! {
            #[injected]
            #(#singleton_initializers)*
            #item
            //#scope_tokens
            #(#library)*
        }
    }
    fn quote_scope(&self) -> TokenStream {
        let name = self.name();        
        let initializer_tokens = self.scope_initializers();
        let (impl_generics, type_generics, where_clause) = self.item.generics.split_for_impl();
        let singleton_initializers = self.singletons.quote_initializers();

        quote! {
            impl #impl_generics curds_core_abstraction::dependency_injection::Scoped for #name #type_generics #where_clause {
                fn scope(&self) -> Self {
                    let mut constructed = Self {
                        #initializer_tokens
                    };
                    #(#singleton_initializers)*

                    constructed
                }
            }
        }
    }
    fn scope_initializers(&self) -> TokenStream {
        let mut initializer_tokens: Vec<TokenStream> = Vec::new();
        match &self.item.fields {
            Fields::Named(named) => {
                for field in &named.named {
                    let name = &field.ident.clone().unwrap();
                    if self.singletons.is_singleton(name) {
                        initializer_tokens.push(quote! { #name: std::default::Default::default() })
                    }
                    else {
                        initializer_tokens.push(quote! { #name: self.#name.clone() })
                    }
                }
            },
            _ => panic!("Only named fields are supported"),
        }

        quote! {
            #(#initializer_tokens),*
        }
    }
}