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

    pub fn expect_ident(ident: &Ident) -> Ident { format_ident!("expect_{}", ident) }
    pub fn expect_return_ident(ident: &Ident) -> Ident { format_ident!("expect_return_{}", ident) }

    pub fn quote(self) -> TokenStream {
        let mocked_trait = self.mocked_trait;
        let vis = &mocked_trait.vis;
        let core_name = format_ident!("WheyCore{}", mocked_trait.ident);
        let generics = &mocked_trait.generics;
        let (impl_generics, type_generics, where_clause) = mocked_trait.generics.split_for_impl();
        
        let mocked_items: Vec<&TraitItemMethod> = mocked_trait.items
            .iter()
            .filter_map(|item| WheyMock::filter_items(item))
            .collect();
        let fields: Vec<TokenStream> = mocked_items
            .iter()
            .flat_map(|item| Self::quote_fields(item))
            .collect();
        let expectations: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| Self::quote_expectations(item))
            .collect();
        let impls: Vec<TokenStream> = mocked_items
            .iter()
            .flat_map(|item| Self::quote_impls(item))
            .collect();
        let returns: Vec<TokenStream> = mocked_items
            .iter()
            .flat_map(|item| Self::quote_returns(item))
            .collect();
        let assert_expectations: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| Self::quote_assert_expectations(item))
            .collect();
        let asserts: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| Self::quote_asserts(item))
            .collect();
        let resets: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| Self::quote_resets(item))
            .collect();

        quote! {
            #[injected]
            #[cfg(test)]
            #vis struct #core_name #generics {
                #(#fields),*
            }
            
            impl #impl_generics #core_name #type_generics #where_clause {
                #(#expectations)*
                #(#impls)*
                #(#returns)*

                pub fn assert(&self) {
                    #(#assert_expectations)*
                }
                #(#asserts)*

                pub fn reset(&self) {
                    #(#resets)*
                }
            }
        }
    }

    fn quote_expectation_input_signature(signature: &Signature) -> Vec<Box<Type>> {
        let mut input_signature: Vec<Box<Type>> = Vec::new();
        for input in &signature.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => match &*ty.ty {
                    Type::Reference(ref_type) => {
                        let value_type = Type::from(*ref_type.elem.clone());
                        input_signature.push(Box::new(value_type));
                    },
                    _ => input_signature.push(ty.ty.clone()),
                },
            }
        }
        input_signature
    }
    
    fn quote_fields(method: &TraitItemMethod) -> Vec<TokenStream> {
        let mut fields = vec![
            Self::quote_expectation_field(method),
            Self::quote_input_comparison_field(method),
            Self::quote_call_count_field(method),
        ];
        match &method.sig.output {
            ReturnType::Default => {},
            ReturnType::Type(_, _) => {
                fields.insert(1, Self::quote_expectation_return_field(method));
            }
        }

        fields
    }
    fn quote_expectation_field(method: &TraitItemMethod) -> TokenStream {
        let expect_ident = Self::expect_ident(&method.sig.ident);

        quote! {
            #[defaulted]
            #expect_ident: std::cell::Cell<bool>
        }
    }
    fn quote_expectation_return_field(method: &TraitItemMethod) -> TokenStream {
        let expect_return_ident = Self::expect_return_ident(&method.sig.ident);

        match &method.sig.output {
            ReturnType::Default => quote! { },
            ReturnType::Type(_, ty) => {
                let mut return_type = *ty.clone();
                match &return_type {
                    Type::Reference(ref_type) => return_type = Type::from(*ref_type.elem.clone()),
                    _ => {},
                }
                quote! {
                    #[defaulted]
                    #expect_return_ident: Vec<(u32, #return_type)>
                }
            }
        }
    }
    fn quote_input_comparison_field(method: &TraitItemMethod) -> TokenStream {
        let input_compare_ident = WheyMock::input_compare_ident(&method.sig.ident);   
        let input_signature = Self::quote_expectation_input_signature(&method.sig);

        quote! {
            #[defaulted]
            #input_compare_ident: std::cell::RefCell<Vec<(u32, (#(#input_signature),*))>>
        }
    }
    fn quote_call_count_field(method: &TraitItemMethod) -> TokenStream {
        let call_count_ident = WheyMock::call_count_ident(&method.sig.ident);

        quote! {
            #[defaulted]
            #call_count_ident: std::cell::Cell<u32>
        }
    }

    fn quote_expectations(method: &TraitItemMethod) -> TokenStream {
        let expect_ident = Self::expect_ident(&method.sig.ident);
        let input_compare_ident = WheyMock::input_compare_ident(&method.sig.ident);
        let input_signature = Self::quote_expectation_input_signature(&method.sig);

        quote! {
            pub fn #expect_ident(&self, expected: (#(#input_signature),*), times: u32) {
                self.#expect_ident.set(true);
                self.#input_compare_ident.borrow_mut().insert(0, (times, expected));
            }
        }
    }

    fn quote_impls(method: &TraitItemMethod) -> Vec<TokenStream> {
        vec![
            Self::quote_input_comparison_impls(method),
            Self::quote_call_count_impls(method),
        ]
    }
    fn quote_input_comparison_impls(method: &TraitItemMethod) -> TokenStream {
        let expect_ident = Self::expect_ident(&method.sig.ident);
        let input_compare_ident = WheyMock::input_compare_ident(&method.sig.ident);
        let mut input_signature: Vec<&FnArg> = Vec::new();
        for input in &method.sig.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(_) => input_signature.push(input),
            }
        }
        let mut input_values: Vec<TokenStream> = Vec::new();
        for input in &method.sig.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => {
                    let input_name = &ty.pat;
                    match &*ty.ty {
                        Type::Reference(_) => {
                            input_values.push(quote!{ *#input_name });
                        },
                        _ => input_values.push(quote!{ #input_name }),
                    }
                },
            }
        }
        let failed_expectation = format!("failed expectation for {}", method.sig.ident);
        let unexpected_invocation = format!("unexpected invocation for {}", method.sig.ident);

        quote! {
            pub fn #input_compare_ident(&self, #(#input_signature),*) {
                if self.#input_compare_ident.borrow().len() > 0 {
                    let mut expectation = self.#input_compare_ident
                        .borrow_mut()
                        .pop()
                        .unwrap();
                    let comparison = expectation.1 == (#(#input_values),*);
                    if expectation.0 > 1 {
                        expectation.0 -= 1;
                        self.#input_compare_ident
                            .borrow_mut()
                            .push(expectation);
                    }
                    if !comparison {
                        panic!(#failed_expectation);
                    }
                }
                else if self.#expect_ident.get() {
                    panic!(#unexpected_invocation);
                }
            }
        }
    }
    fn quote_call_count_impls(method: &TraitItemMethod) -> TokenStream {
        let call_count_ident = WheyMock::call_count_ident(&method.sig.ident);

        quote! {
            pub fn #call_count_ident(&self) {
                self.#call_count_ident.set(self.#call_count_ident.get() + 1);
            }
        }
    }

    fn quote_returns(method: &TraitItemMethod) -> Vec<TokenStream> {
        let mut returns = vec![
            Self::quote_dummy_returns(method),
        ];
        match &method.sig.output {
            ReturnType::Default => {},
            ReturnType::Type(_, _) => {
                returns.insert(0, Self::quote_expected_returns(method));
            }
        }

        returns
    }
    fn quote_expected_returns(method: &TraitItemMethod) -> TokenStream {
        let expect_return_ident = Self::expect_return_ident(&method.sig.ident);

        match &method.sig.output {
            ReturnType::Default => quote! { },
            ReturnType::Type(_, ty) => {                        
                let mut return_type = *ty.clone();
                match &return_type {
                    Type::Reference(ref_type) => return_type = Type::from(*ref_type.elem.clone()),
                    _ => {},
                }
                quote! {
                    pub fn #expect_return_ident(&mut self, returned: #return_type, times: u32) {
                        self.#expect_return_ident.push((times, returned));
                    }
                }
            }
        }
    }
    fn quote_dummy_returns(method: &TraitItemMethod) -> TokenStream {
        let dummy_ident = WheyMock::dummy_ident(&method.sig.ident);
        let expect_return_ident = Self::expect_return_ident(&method.sig.ident);
        let mut input_values: Vec<&Box<Pat>> = Vec::new();
        for input in &method.sig.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => input_values.push(&ty.pat),
            }
        }

        match &method.sig.output {
            ReturnType::Default => quote! {
                pub fn #dummy_ident(&self) {}
            },
            ReturnType::Type(_, ty) => {
                match &**ty {
                    Type::Reference(ref_type) => {
                        let unexpected_invocation = format!("unexpected invocation for {}", method.sig.ident);
                        quote! {
                            pub fn #dummy_ident(&self) -> #ty {
                                &self.#expect_return_ident
                                    .iter()
                                    .find(|expectation| expectation.0 > 0)
                                    .map(|expectation| &expectation.1)
                                    .unwrap()
                            }
                        }
                    },
                    Type::Path(_) => {
                        quote! {
                            pub fn #dummy_ident(&self) -> #ty {
                                if self.#expect_return_ident.borrow().len() > 0 {
                                    let mut expectation = self.#expect_return_ident
                                        .borrow_mut()
                                        .pop()
                                        .unwrap();
                                    let return_value = expectation.1.clone();
                                    if expectation.0 > 1 {
                                        expectation.0 -= 1;
                                        self.#expect_return_ident
                                            .borrow_mut()
                                            .push(expectation);
                                    }
                                    return return_value;
                                }
                                curds_core_abstraction::whey::DummyDefault::dummy()
                            }
                        }
                    },
                    _ => quote! {},
                }
            }
        }
    }

    fn quote_asserts(method: &TraitItemMethod) -> TokenStream {
        let assert_ident = WheyMock::assert_ident(&method.sig.ident);
        let call_count_ident = WheyMock::call_count_ident(&method.sig.ident);

        quote! {
            pub fn #assert_ident(&self, expected_calls: u32) {
                assert_eq!(expected_calls, self.#call_count_ident.get())
            }
        }
    }
    fn quote_assert_expectations(method: &TraitItemMethod) -> TokenStream {
        let input_compare_ident = WheyMock::input_compare_ident(&method.sig.ident);
        let failure = format!("unfulfilled expectations for {}", method.sig.ident);
        let return_expectations = match &method.sig.output {
            ReturnType::Default => quote! {},
            ReturnType::Type(_, _) => {
                let expect_return_ident = Self::expect_return_ident(&method.sig.ident);
                quote! {
                    todo!("asserts");
                    // if self.#expect_return_ident.borrow().len() > 0 {
                    //     panic!(#failure);
                    // }
                }
            }
        };

        quote! {
            if self.#input_compare_ident.borrow().len() > 0 {
                panic!(#failure);
            }
            #return_expectations
        }      
    }

    fn quote_resets(method: &TraitItemMethod) -> TokenStream {
        let expect_ident = Self::expect_ident(&method.sig.ident);
        let input_compare_ident = WheyMock::input_compare_ident(&method.sig.ident);
        let call_count_ident = WheyMock::call_count_ident(&method.sig.ident);

        quote! {
            self.#expect_ident.set(false);
            self.#input_compare_ident.borrow_mut().clear();
            self.#call_count_ident.set(0);
        }
    }
}