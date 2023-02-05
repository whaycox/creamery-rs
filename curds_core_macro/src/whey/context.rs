use super::*;

pub const MOCKS_IDENTIFIER: &str = "mocks";
pub const MOCKS_SINGLETON_IDENTIFIER: &str = "mocks_singleton";

pub struct WheyContext {
    item: ItemStruct,
    mocked_traits: Vec<WheyMockedType>,
    mocked_singleton_traits: Vec<WheyMockedType>,
}

impl Parse for WheyContext {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut item: ItemStruct = input.parse()?;
        let mocked_traits = Self::parse_mocks(&mut item)?;
        let mocked_singleton_traits = Self::parse_singleton_mocks(&mut item)?;
        Ok(WheyContext {
            item,
            mocked_traits,
            mocked_singleton_traits,
        })
    }
}

impl WheyContext {
    fn parse_mocks(item: &mut ItemStruct) -> Result<Vec<WheyMockedType>> {
        let mut parsed: Vec<WheyMockedType> = Vec::new();
        let length = item.attrs.len();
        if length > 0 {
            let mut attribute_index = length - 1;
            loop {
                let attribute = &item.attrs[attribute_index];
                if attribute.path.is_ident(MOCKS_IDENTIFIER) {
                    parsed.push(attribute.parse_args()?);
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
    fn parse_singleton_mocks(item: &mut ItemStruct) -> Result<Vec<WheyMockedType>> {
        let mut parsed: Vec<WheyMockedType> = Vec::new();
        let length = item.attrs.len();
        if length > 0 {
            let mut attribute_index = length - 1;
            loop {
                let attribute = &item.attrs[attribute_index];
                if attribute.path.is_ident(MOCKS_SINGLETON_IDENTIFIER) {
                    parsed.push(attribute.parse_args()?);
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

    pub fn quote(self, context_type: Option<Ident>) -> TokenStream {
        let item = &self.item;
        let context_ident = &item.ident;
        let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

        let mocked_singletons: Vec<TokenStream> = self.mocked_singleton_traits
            .iter()
            .map(|singleton| {
                let mocked_trait = &singleton.mocked_trait;
                let whey_name = &singleton.whey_name;
                let core_name = &singleton.core_name;
                quote! { 
                    #[generates_singleton(#core_name)]
                    #[generates_singleton(dyn #mocked_trait ~ #whey_name)]
                    #[generates(#whey_name)]
                }
            })
            .collect();
        let mocked_traits: Vec<TokenStream> = self.mocked_traits
            .iter()
            .map(|mocked| {
                let mocked_trait = &mocked.mocked_trait;
                let whey_name = &mocked.whey_name;
                let core_name = &mocked.core_name;
                quote! { 
                    #[generates_singleton(#core_name)]
                    #[generates(dyn #mocked_trait ~ #whey_name)] 
                    #[generates(#whey_name)]
                }
            })
            .collect();

        let mocked_generators: Vec<TokenStream> = self.mocked_traits
            .iter()
            .map(|mocked| {
                let whey_name = &mocked.whey_name;

                quote! { 
                    impl #impl_generics curds_core_abstraction::whey::MockingContext<#whey_name> for #context_ident #type_generics #where_clause {
                        fn mocked(&self) -> #whey_name {
                            self.generate()
                        }
                    }
                }
            })
            .collect();
        let mocked_singleton_generators: Vec<TokenStream> = self.mocked_singleton_traits
            .iter()
            .map(|singleton| {
                let whey_name = &singleton.whey_name;

                quote! { 
                    impl #impl_generics curds_core_abstraction::whey::MockingContext<#whey_name> for #context_ident #type_generics #where_clause {
                        fn mocked(&self) -> #whey_name {
                            self.generate()
                        }
                    }
                }
            })
            .collect();
        let mocked_resets: Vec<TokenStream> = self.mocked_traits
            .iter()
            .map(|mocked| {
                let core_name = &mocked.core_name;

                quote! { 
                    <Self as curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#core_name>>>::generate(&self).reset();
                }
            })
            .collect();
        let mocked_singleton_resets: Vec<TokenStream> = self.mocked_singleton_traits
            .iter()
            .map(|singleton| {
                let core_name = &singleton.core_name;

                quote! { 
                    <Self as curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#core_name>>>::generate(&self).reset();
                }
            })
            .collect();

        quote! {
            #[service_provider]
            #(#mocked_singletons)*
            #(#mocked_traits)*
            #item

            #(#mocked_generators)*
            #(#mocked_singleton_generators)*

            impl #impl_generics curds_core_abstraction::whey::Whey for #context_ident #type_generics #where_clause {
                fn reset(&self) {
                    #(#mocked_resets)*
                    #(#mocked_singleton_resets)*
                }
            }
        }
    }
}