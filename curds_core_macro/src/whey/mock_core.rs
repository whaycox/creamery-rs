use super::*;

pub struct WheyMockCore<'a> {
    pub mock: &'a WheyMock
}
impl<'a> WheyMockCore<'a> {
    pub fn new(mock: &'a WheyMock) -> Self {
        WheyMockCore {
            mock
        }
    }

    pub fn expect_calls(ident: &Ident) -> Ident { format_ident!("expect_calls_{}", ident) }
    pub fn record_call(ident: &Ident) -> Ident { format_ident!("record_call_{}", ident) }
    pub fn expect_input(ident: &Ident) -> Ident { format_ident!("expect_input_{}", ident) }
    pub fn store_return(ident: &Ident) -> Ident { format_ident!("store_return_{}", ident) }
    pub fn generate_return(ident: &Ident) -> Ident { format_ident!("generate_return_{}", ident) }
    pub fn core_name(ident: &Ident) -> Ident { format_ident!("WheyCore{}", ident) }
    fn expected_calls(ident: &Ident) -> Ident { format_ident!("expected_calls_{}", ident) }
    fn recorded_calls(ident: &Ident) -> Ident { format_ident!("recorded_calls_{}", ident) }
    fn expected_input(ident: &Ident) -> Ident { format_ident!("expected_input_{}", ident) }
    fn expect_ident(ident: &Ident) -> Ident { format_ident!("expect_{}", ident) }
    fn returned(ident: &Ident) -> Ident { format_ident!("returned_{}", ident) }

