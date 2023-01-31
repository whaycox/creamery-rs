use super::*;

pub struct WheyMock {
    mocked_trait: ItemTrait,
}

impl Parse for WheyMock {
    fn parse(input: ParseStream) -> Result<Self> {
        let item: ItemTrait = input.parse()?;
        
        Ok(WheyMock {
            mocked_trait: item,
        })
    }
}

impl WheyMock {
    pub fn quote(self) -> TokenStream {
        let mocked_trait = self.mocked_trait;
        let whey_mock = Self::quote_trait(&mocked_trait);

        quote! {
            #mocked_trait
            #whey_mock
        }
    }
    fn quote_trait(mocked_trait: &ItemTrait) -> TokenStream {
        let vis = &mocked_trait.vis;
        let base_name = &mocked_trait.ident;
        let whey_name = format_ident!("Whey{}", mocked_trait.ident);
        let generics = &mocked_trait.generics;
        let (impl_generics, type_generics, where_clause) = mocked_trait.generics.split_for_impl();

        let mocked_items: Vec<&TraitItem> = mocked_trait.items
            .iter()
            .filter(|item| Self::filter_items(item))
            .collect();
        let call_counts: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| Self::quote_call_count(item))
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
                #(#call_counts),*
            }

            impl #impl_generics #whey_name #type_generics #where_clause {
                #(#assert_methods)*
            }

            #[cfg(test)]
            impl #base_name for #whey_name {
                #(#mocked_impls)*
            }
        }
    }
    fn filter_items(item: &TraitItem) -> bool {
        match item {
            TraitItem::Method(_) => true,
            _ => false,
        }
    }
    fn call_count_ident(ident: &Ident) -> Ident { format_ident!("call_count_{}", ident) }
    fn quote_call_count(item: &TraitItem) -> TokenStream {
        match item {
            TraitItem::Method(method) => {
                let call_count_ident = Self::call_count_ident(&method.sig.ident);

                quote! {
                    #[defaulted]
                    #call_count_ident: Cell<u32>
                }
            },
            _ => panic!("Unexpected trait item: {:?}", item),
        }
    }
    fn quote_assert(item: &TraitItem) -> TokenStream {
        match item {
            TraitItem::Method(method) => {
                let assert_ident = format_ident!("assert_{}", method.sig.ident);
                let call_count_ident = Self::call_count_ident(&method.sig.ident);

                quote! {
                    pub fn #assert_ident(&self, expected_calls: u32) {
                        assert_eq!(expected_calls, self.#call_count_ident.get())
                    }
                }
            },
            _ => panic!("Unexpected trait item: {:?}", item),
        }
    }
    fn quote_impl(item: &TraitItem) -> TokenStream {
        match item {
            TraitItem::Method(method) => {
                let signature = &method.sig;
                let call_count_ident = Self::call_count_ident(&method.sig.ident);

                quote! {
                    #signature { 
                        self.#call_count_ident.set(self.#call_count_ident.get() + 1)
                    }
                }
            },
            _ => panic!("Unexpected trait item: {:?}", item),
        }
    }
}
