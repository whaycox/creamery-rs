use super::*;

const MOCKED_RETURN_IDENTIFIER: &str = "mocked_return";

pub struct WheyMock {
    pub mocked_trait: ItemTrait,
    //pub mocked_returns: HashSet<Ident>,
}

impl Parse for WheyMock {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut mocked_trait: ItemTrait = input.parse()?;
        //let mocked_returns = Self::parse_mocked_returns(&mut mocked_trait)?;
        
        Ok(WheyMock {
            mocked_trait,
            //mocked_returns,
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

    fn parse_mocked_returns(mocked_trait: &mut ItemTrait) -> Result<HashSet<Ident>> {
        let mut types: HashSet<Ident> = HashSet::new();
        for item in &mut mocked_trait.items {
            match item {
                TraitItem::Method(method) => {
                    match &method.sig.output {
                        ReturnType::Type(_, _) => {
                            let length = method.attrs.len();
                            if length > 0 {
                                let mut attribute_index = 0;
                                while attribute_index < length {
                                    let attribute = &method.attrs[attribute_index];
                                    if attribute.path.is_ident(MOCKED_RETURN_IDENTIFIER) {
                                        let method_ident = method.sig.ident.clone();
                                        
                                        types.insert(method_ident);
                                        method.attrs.remove(attribute_index);
                                        break;
                                    }

                                    attribute_index = attribute_index + 1;
                                }
                            }
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        }

        Ok(types)
    }

    pub fn quote(self) -> TokenStream {
        let mocked_trait = &self.mocked_trait;
        //let whey_mock = Self::quote_mocked_trait(&mocked_trait);
        let core = self.core().quote();

        quote! {
            #mocked_trait
            //#whey_mock
            #core
        }
    }
    fn quote_mocked_trait(mocked_trait: &ItemTrait) -> TokenStream {
        let vis = &mocked_trait.vis;
        let base_name = &mocked_trait.ident;
        let whey_name = format_ident!("Whey{}", mocked_trait.ident);
        let core_name = format_ident!("WheyCore{}", mocked_trait.ident);
        let generics = &mocked_trait.generics;

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
                core: #core_name,
            }

            #[cfg(test)]
            impl #base_name for #whey_name {
                #(#mocked_impls)*
            }
        }
    }
    fn quote_impl(method: &TraitItemMethod) -> TokenStream {
        let signature = &method.sig;
        let consume_expectation = WheyMockCore::consume_expectation_ident(&method.sig.ident);
        let mut input_signature: Vec<&Box<Pat>> = Vec::new();
        for input in &method.sig.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => input_signature.push(&ty.pat),
            }
        }
        let mut input_values = quote! {};
        if input_signature.len() > 0 {
            input_values = quote! { (#(#input_signature),*) };
        }

        quote! {
            #signature {
                self.core.#consume_expectation(#input_values)
            }
        }
    }
}
