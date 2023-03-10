use super::*;

pub struct WheyMockCore {
    mocked_trait: ItemTrait
}
impl WheyMockCore {
    pub fn new(mocked_trait: ItemTrait) -> Self {
        WheyMockCore {
            mocked_trait
        }
    }

    pub fn load_expectation_ident(ident: &Ident) -> Ident { format_ident!("load_{}", ident) }
    pub fn consume_expectation_ident(ident: &Ident) -> Ident { format_ident!("consume_{}", ident) }
    fn core_name(ident: &Ident) -> Ident { format_ident!("WheyCore{}", ident) }
    fn expect_ident(ident: &Ident) -> Ident { format_ident!("expect_{}", ident) }
    fn dummy_ident(ident: &Ident) -> Ident { format_ident!("dummy_{}", ident) }

    pub fn quote(self) -> TokenStream {
        let mocked_trait = self.mocked_trait;
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
            .flat_map(|item| Self::quote_fields(item, &core_name))
            .collect();
        let impls: Vec<TokenStream> = mocked_items
            .iter()
            .flat_map(|item| Self::quote_impls(item, &core_name))
            .collect();
        let assert_expectations: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| Self::quote_assert_expectations(item))
            .collect();
        let mocked_expectations: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| WheyMockExpectation::quote_struct(item, &core_name))
            .collect();

        quote! {
            #[injected]
            #[cfg(test)]
            #[allow(non_snake_case)]
            #vis struct #core_name #generics {
                #(#fields),*
            }
        
            impl #impl_generics #core_name #type_generics #where_clause {
                #(#impls)*

                pub fn assert(&self) {
                    #(#assert_expectations)*
                }
            }
            impl #impl_generics Drop for #core_name #type_generics #where_clause {
                fn drop(&mut self) {
                    self.assert()
                }
            }

            #(#mocked_expectations)*
        }
    }
    
    fn quote_fields(method: &TraitItemMethod, core_name: &Ident) -> Vec<TokenStream> {
        let mut fields = vec![
            Self::quote_expectation_field(method, core_name),
        ];
        match &method.sig.output {
            ReturnType::Default => {},
            ReturnType::Type(_, ty) => {
                match &**ty {
                    Type::Reference(ref_type) => {
                        let value_type = Box::new(Type::from(*ref_type.elem.clone()));
                        fields.push(Self::quote_dummy_field(method, &value_type));
                    },
                    _ => fields.push(Self::quote_dummy_field(method, ty)),
                }
            },
        }

        fields
    }
    fn quote_expectation_field(method: &TraitItemMethod, core_name: &Ident) -> TokenStream {
        let expect_ident = Self::expect_ident(&method.sig.ident);
        let expectation_name = WheyMockExpectation::expectation_name(&method.sig.ident, core_name);

        quote! {
            #[defaulted]
            #expect_ident: Vec<#expectation_name>
        }
    }
    fn quote_dummy_field(method: &TraitItemMethod, dummy_type: &Box<Type>) -> TokenStream {
        let dummy_name = Self::dummy_ident(&method.sig.ident);

        quote! {
            #[defaulted(curds_core_abstraction::whey::DummyDefault::dummy())]
            #dummy_name: #dummy_type
        }
    }

    fn quote_impls(method: &TraitItemMethod, core_name: &Ident) -> Vec<TokenStream> {
        vec![
            Self::quote_expectation_load(method, core_name),
            Self::quote_expectation_consume(method),
        ]
    }
    fn quote_expectation_load(method: &TraitItemMethod, core_name: &Ident) -> TokenStream {
        let load_expectation = Self::load_expectation_ident(&method.sig.ident);
        let expectation_ident = Self::expect_ident(&method.sig.ident);
        let expectation_type = WheyMockExpectation::expectation_name(&method.sig.ident, core_name);
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
                expectation_fields.push(quote! { expected_return: expected_return.unwrap_or(curds_core_abstraction::whey::DummyDefault::dummy()) });
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
    fn quote_expectation_consume(method: &TraitItemMethod) -> TokenStream {
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

    fn quote_assert_expectations(method: &TraitItemMethod) -> TokenStream {
        let expect_ident = Self::expect_ident(&method.sig.ident);
        let failure = format!("unfulfilled expectations for {}", method.sig.ident);
        quote! {
            if self.#expect_ident.iter().any(|expectation| !expectation.is_consumed()) {
                panic!(#failure);
            }
        }      
    }
}