    pub fn quote(self) -> TokenStream {
        let mocked_trait = &self.mock.mocked_trait;
        let vis = &mocked_trait.vis;
        let core_name = Self::core_name(&mocked_trait.ident);
        let generics = &mocked_trait.generics;
        let (impl_generics, type_generics, where_clause) = mocked_trait.generics.split_for_impl();
        
        let mocked_items: Vec<&TraitItemMethod> = mocked_trait.items
            .iter()
            .filter_map(|item| WheyMock::filter_items(item))
            .collect();
        let fields: Vec<TokenStream> = mocked_items
            .iter()
            .flat_map(|item| self.quote_fields(item))
            .collect();
        let impls: Vec<TokenStream> = mocked_items
            .iter()
            .flat_map(|item| self.quote_impls(item))
            .collect();
        let assert_expectations: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| self.quote_assert_expectations(item))
            .collect();
        let reset_expectations: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| self.quote_reset_expectations(item))
            .collect();
        // let mocked_expectations: Vec<TokenStream> = mocked_items
        //     .iter()
        //     .map(|item| self.expectation(item).quote_struct())
        //     .collect();

        quote! {
            #[injected]
            #[cfg(test)]
            #[allow(non_snake_case)]
            #vis struct #core_name #generics {
                #(#fields),*
            }
        
            #[cfg(test)]
            impl #impl_generics #core_name #type_generics #where_clause {
                #(#impls)*

                pub fn assert(&mut self) {
                    #(#assert_expectations)*
                    self.reset();
                }

                pub fn reset(&mut self) {
                    #(#reset_expectations)*
                }
            }
            
            #[cfg(test)]
            impl #impl_generics Drop for #core_name #type_generics #where_clause {
                fn drop(&mut self) {
                    if !std::thread::panicking() {
                        self.assert();
                    }
                }
            }

            // #(#mocked_expectations)*
        }
    }
    
    fn quote_fields(&self, method: &TraitItemMethod) -> Vec<TokenStream> {
        let mut fields = vec![
            self.quote_expected_calls_field(method),
            self.quote_recorded_calls_field(method),
            //self.quote_expected_input_field(method),
            //self.quote_expectation_field(method),
        ];
        match &method.sig.output {
            ReturnType::Default => {},
            ReturnType::Type(_, ty) => {
                fields.push(self.quote_returned_field(&method.sig.ident, ty))
            },
        }

        fields
    }
    fn quote_expected_calls_field(&self, method: &TraitItemMethod) -> TokenStream {
        let expected_calls_field = Self::expected_calls(&method.sig.ident);
        quote! {
            #[defaulted]
            #expected_calls_field: std::option::Option<u32>
        }
    }
    fn quote_recorded_calls_field(&self, method: &TraitItemMethod) -> TokenStream {
        let recorded_calls_field = Self::recorded_calls(&method.sig.ident);
        quote! {
            #[defaulted]
            #recorded_calls_field: u32
        }
    }
    fn quote_expected_input_field(&self, method: &TraitItemMethod) -> TokenStream {
        let expected_input_field = Self::expected_input(&method.sig.ident);
        quote! {
            #[defaulted]
            #expected_input_field: u32
        }
    }
    fn quote_returned_field(&self, ident: &Ident, returned_type: &Box<Type>) -> TokenStream {
        let returned_field = Self::returned(ident);
        quote! {
            #[defaulted]
            #returned_field: Vec<Box<dyn Fn() -> #returned_type>>
        }
    }

    fn quote_impls(&self, method: &TraitItemMethod) -> Vec<TokenStream> {
        let mut impls = vec![
            self.quote_expect_calls(method),
            self.quote_record_call(method),
            //self.quote_expect_input(method),
            //self.quote_expectation_consume(method),
        ];        
        match &method.sig.output {
            ReturnType::Default => {},
            ReturnType::Type(_, ty) => {
                impls.push(self.quote_store_return(&method.sig.ident, ty));
                impls.push(self.quote_generate_return(&method.sig.ident, ty));
            },
        }

        impls
    }
    fn quote_expect_calls(&self, method: &TraitItemMethod) -> TokenStream {
        let expect_calls = Self::expect_calls(&method.sig.ident);
        let expected_calls_field = Self::expected_calls(&method.sig.ident);
        quote! {
            pub fn #expect_calls(&mut self, calls: u32) {
                self.#expected_calls_field = Some(calls);
            }
        }
    }
    fn quote_record_call(&self, method: &TraitItemMethod) -> TokenStream {
        let record_call = Self::record_call(&method.sig.ident);
        let recorded_calls = Self::recorded_calls(&method.sig.ident);
        quote! {
            pub fn #record_call(&mut self) {
                self.#recorded_calls += 1;
            }
        }
    }
    fn quote_expect_input(&self, method: &TraitItemMethod) -> TokenStream {
        let expect_input = Self::expect_input(&method.sig.ident);
        // let expectation_ident = Self::expect_ident(&method.sig.ident);
        // let expectation_type = WheyMockExpectation::expectation_name(&method.sig.ident, &Self::core_name(&self.mock.mocked_trait.ident));
        let mut load_signature = vec![quote! { &mut self }];
        //let mut expectation_fields = vec![quote! { times: std::cell::Cell::new(times) }];
        match WheyMockExpectation::parse_consume_input_types(&method.sig) {
            Some(expected_values) => {
                load_signature.push(quote! { expected: Box<dyn Fn(#(#expected_values),*) -> bool> });
                //expectation_fields.push(quote! { input_comparison: expected });
            },
            None => load_signature.push(quote! { expected: () }),
        }
        load_signature.push(quote! { times: u32 });
        quote! {
            pub fn #expect_input(#(#load_signature),*) {
                panic!("not yet implemented");
                // self.#expectation_ident.push(#expectation_type {
                //     #(#expectation_fields),*
                // });
            }
        }
    }
    fn quote_store_return(&self, ident: &Ident, returned_type: &Box<Type>) -> TokenStream {
        let store_return = Self::store_return(ident);
        let returned = Self::returned(ident);
        quote! {
            pub fn #store_return(&mut self, generator: Box<dyn Fn() -> #returned_type>) {
                panic!("not yet implemented")
            }
        }
    }
    fn quote_generate_return(&self, ident: &Ident, returned_type: &Box<Type>) -> TokenStream {
        let generate_return = Self::generate_return(ident);
        let returned = Self::returned(ident);
        quote! {
            pub fn #generate_return(&mut self) -> #returned_type {
                panic!("not yet implemented")
            }
        }
    }

    fn quote_assert_expectations(&self, method: &TraitItemMethod) -> TokenStream {
        let expected_calls = Self::expected_calls(&method.sig.ident);
        let recorded_calls = Self::recorded_calls(&method.sig.ident);
        let expected_calls_failure = format!("expected {{}} calls to {}::{} but recorded {{}} instead", self.mock.mocked_trait.ident, method.sig.ident);
        
        let expect_ident = Self::expect_ident(&method.sig.ident);
        let failure = format!("unfulfilled expectations for {}", method.sig.ident);
        
        quote! {
            if self.#expected_calls.is_some() && self.#expected_calls.unwrap() != self.#recorded_calls {
                panic!(#expected_calls_failure, self.#expected_calls.unwrap(), self.#recorded_calls);
            }
            // if self.#expect_ident.iter().any(|expectation| !expectation.is_consumed()) {
            //     panic!(#failure);
            // }
        }      
    }

    fn quote_reset_expectations(&self, method: &TraitItemMethod) -> TokenStream {
        let expected_calls = Self::expected_calls(&method.sig.ident);
        let recorded_calls = Self::recorded_calls(&method.sig.ident);
        
        quote! {
            self.#expected_calls = None;
            self.#recorded_calls = 0;
        }      
    }

    fn expectation(&'a self, method: &'a TraitItemMethod) -> WheyMockExpectation { WheyMockExpectation::new(self, method) }
}