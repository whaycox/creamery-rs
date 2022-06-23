use super::*;

pub struct WheyMock {
    mocked_trait: ItemTrait,
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

        let mocked_items: Vec<&TraitItem> = mocked_trait.items
            .iter()
            .filter(|item| Self::filter_items(item))
            .collect();
        let mocked_setups: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| Self::quote_setup(item))
            .collect();
        let mocked_impls: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| Self::quote_impl(item))
            .collect();

        quote! {
            #[injected]
            #vis struct #whey_name #generics {
                #(#mocked_setups),*
            }

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
    fn quote_setup(item: &TraitItem) -> TokenStream {
        match item {
            TraitItem::Method(method) => {
                let ident = format_ident!("{}_setup", method.sig.ident);
                let inputs: Vec<TokenStream> = method.sig.inputs
                    .iter()
                    .filter_map(|input| {
                        match input {
                            FnArg::Typed(arg) => {
                                let ty = &arg.ty;
                                Some(quote! { #ty })
                            },
                            _ => None,
                        }
                    })
                    .collect();
                let output = match &method.sig.output {
                    ReturnType::Type(_, ty) => quote! { #ty },
                    _ => panic!("Unexpected output: {:?}", method.sig.output),
                };

                quote! {
                    #[defaulted]
                    #ident: curds_core_abstraction::whey::WheySetup<(#(#inputs),*), #output>
                }
            },
            _ => panic!("Unexpected trait item: {:?}", item),
        }
    }
    fn quote_impl(item: &TraitItem) -> TokenStream {
        match item {
            TraitItem::Method(method) => {
                let sig = &method.sig;
                let ident = format_ident!("{}_setup", method.sig.ident);
                let inputs: Vec<TokenStream> = method.sig.inputs
                    .iter()
                    .filter_map(|input| {
                        match input {
                            FnArg::Typed(arg) => {
                                let pat = &arg.pat;
                                Some(quote! { #pat })
                            },
                            _ => None,
                        }
                    })
                    .collect();

                quote! {
                    #sig {
                        self.#ident.consume((#(#inputs),*))
                    }
                }
            },
            _ => panic!("Unexpected trait item: {:?}", item),
        }
    }
}

impl Parse for WheyMock {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(WheyMock {
            mocked_trait: input.parse()?,
        })
    }
}
