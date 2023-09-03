use super::*;

pub const MOCKS_IDENTIFIER: &str = "mocks";
pub const MOCKS_SINGLETON_IDENTIFIER: &str = "mocks_singleton";

pub struct WheyContext {
    item: ItemStruct,
    mocked_traits: MockedCollection,
}

impl Parse for WheyContext {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut item: ItemStruct = input.parse()?;
        let mocked_traits = Self::parse_mocks(&mut item)?;
        mocked_traits.add_fields(&mut item);

        Ok(WheyContext {
            item,
            mocked_traits,
        })
    }
}

impl WheyContext {
    fn parse_mocks(item: &mut ItemStruct) -> Result<MockedCollection> {
        let mut collection = MockedCollection::new();
        let length = item.attrs.len();
        if length > 0 {
            let mut attribute_index = length - 1;
            loop {
                let attribute = &item.attrs[attribute_index];
                if attribute.path.is_ident(MOCKS_IDENTIFIER) {
                    collection.add(WheyMockedTrait::Transient(attribute.parse_args()?));
                    item.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(MOCKS_SINGLETON_IDENTIFIER) {
                    collection.add(WheyMockedTrait::Singleton(attribute.parse_args()?));
                    item.attrs.remove(attribute_index);
                }
    
                if attribute_index == 0 {
                    break;
                }
                attribute_index = attribute_index - 1;
            }
        }

        Ok(collection)
    }

    pub fn quote(self, test_type: Option<Ident>) -> TokenStream {
        let item = &self.item;
        let context_ident = &item.ident;
        let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();
        let test_type_generator = match test_type {
            Some(test_ident) => quote! { #[generates(#test_ident)] },
            None => quote! {},
        };

        let mocked_traits = self.mocked_traits.quote_attributes();
        let core_references = self.mocked_traits.quote_core_reference(item);
        let core_generators = self.mocked_traits.quote_core_generators(item);

        quote! {
            #[service_provider]
            #(#mocked_traits)*
            #test_type_generator
            #item

            #[allow(non_snake_case)]
            impl #impl_generics #context_ident #type_generics #where_clause {
                #(#core_references)*
            }

            #(#core_generators)*
        }
    }
}