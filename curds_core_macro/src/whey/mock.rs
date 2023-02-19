use super::*;

pub struct WheyMock {
    mocked_trait: ItemTrait,
    core: WheyMockCore,
}

impl Parse for WheyMock {
    fn parse(input: ParseStream) -> Result<Self> {
        let mocked_trait: ItemTrait = input.parse()?;
        let core = WheyMockCore::new(mocked_trait.clone());
        
        Ok(WheyMock {
            mocked_trait,
            core,
        })
    }
}

impl WheyMock {
    pub fn filter_items(item: &TraitItem) -> Option<&TraitItemMethod> {
        match item {
            TraitItem::Method(method) => Some(method),
            _ => None,
        }
    }
    pub fn input_compare_ident(ident: &Ident) -> Ident { format_ident!("input_compare_{}", ident) }
    pub fn call_count_ident(ident: &Ident) -> Ident { format_ident!("call_count_{}", ident) }
    pub fn dummy_ident(ident: &Ident) -> Ident { format_ident!("dummy_{}", ident) }
    pub fn assert_ident(ident: &Ident) -> Ident { format_ident!("assert_{}", ident) }

    pub fn quote(self) -> TokenStream {
        let mocked_trait = self.mocked_trait;
        let whey_mock = Self::quote_mocked_trait(&mocked_trait);
        let core = self.core.quote();

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
        let (impl_generics, type_generics, where_clause) = mocked_trait.generics.split_for_impl();

        let mocked_items: Vec<&TraitItemMethod> = mocked_trait.items
            .iter()
            .filter_map(|item| Self::filter_items(item))
            .collect();
        let assert_methods: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| Self::quote_assert(item))
            .collect();
        let mocked_impls: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| Self::quote_impl(item))
            .collect();

        quote! {
            #[injected]
            #[cfg(test)]
            #vis struct #whey_name #generics {
                core: std::rc::Rc<#core_name>,
            }

            impl #impl_generics #whey_name #type_generics #where_clause {
                pub fn assert(&self) {
                    self.core.assert();
                }
                #(#assert_methods)*
            }

            #[cfg(test)]
            impl #base_name for #whey_name {
                #(#mocked_impls)*
            }
        }
    }
    fn quote_assert(method: &TraitItemMethod) -> TokenStream {
        let assert_ident = Self::assert_ident(&method.sig.ident);

        quote! {
            pub fn #assert_ident(&self, expected_calls: u32) {
                self.core.#assert_ident(expected_calls);
            }
        }
    }
    fn quote_impl(method: &TraitItemMethod) -> TokenStream {
        let signature = &method.sig;
        let input_compare_ident = Self::input_compare_ident(&method.sig.ident);
        let mut input_signature: Vec<&Box<Pat>> = Vec::new();
        for input in &method.sig.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => input_signature.push(&ty.pat),
            }
        }
        let call_count_ident = Self::call_count_ident(&method.sig.ident);
        let dummy_ident = Self::dummy_ident(&method.sig.ident);

        quote! {
            #signature {
                self.core.#input_compare_ident(#(#input_signature),*);
                self.core.#call_count_ident();
                self.core.#dummy_ident()
            }
        }
    }
}
