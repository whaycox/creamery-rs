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

    pub fn load_expectation_ident(ident: &Ident) -> Ident { format_ident!("load_{}", ident) }
    pub fn consume_expectation_ident(ident: &Ident) -> Ident { format_ident!("consume_{}", ident) }
    pub fn core_name(ident: &Ident) -> Ident { format_ident!("WheyCore{}", ident) }
    fn expect_calls(ident: &Ident) -> Ident { format_ident!("expect_calls_{}", ident) }
    fn expected_calls(ident: &Ident) -> Ident { format_ident!("expected_calls_{}", ident) }
    fn calls(ident: &Ident) -> Ident { format_ident!("calls_{}", ident) }
    fn record_call(ident: &Ident) -> Ident { format_ident!("record_call_{}", ident) }
    fn expect_ident(ident: &Ident) -> Ident { format_ident!("expect_{}", ident) }
    fn dummy_ident(ident: &Ident) -> Ident { format_ident!("dummy_{}", ident) }

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

                pub fn assert(&self) {
                    #(#assert_expectations)*
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
            self.quote_calls_field(method),
            //self.quote_expectation_field(method),
        ];
        // match &method.sig.output {
        //     ReturnType::Default => {},
        //     ReturnType::Type(_, ty) => {
        //         match &**ty {
        //             Type::Reference(ref_type) => {
        //                 let value_type = Box::new(Type::from(*ref_type.elem.clone()));
        //                 fields.push(Self::quote_dummy_field(method, &value_type));
        //             },
        //             _ => fields.push(Self::quote_dummy_field(method, ty)),
        //         }
        //     },
        // }

        fields
    }
    fn quote_expected_calls_field(&self, method: &TraitItemMethod) -> TokenStream {
        let expected_calls_field = Self::expected_calls(&method.sig.ident);
        quote! {
            #[defaulted]
            #expected_calls_field: std::option::Option<u32>
        }
    }
    fn quote_calls_field(&self, method: &TraitItemMethod) -> TokenStream {
        let calls_field = Self::calls(&method.sig.ident);
        quote! {
            #[defaulted]
            #calls_field: u32
        }
    }
    fn quote_expectation_field(&self, method: &TraitItemMethod) -> TokenStream {
        let expect_ident = Self::expect_ident(&method.sig.ident);
        let expectation_name = WheyMockExpectation::expectation_name(&method.sig.ident, &Self::core_name(&self.mock.mocked_trait.ident));

        quote! {
            #[defaulted]
            #expect_ident: Vec<#expectation_name>
        }
    }
    fn quote_dummy_field(method: &TraitItemMethod, dummy_type: &Box<Type>) -> TokenStream {
        let dummy_name = Self::dummy_ident(&method.sig.ident);

        quote! {
            #[defaulted(std::default::Default::default())]
            #dummy_name: #dummy_type
        }
    }

    fn quote_impls(&self, method: &TraitItemMethod) -> Vec<TokenStream> {
        vec![
            self.quote_expect_calls(method),
            self.quote_call(method),
            //self.quote_expectation_load(method),
            //self.quote_expectation_consume(method),
        ]
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
    fn quote_call(&self, method: &TraitItemMethod) -> TokenStream {
        let record_call = Self::record_call(&method.sig.ident);
        let calls = Self::calls(&method.sig.ident);
        quote! {
            pub fn #record_call(&mut self) {
                self.#calls += 1;
            }

        }
    }
    fn quote_expectation_load(&self, method: &TraitItemMethod) -> TokenStream {
        let load_expectation = Self::load_expectation_ident(&method.sig.ident);
        let expectation_ident = Self::expect_ident(&method.sig.ident);
        let expectation_type = WheyMockExpectation::expectation_name(&method.sig.ident, &Self::core_name(&self.mock.mocked_trait.ident));
        let mut load_signature = vec![quote! { &mut self }];
        let mut expectation_fields = vec![quote! { times: std::cell::Cell::new(times) }];
        match WheyMockExpectation::parse_expectation_input_types(&method.sig) {
            Some(expected_values) => {
                load_signature.push(quote! { expected: (#(#expected_values),*) });
                expectation_fields.push(quote! { input_comparison: expected });
            },
            None => load_signature.push(quote! { expected: () }),
        }
        match WheyMockExpectation::parse_expectation_return_type(&method.sig) {
            Some(expected_return) => {
                load_signature.push(quote! { expected_return: Option<#expected_return> });
                expectation_fields.push(quote! { expected_return: expected_return.unwrap_or(std::default::Default::default()) });
            },
            None => load_signature.push(quote! { expected_return: Option<()> }),
        }
        load_signature.push(quote! { times: u32 });
        quote! {
            pub fn #load_expectation(#(#load_signature),*) {
                self.#expectation_ident.push(#expectation_type {
                    #(#expectation_fields),*
                });
            }
        }
    }
    fn quote_expectation_consume(&self, method: &TraitItemMethod) -> TokenStream {
        let consume_expectation = Self::consume_expectation_ident(&method.sig.ident);
        let expect_ident = Self::expect_ident(&method.sig.ident);
        let mut input_name = quote! {};
        let mut input_signature: Vec<TokenStream> = vec![];
        let mut iterate_tokens = quote! { self.#expect_ident.iter().find(|expectation| !expectation.is_consumed()) };
        match &method.sig.inputs.first().unwrap() {
            FnArg::Receiver(receiver) => match &receiver.mutability {
                Some(_) => {
                    input_signature.push(quote! { &mut self });
                    iterate_tokens = quote! { self.#expect_ident.iter_mut().find(|expectation| !expectation.is_consumed()) };
                },
                None => input_signature.push(quote! { &self }),
            },
            _ => {},
        }
        match WheyMockExpectation::parse_consume_input_types(&method.sig) {
            Some(types) => {
                input_name = quote! { expected };
                input_signature.push(quote! { expected: (#(#types),*) });
            },
            None => {},
        }
        let dummy_name = Self::dummy_ident(&method.sig.ident);
        let mut return_type: Option<TokenStream> = None;
        let failure = format!("unexpected invocation for {}", method.sig.ident);
        let mut unexpected_return = quote! { {} };
        match &method.sig.output {
            ReturnType::Default => {},
            ReturnType::Type(_, ty) => {
                return_type = Some(quote! { -> #ty });
                unexpected_return = match &**ty {
                    Type::Reference(reference_type) => match &reference_type.mutability {
                        Some(_) => quote!{ &mut self.#dummy_name },
                        None => quote!{ &self.#dummy_name },
                    },
                    _ => quote!{ self.#dummy_name.clone() },
                };
            },
        };

        quote! {
            pub fn #consume_expectation(#(#input_signature),*) #return_type {
                if self.#expect_ident.len() == 0 {
                    #unexpected_return
                }
                else {
                    match #iterate_tokens {
                        Some(expectation) => expectation.consume(#input_name),
                        None => panic!(#failure),
                    }
                }
            }
        }
    }

    fn quote_assert_expectations(&self, method: &TraitItemMethod) -> TokenStream {
        let expected_calls = Self::expected_calls(&method.sig.ident);
        let recorded_calls = Self::calls(&method.sig.ident);
        let expected_calls_failure = format!("expected {{}} calls to {}::{} but only recorded {{}}", self.mock.mocked_trait.ident, method.sig.ident);
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

    fn expectation(&'a self, method: &'a TraitItemMethod) -> WheyMockExpectation { WheyMockExpectation::new(self, method) }
}