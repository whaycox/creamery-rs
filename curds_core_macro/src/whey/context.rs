use super::*;

pub const MOCKS_IDENTIFIER: &str = "mocks";
pub const MOCKS_SINGLETON_IDENTIFIER: &str = "mocks_singleton";
pub const MOCK_DEFAULT_RETURN: &str = "mock_default_return";
pub const MOCK_RETURN: &str = "mock_return";
pub const MOCK_INPUT: &str = "mock_input";
pub const MOCK_SEQUENCE: &str = "mock_sequence";

pub struct WheyContext {
    item: ItemStruct,
    mocked_traits: Vec<WheyMockedTrait>,
    default_returns: Vec<WheyContextDefaultReturn>,
    return_generators: Vec<WheyContextReturnGenerator>,
    input_comparisons: Vec<WheyContextInputComparison>,
    sequences: Vec<WheyContextSequence>
}

impl Parse for WheyContext {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut item: ItemStruct = input.parse()?;
        let mocked_traits = Self::parse_mocks(&mut item)?;
        let default_returns = Self::parse_default_returns(&mut item)?;
        let return_generators = Self::parse_return_generators(&mut item)?;
        let input_comparisons = Self::parse_input_comparisons(&mut item)?;
        let sequences = Self::parse_sequences(&mut item)?;

        Ok(WheyContext {
            item,
            mocked_traits,
            default_returns,
            return_generators,
            input_comparisons,
            sequences,
        })
    }
}

impl WheyContext {
    fn parse_mocks(item: &mut ItemStruct) -> Result<Vec<WheyMockedTrait>> {
        let mut mocks: Vec<WheyMockedTrait> = Vec::new();
        let length = item.attrs.len();
        if length > 0 {
            let mut attribute_index = length - 1;
            loop {
                let attribute = &item.attrs[attribute_index];
                if attribute.path.is_ident(MOCKS_IDENTIFIER) {
                    mocks.push(WheyMockedTrait::Transient(attribute.parse_args()?));
                    item.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(MOCKS_SINGLETON_IDENTIFIER) {
                    mocks.push(WheyMockedTrait::Singleton(attribute.parse_args()?));
                    item.attrs.remove(attribute_index);
                }
    
                if attribute_index == 0 {
                    break;
                }
                attribute_index = attribute_index - 1;
            }
        }

        mocks.reverse();
        Ok(mocks)
    }
    fn parse_default_returns(item: &mut ItemStruct) -> Result<Vec<WheyContextDefaultReturn>> {
        let mut default_returns: Vec<WheyContextDefaultReturn> = Vec::new();
        let length = item.attrs.len();
        if length > 0 {
            let mut attribute_index = length - 1;
            loop {
                let attribute = &item.attrs[attribute_index];
                if attribute.path.is_ident(MOCK_DEFAULT_RETURN) {
                    default_returns.push(attribute.parse_args()?);
                    item.attrs.remove(attribute_index);
                }
    
                if attribute_index == 0 {
                    break;
                }
                attribute_index = attribute_index - 1;
            }
        }

        default_returns.reverse();
        Ok(default_returns)
    }
    fn parse_return_generators(item: &mut ItemStruct) -> Result<Vec<WheyContextReturnGenerator>> {
        let mut return_generators: Vec<WheyContextReturnGenerator> = Vec::new();
        let length = item.attrs.len();
        if length > 0 {
            let mut attribute_index = length - 1;
            loop {
                let attribute = &item.attrs[attribute_index];
                if attribute.path.is_ident(MOCK_RETURN) {
                    return_generators.push(attribute.parse_args()?);
                    item.attrs.remove(attribute_index);
                }
    
                if attribute_index == 0 {
                    break;
                }
                attribute_index = attribute_index - 1;
            }
        }

        return_generators.reverse();
        Ok(return_generators)
    }
    fn parse_input_comparisons(item: &mut ItemStruct) -> Result<Vec<WheyContextInputComparison>> {
        let mut input_comparisons: Vec<WheyContextInputComparison> = Vec::new();
        let length = item.attrs.len();
        if length > 0 {
            let mut attribute_index = length - 1;
            loop {
                let attribute = &item.attrs[attribute_index];
                if attribute.path.is_ident(MOCK_INPUT) {
                    input_comparisons.push(attribute.parse_args()?);
                    item.attrs.remove(attribute_index);
                }
    
                if attribute_index == 0 {
                    break;
                }
                attribute_index = attribute_index - 1;
            }
        }

        input_comparisons.reverse();
        Ok(input_comparisons)
    }
    fn parse_sequences(item: &mut ItemStruct) -> Result<Vec<WheyContextSequence>> {
        let mut sequences: Vec<WheyContextSequence> = Vec::new();
        let length = item.attrs.len();
        if length > 0 {
            let mut attribute_index = length - 1;
            loop {
                let attribute = &item.attrs[attribute_index];
                if attribute.path.is_ident(MOCK_SEQUENCE) {
                    sequences.push(attribute.parse_args()?);
                    item.attrs.remove(attribute_index);
                }
    
                if attribute_index == 0 {
                    break;
                }
                attribute_index = attribute_index - 1;
            }
        }

        sequences.reverse();
        Ok(sequences)
    }

    pub fn quote(self, test_type: Option<Type>) -> TokenStream {
        let item = &self.item;
        let context_ident = &item.ident;
        let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();
        let test_type_attribute = match &test_type {
            Some(test_ident) => quote! { #[generates(#test_ident)] },
            None => quote! {},
        };
        let test_type_generator = match &test_type {
            Some(test_ident) => quote! { 
                pub fn test_type(&mut self) -> #test_ident {
                    self.generate()
                }
            },
            None => quote! {},
        };

        let mocked_traits: Vec<TokenStream> = self.mocked_traits
            .iter()
            .map(|item| item.quote_attribute_generator())
            .collect();
        let mocked_asserts: Vec<TokenStream> = self.mocked_traits
            .into_iter()
            .map(|item| item.quote_assert())
            .collect();
        
        let default_returns: Vec<TokenStream> = self.default_returns
            .into_iter()
            .map(|item| item.quote())
            .collect();
        let return_generators: Vec<TokenStream> = self.return_generators
            .into_iter()
            .map(|item| item.quote())
            .collect();
        let input_comparisons: Vec<TokenStream> = self.input_comparisons
            .into_iter()
            .map(|item| item.quote())
            .collect();
        let sequences: Vec<TokenStream> = self.sequences
            .into_iter()
            .map(|item| item.quote())
            .collect();

        quote! {
            #[service_provider]
            #(#mocked_traits)*
            #test_type_attribute
            #[generates_singleton(curds_core_abstraction::whey::WheySynchronizer)]
            #item

            #[allow(non_snake_case)]
            impl #impl_generics #context_ident #type_generics #where_clause {
                #test_type_generator

                pub fn initialize(&mut self) {
                    #(#default_returns)*
                    #(#return_generators)*
                    #(#input_comparisons)*
                    #(#sequences)*
                }

                pub fn assert(&mut self) {
                    #(#mocked_asserts)*
                    {
                        let synchronizer = curds_core_abstraction::dependency_injection::ServiceGenerator::<std::rc::Rc<std::sync::RwLock<curds_core_abstraction::whey::WheySynchronizer>>>::generate(self);
                        synchronizer.write().unwrap().assert();
                    }
                }
            }
        }
    }
}