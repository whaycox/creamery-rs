use std::collections::HashSet;

use super::*;

pub struct WheyMock {
    pub mocked_trait: ItemTrait,
}

impl Parse for WheyMock {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            mocked_trait: input.parse()?,
        })
    }
}

impl WheyMock {
    fn filter_items(item: &TraitItem) -> Option<&TraitItemMethod> {
        match item {
            TraitItem::Method(method) => Some(method),
            _ => None,
        }
    }
    fn trait_lifetimes(item: &ItemTrait) -> HashSet<Ident> {
        let mut lifetimes: HashSet<Ident> = HashSet::new();
        for generic_param in item.generics.params.iter() {
            if let GenericParam::Lifetime(lifetime) = generic_param {
                lifetimes.insert(lifetime.lifetime.ident.clone());
            }
        }

        lifetimes
    }

    pub fn quote(self) -> TokenStream {
        let mocked_trait = &self.mocked_trait;
        let testing_mock = self.quote_testing_mock(&mocked_trait);

        quote! {
            #mocked_trait
            #testing_mock
        }
    }
    fn quote_testing_mock(&self, mocked_trait: &ItemTrait) -> TokenStream {
        let vis = &mocked_trait.vis;
        let base_name = &mocked_trait.ident;
        let testing_name = testing_struct_name(&mocked_trait.ident);
        let trait_lifetimes = Self::trait_lifetimes(&mocked_trait);
        let generics = &mocked_trait.generics;
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        let mocked_items: Vec<&TraitItemMethod> = mocked_trait.items
            .iter()
            .filter_map(|item| Self::filter_items(item))
            .collect();
        let fields: Vec<TokenStream> = mocked_items
            .iter()
            .flat_map(|item| quote_fields(item, &trait_lifetimes))
            .collect();
        let initializers: Vec<TokenStream> = mocked_items
            .iter()
            .flat_map(|item| quote_field_initializers(item))
            .collect();
        let mocked_impls: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| quote_impl(&testing_name, item))
            .collect();
        let setup_expectations: Vec<TokenStream> = mocked_items
            .iter()
            .flat_map(|item| quote_setup_expectations(item, &trait_lifetimes))
            .collect();        
        let assert_expectations: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| quote_assert_expectations(&testing_name, item))
            .collect();
        let reset_expectations: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| quote_reset_expectations(item))
            .collect();

        quote! {
            #vis struct #testing_name #generics {
                #(#fields),*
            }

            impl #impl_generics #testing_name #type_generics #where_clause {
                pub fn new() -> Self {
                    Self {
                        #(#initializers),*
                    }
                }

                #(#setup_expectations)*

                pub fn assert(&self) {
                    #(#assert_expectations)*
                    self.reset();
                }

                pub fn reset(&self) {
                    #(#reset_expectations)*
                }
            }

            impl #impl_generics Drop for #testing_name #type_generics #where_clause {
                fn drop(&mut self) {
                    if !std::thread::panicking() {
                        self.assert();
                    }
                }
            }

            impl #impl_generics #base_name #generics for #testing_name #type_generics #where_clause {
                #(#mocked_impls)*
            }
        }
    }
}
