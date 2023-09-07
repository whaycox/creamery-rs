use super::*;

pub struct WheyMock {
    pub mocked_trait: ItemTrait,
}

impl Parse for WheyMock {
    fn parse(input: ParseStream) -> Result<Self> {
        let mocked_trait: ItemTrait = input.parse()?;
        
        Ok(WheyMock {
            mocked_trait,
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
                core: std::rc::Rc<std::sync::RwLock<#core_name>>,
            }

            #[cfg(test)]
            impl #base_name for #whey_name {
                #(#mocked_impls)*
            }
        }
    }
    fn quote_impl(method: &TraitItemMethod) -> TokenStream {
        let signature = &method.sig;
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

        let record_call = WheyMockCore::record_call(&method.sig.ident);

        quote! {
            #signature {
                let mut core = self.core.write().unwrap();
                core.#record_call();
            }
        }
    }
}
