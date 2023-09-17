use super::*;

pub const DEFAULT_RETURN_IDENTIFIER: &str = "mock_default_return";

pub struct WheyMock {
    pub mocked_trait: ItemTrait,
    pub defaulted_returns: HashMap<Ident, TokenStream>,
}

impl Parse for WheyMock {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut mocked_trait: ItemTrait = input.parse()?;
        let defaulted_returns = Self::parse_defaulted(&mut mocked_trait)?;
        
        Ok(WheyMock {
            mocked_trait,
            defaulted_returns,
        })
    }
}

impl WheyMock {
    fn core(&self) -> WheyMockCore { WheyMockCore::new(&self) }
    
    pub fn filter_items(item: &TraitItem) -> Option<&TraitItemMethod> {
        match item {
            TraitItem::Method(method) => Some(method),
            _ => None,
        }
    }

    fn parse_defaulted(item: &mut ItemTrait) -> Result<HashMap<Ident, TokenStream>> {
        let mut default_return: HashMap<Ident, TokenStream> = HashMap::new();
        for method in &mut item.items {
            match method {
                TraitItem::Method(trait_method) => {
                    let length = trait_method.attrs.len();
                    if length > 0 {
                        let mut attribute_index = 0;
                        while attribute_index < length {
                            let attribute = &trait_method.attrs[attribute_index];
                            if attribute.path.is_ident(DEFAULT_RETURN_IDENTIFIER) {
                                let ident = trait_method.sig.ident.clone();
                                let mut default_value = quote! { Some(std::boxed::Box::new(|| std::default::Default::default())) };
                                if !attribute.tokens.is_empty() {
                                    let generator: ExprClosure = attribute.parse_args()?;
                                    default_value = quote! { Some(std::boxed::Box::new(#generator)) };
                                }
                                
                                default_return.insert(ident, default_value);
                                trait_method.attrs.remove(attribute_index);
                                break;
                            }

                            attribute_index = attribute_index + 1;
                        }
                    }
                },
                _ => panic!("Only named fields are supported"),
            }
        }

        Ok(default_return)
    }

    pub fn quote(self) -> TokenStream {
        let mocked_trait = &self.mocked_trait;
        let whey_mock = Self::quote_mocked_trait(&mocked_trait);
        let core = self.core().quote();

        quote! {
            #mocked_trait
            #whey_mock
            #core
        }
    }
    fn quote_mocked_trait(mocked_trait: &ItemTrait) -> TokenStream {
        let vis = &mocked_trait.vis;
        let base_name = &mocked_trait.ident;
        let whey_name = format_ident!("Whey{}", mocked_trait.ident);
        let core_name = format_ident!("WheyCore{}", mocked_trait.ident);
        let generics = &mocked_trait.generics;
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        let mocked_items: Vec<&TraitItemMethod> = mocked_trait.items
            .iter()
            .filter_map(|item| Self::filter_items(item))
            .collect();
        let mocked_impls: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| Self::quote_impl(item))
            .collect();

        quote! {
            #[injected]
            #[cfg(test)]
            #vis struct #whey_name #generics {
                core: std::rc::Rc<std::sync::RwLock<#core_name #generics>>,
            }

            #[cfg(test)]
            impl #impl_generics #base_name #generics for #whey_name #type_generics #where_clause {
                #(#mocked_impls)*
            }
        }
    }
    fn quote_impl(method: &TraitItemMethod) -> TokenStream {
        let signature = &method.sig;
        let mut input_names: Vec<&Box<Pat>> = Vec::new();
        for input in &method.sig.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => input_names.push(&ty.pat),
            }
        }

        let record_call = WheyMockCore::record_call(&method.sig.ident);

        let compare_input = if input_names.len() > 0 {
            let core_comparer = WheyMockCore::consume_expected_input(&method.sig.ident);
            quote! {
                core.#core_comparer(#(#input_names),*);
            }
        }
        else { quote! {} };
        let generate_return = match &method.sig.output {
            ReturnType::Default => quote! {},
            ReturnType::Type(_, _) => {
                let core_generator = WheyMockCore::generate_return(&method.sig.ident);
                quote! {
                    core.#core_generator(#(#input_names),*)
                }
            },
        };

        quote! {
            #signature {
                let mut core = self.core.write().unwrap();
                core.#record_call();

                #compare_input
                #generate_return
            }
        }
    }
}
