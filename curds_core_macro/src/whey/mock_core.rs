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

    pub fn quote(self) -> TokenStream {
        let mocked_trait = self.mocked_trait;
        let vis = &mocked_trait.vis;
        let core_name = format_ident!("WheyCore{}", mocked_trait.ident);
        let generics = &mocked_trait.generics;
        let (impl_generics, type_generics, where_clause) = mocked_trait.generics.split_for_impl();
        
        let mocked_items: Vec<&TraitItem> = mocked_trait.items
            .iter()
            .filter(|item| WheyMock::filter_items(item))
            .collect();
        let mut fields: Vec<TokenStream> = mocked_items
            .iter()
            .flat_map(|item| Self::quote_fields(item))
            .collect();
        fields.insert(0, quote! {
            #[defaulted]
            whey_core_failing: std::cell::Cell<bool>
        });
        let expectations: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| Self::quote_expectations(item))
            .collect();
        let impls: Vec<TokenStream> = mocked_items
            .iter()
            .flat_map(|item| Self::quote_impls(item))
            .collect();
        let dummy_returns: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| Self::quote_dummy_returns(item))
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
            
        let drops: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| Self::quote_drops(item))
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
                #(#dummy_returns)*

                pub fn assert(&self) {
                    #(#assert_expectations)*
                }
                #(#asserts)*

                pub fn reset(&self) {
                    #(#resets)*
                }
            }

            impl #impl_generics core::ops::Drop for #core_name #type_generics #where_clause {
                fn drop(&mut self) {
                    if !self.whey_core_failing.get() {
                        #(#drops)*
                    }
                }
            }
        }
    }
    fn quote_fields(item: &TraitItem) -> Vec<TokenStream> {
        vec![
            Self::quote_expectation_field(item),
            Self::quote_input_comparison_field(item),
            Self::quote_call_count_field(item),
        ]
    }
    fn quote_expectation_field(item: &TraitItem) -> TokenStream {
        match item {
            TraitItem::Method(method) => {
                let expect_ident = Self::expect_ident(&method.sig.ident);

                quote! {
                    #[defaulted]
                    #expect_ident: std::cell::Cell<bool>
                }
            },
            _ => panic!("Unexpected trait item: {:?}", item),
        }
    }
    fn quote_input_comparison_field(item: &TraitItem) -> TokenStream {
        match item {
            TraitItem::Method(method) => {
                let input_compare_ident = WheyMock::input_compare_ident(&method.sig.ident);
                let mut input_signature: Vec<Box<Type>> = Vec::new();
                for input in &method.sig.inputs {
                    match input {
                        FnArg::Receiver(_) => {},
                        FnArg::Typed(ty) => match &*ty.ty {
                            Type::Reference(ref_type) => {
                                match ref_type.mutability {
                                    Some(_) => {
                                        let mut immutable_ty = ref_type.clone();
                                        immutable_ty = TypeReference {
                                            and_token: immutable_ty.and_token,
                                            lifetime: immutable_ty.lifetime,
                                            mutability: None,
                                            elem: immutable_ty.elem,
                                        };

                                        input_signature.push(Box::new(Type::Reference(immutable_ty)));
                                    },
                                    None => input_signature.push(ty.ty.clone()),
                                }
                            },
                            _ => input_signature.push(ty.ty.clone()),
                        },
                    }
                }

                quote! {
                    #[defaulted]
                    #input_compare_ident: std::cell::RefCell<Vec<(u32, std::boxed::Box<dyn Fn(#(#input_signature),*) -> bool>)>>
                }
            },
            _ => panic!("Unexpected trait item: {:?}", item),
        }
    }
    fn quote_call_count_field(item: &TraitItem) -> TokenStream {
        match item {
            TraitItem::Method(method) => {
                let call_count_ident = WheyMock::call_count_ident(&method.sig.ident);

                quote! {
                    #[defaulted]
                    #call_count_ident: std::cell::Cell<u32>
                }
            },
            _ => panic!("Unexpected trait item: {:?}", item),
        }
    }

    fn quote_expectations(item: &TraitItem) -> TokenStream {
        match item {
            TraitItem::Method(method) => {
                let expect_ident = Self::expect_ident(&method.sig.ident);
                let input_compare_ident = WheyMock::input_compare_ident(&method.sig.ident);
                let mut input_signature: Vec<Box<Type>> = Vec::new();
                for input in &method.sig.inputs {
                    match input {
                        FnArg::Receiver(_) => {},
                        FnArg::Typed(ty)=> match &*ty.ty {
                            Type::Reference(ref_type) => {
                                match ref_type.mutability {
                                    Some(_) => {
                                        let mut immutable_ty = ref_type.clone();
                                        immutable_ty = TypeReference {
                                            and_token: immutable_ty.and_token,
                                            lifetime: immutable_ty.lifetime,
                                            mutability: None,
                                            elem: immutable_ty.elem,
                                        };

                                        input_signature.push(Box::new(Type::Reference(immutable_ty)));
                                    },
                                    None => input_signature.push(ty.ty.clone()),
                                }
                            },
                            _ => input_signature.push(ty.ty.clone()),
                        },
                    }
                }

                quote! {
                    pub fn #expect_ident(&self, comparison: std::boxed::Box<dyn Fn(#(#input_signature),*) -> bool>, times: u32) {
                        self.#expect_ident.set(true);
                        self.#input_compare_ident.borrow_mut().push((times, comparison));
                    }
                }
            },
            _ => panic!("Unexpected trait item: {:?}", item),
        }
    }

    fn quote_impls(item: &TraitItem) -> Vec<TokenStream> {
        vec![
            Self::quote_input_comparison_impls(item),
            Self::quote_call_count_impls(item),
        ]
    }
    fn quote_input_comparison_impls(item: &TraitItem) -> TokenStream {
        match item {
            TraitItem::Method(method) => {
                let expect_ident = Self::expect_ident(&method.sig.ident);
                let input_compare_ident = WheyMock::input_compare_ident(&method.sig.ident);
                let mut input_signature: Vec<&FnArg> = Vec::new();
                for input in &method.sig.inputs {
                    match input {
                        FnArg::Receiver(_) => {},
                        FnArg::Typed(_) => input_signature.push(input),
                    }
                }
                let mut input_values: Vec<&Box<Pat>> = Vec::new();
                for input in &method.sig.inputs {
                    match input {
                        FnArg::Receiver(_) => {},
                        FnArg::Typed(ty) => input_values.push(&ty.pat),
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
                            let comparison = expectation.1(#(#input_values),*);
                            if expectation.0 > 1 {
                                expectation.0 -= 1;
                                self.#input_compare_ident
                                    .borrow_mut()
                                    .push(expectation);
                            }
                            if !comparison {
                                self.whey_core_failing.set(true);
                                panic!(#failed_expectation);
                            }
                        }
                        else if self.#expect_ident.get() {
                            panic!(#unexpected_invocation);
                        }
                    }
                }
            },
            _ => panic!("Unexpected trait item: {:?}", item),
        }
    }
    fn quote_call_count_impls(item: &TraitItem) -> TokenStream {
        match item {
            TraitItem::Method(method) => {
                let call_count_ident = WheyMock::call_count_ident(&method.sig.ident);

                quote! {
                    pub fn #call_count_ident(&self) {
                        self.#call_count_ident.set(self.#call_count_ident.get() + 1);
                    }
                }
            },
            _ => panic!("Unexpected trait item: {:?}", item),
        }
    }

    fn quote_dummy_returns(item: &TraitItem) -> TokenStream {
        match item {
            TraitItem::Method(method) => {
                let dummy_ident = WheyMock::dummy_ident(&method.sig.ident);

                match &method.sig.output {
                    ReturnType::Default => quote! {
                        pub fn #dummy_ident(&self) {}
                    },
                    ReturnType::Type(_, ty) => {
                        quote! {
                            pub fn #dummy_ident(&self) -> #ty {
                                curds_core_abstraction::whey::DummyDefault::dummy()
                            }
                        }
                    }
                }
            },
            _ => panic!("Unexpected trait item: {:?}", item),
        }
    }

    fn quote_asserts(item: &TraitItem) -> TokenStream {
        match item {
            TraitItem::Method(method) => {
                let assert_ident = WheyMock::assert_ident(&method.sig.ident);
                let call_count_ident = WheyMock::call_count_ident(&method.sig.ident);

                quote! {
                    pub fn #assert_ident(&self, expected_calls: u32) {
                        assert_eq!(expected_calls, self.#call_count_ident.get())
                    }
                }
            },
            _ => panic!("Unexpected trait item: {:?}", item),
        }
    }
    fn quote_assert_expectations(item: &TraitItem) -> TokenStream {
        match item {
            TraitItem::Method(method) => {
                let input_compare_ident = WheyMock::input_compare_ident(&method.sig.ident);
                let failure = format!("unfulfilled expectations for {}", method.sig.ident);

                quote! {
                    if self.#input_compare_ident.borrow().len() > 0 {
                        self.whey_core_failing.set(true);
                        panic!(#failure);
                    }
                }
            },
            _ => panic!("Unexpected trait item: {:?}", item),
        }        
    }

    fn quote_resets(item: &TraitItem) -> TokenStream {
        match item {
            TraitItem::Method(method) => {
                let expect_ident = Self::expect_ident(&method.sig.ident);
                let input_compare_ident = WheyMock::input_compare_ident(&method.sig.ident);
                let call_count_ident = WheyMock::call_count_ident(&method.sig.ident);

                quote! {
                    self.#expect_ident.set(false);
                    self.#input_compare_ident.borrow_mut().clear();
                    self.#call_count_ident.set(0);
                }
            },
            _ => panic!("Unexpected trait item: {:?}", item),
        }
    }
    
    fn quote_drops(item: &TraitItem) -> TokenStream {
        match item {
            TraitItem::Method(method) => {
                let input_compare_ident = WheyMock::input_compare_ident(&method.sig.ident);
                let failure = format!("unfulfilled expectations for {}", method.sig.ident);

                quote! {
                    if self.#input_compare_ident.borrow().len() > 0 {
                        panic!(#failure);
                    }
                }
            },
            _ => panic!("Unexpected trait item: {:?}", item),
        }
    }
}