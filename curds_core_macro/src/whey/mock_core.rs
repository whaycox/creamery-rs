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
    pub fn store_expected_input(ident: &Ident) -> Ident { format_ident!("store_expected_input_{}", ident) }
    pub fn consume_expected_input(ident: &Ident) -> Ident { format_ident!("consume_expected_input_{}", ident) }
    pub fn default_return(ident: &Ident) -> Ident { format_ident!("default_return_{}", ident) }
    pub fn store_return(ident: &Ident) -> Ident { format_ident!("store_return_{}", ident) }
    pub fn generate_return(ident: &Ident) -> Ident { format_ident!("generate_return_{}", ident) }
    pub fn core_name(ident: &Ident) -> Ident { format_ident!("WheyCore{}", ident) }
    fn expected_calls(ident: &Ident) -> Ident { format_ident!("expected_calls_{}", ident) }
    fn recorded_calls(ident: &Ident) -> Ident { format_ident!("recorded_calls_{}", ident) }
    fn default_generator(ident: &Ident) -> Ident { format_ident!("default_generator_{}", ident) }
    fn expected_input(ident: &Ident) -> Ident { format_ident!("expected_input_{}", ident) }
    fn expected_input_times(ident: &Ident) -> Ident { format_ident!("expected_input_times_{}", ident) }
    fn returned_times(ident: &Ident) -> Ident { format_ident!("returned_times_{}", ident) }
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
        }
    }
    
    fn quote_fields(&self, method: &TraitItemMethod) -> Vec<TokenStream> {
        let mut fields = vec![
            self.quote_expected_calls_field(method),
            self.quote_recorded_calls_field(method),
        ];
        let mut input_types: Vec<Box<Type>> = Vec::new();
        for input in &method.sig.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => match &*ty.ty {
                    _ => input_types.push(ty.ty.clone()),
                },
            }
        }

        if input_types.len() > 0 {
            fields.push(self.quote_expected_input_times_field(&method.sig.ident));
            fields.push(self.quote_expected_input_field(&method.sig.ident, &input_types));
        }
        match &method.sig.output {
            ReturnType::Default => {},
            ReturnType::Type(_, ty) => {
                fields.push(self.quote_default_return_field(&method.sig.ident, &input_types, ty));
                fields.push(self.quote_returned_times_field(&method.sig.ident));
                fields.push(self.quote_returned_field(&method.sig.ident, &input_types, ty));
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
    fn quote_expected_input_times_field(&self, ident: &Ident) -> TokenStream {
        let expected_input_times_field = Self::expected_input_times(ident);
        quote! {
            #[defaulted]
            #expected_input_times_field: std::vec::Vec<u32>
        }
    }
    fn quote_expected_input_field(&self, ident: &Ident, input_types: &Vec<Box<Type>>) -> TokenStream {
        let expected_input_field = Self::expected_input(ident);
        quote! {
            #[defaulted]
            #expected_input_field: std::vec::Vec<std::boxed::Box<dyn Fn(#(#input_types),*) -> bool>>
        }
    }
    fn quote_default_return_field(&self, ident: &Ident, input_types: &Vec<Box<Type>>, returned_type: &Box<Type>) -> TokenStream {
        let default_generator_field = Self::default_generator(ident);
        let mut default_attribute = quote! { #[defaulted] };
        if self.mock.defaulted_returns.contains_key(ident) {
            let default_generator = &self.mock.defaulted_returns[ident];
            default_attribute = quote! { #[defaulted(#default_generator)] };
        }

        quote! {
            #default_attribute
            #default_generator_field: std::option::Option<std::boxed::Box<dyn Fn(#(#input_types),*) -> #returned_type>>
        }
    }
    fn quote_returned_times_field(&self, ident: &Ident) -> TokenStream {
        let returned_times_field = Self::returned_times(ident);
        quote! {
            #[defaulted]
            #returned_times_field: std::vec::Vec<u32>
        }
    }
    fn quote_returned_field(&self, ident: &Ident, input_types: &Vec<Box<Type>>, returned_type: &Box<Type>) -> TokenStream {
        let returned_field = Self::returned(ident);
        quote! {
            #[defaulted]
            #returned_field: std::vec::Vec<std::boxed::Box<dyn Fn(#(#input_types),*) -> #returned_type>>
        }
    }

    fn quote_impls(&self, method: &TraitItemMethod) -> Vec<TokenStream> {
        let mut impls = vec![
            self.quote_expect_calls(method),
            self.quote_record_call(method),
        ];
        let mut input_types: Vec<Box<Type>> = Vec::new();
        for input in &method.sig.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => match &*ty.ty {
                    _ => input_types.push(ty.ty.clone()),
                },
            }
        }

        if input_types.len() > 0 {
            impls.push(self.quote_store_expected_input(&method.sig.ident, &input_types));
            impls.push(self.quote_consume_expected_input(&method, &input_types));
        }
        match &method.sig.output {
            ReturnType::Default => {},
            ReturnType::Type(_, ty) => {
                impls.push(self.quote_default_return(&method, &input_types, ty));
                impls.push(self.quote_store_return(&method, &input_types, ty));
                impls.push(self.quote_generate_return(&method, ty));
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
    fn quote_store_expected_input(&self, ident: &Ident, input_types: &Vec<Box<Type>>) -> TokenStream {
        let store_input = Self::store_expected_input(ident);
        let expected_input_times_field = Self::expected_input_times(ident);
        let expected_input_field = Self::expected_input(ident);

        quote! {
            pub fn #store_input(&mut self, comparison: std::boxed::Box<dyn Fn(#(#input_types),*) -> bool>, times: u32) {
                self.#expected_input_times_field.push(times);
                self.#expected_input_field.push(comparison);
            }
        }
    }
    fn quote_consume_expected_input(&self, method: &TraitItemMethod, input_types: &Vec<Box<Type>>) -> TokenStream {
        let consume_input = Self::consume_expected_input(&method.sig.ident);
        let mut signature_inputs: Vec<TokenStream> = vec![ quote! { &mut self } ];
        let mut input_names: Vec<&Box<Pat>> = Vec::new();
        for input in &method.sig.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => match &*ty.ty {
                    _ => {
                        signature_inputs.push(quote! { #input });
                        input_names.push(&ty.pat);
                    },
                },
            }
        }
        let expected_input_times_field = Self::expected_input_times(&method.sig.ident);
        let expected_input_field = Self::expected_input(&method.sig.ident);
        let expected_input_failure = format!("the expected inputs for {}::{} were not supplied", self.mock.mocked_trait.ident, method.sig.ident);

        quote! {
            pub fn #consume_input(#(#signature_inputs),*) {
                let length = self.#expected_input_times_field.len();
                for i in 0..length {
                    if self.#expected_input_times_field[i] > 0 {
                        self.#expected_input_times_field[i] -= 1;
                        if !(self.#expected_input_field[i])(#(#input_names),*) {
                            panic!(#expected_input_failure);
                        }
                    }
                }
            }
        }
    }
    fn quote_default_return(&self, method: &TraitItemMethod, input_types: &Vec<Box<Type>>, returned_type: &Box<Type>) -> TokenStream {
        let default_return = Self::default_return(&method.sig.ident);
        let default_generator_field = Self::default_generator(&method.sig.ident);

        quote! {
            pub fn #default_return(&mut self, generator: std::boxed::Box<dyn Fn(#(#input_types),*) -> #returned_type>) {
                self.#default_generator_field = Some(generator);
            }
        }
    }
    fn quote_store_return(&self, method: &TraitItemMethod, input_types: &Vec<Box<Type>>, returned_type: &Box<Type>) -> TokenStream {
        let store_return = Self::store_return(&method.sig.ident);
        let returned_times_field = Self::returned_times(&method.sig.ident);
        let returned_field = Self::returned(&method.sig.ident);

        quote! {
            pub fn #store_return(&mut self, generator: std::boxed::Box<dyn Fn(#(#input_types),*) -> #returned_type>, times: u32) {
                self.#returned_times_field.push(times);
                self.#returned_field.push(generator);
            }
        }
    }
    fn quote_generate_return(&self, method: &TraitItemMethod, returned_type: &Box<Type>) -> TokenStream {
        let generate_return = Self::generate_return(&method.sig.ident);
        let mut signature_inputs: Vec<TokenStream> = vec![ quote! { &mut self } ];
        let mut input_names: Vec<&Box<Pat>> = Vec::new();
        for input in &method.sig.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => match &*ty.ty {
                    _ => {
                        signature_inputs.push(quote! { #input });
                        input_names.push(&ty.pat);
                    },
                },
            }
        }
        let returned_times_field = Self::returned_times(&method.sig.ident);
        let returned_field = Self::returned(&method.sig.ident);
        let default_generator_field = Self::default_generator(&method.sig.ident);

        quote! {
            pub fn #generate_return(#(#signature_inputs),*) -> #returned_type {
                let length = self.#returned_times_field.len();
                for i in 0..length {
                    if self.#returned_times_field[i] > 0 {
                        self.#returned_times_field[i] -= 1;
                        return (self.#returned_field[i])(#(#input_names),*)
                    }
                }
                match &self.#default_generator_field {
                    Some(generator) => return generator(#(#input_names),*),
                    _ => panic!("a return is necessary but none have been supplied"),
                }
            }
        }
    }

    fn quote_assert_expectations(&self, method: &TraitItemMethod) -> TokenStream {
        let expected_calls = Self::expected_calls(&method.sig.ident);
        let recorded_calls = Self::recorded_calls(&method.sig.ident);
        let expected_calls_failure = format!("expected {{}} calls to {}::{} but recorded {{}} instead", self.mock.mocked_trait.ident, method.sig.ident);
        
        let mut has_inputs = false;
        for input in &method.sig.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => {
                    has_inputs = true;
                    break;
                },
            }
        }
        let input_assert = if has_inputs {
            let expected_input_times_field = Self::expected_input_times(&method.sig.ident);
            let expected_input_times_failure = format!("not all stored input comparisons for {}::{} have been consumed", self.mock.mocked_trait.ident, method.sig.ident);

            quote! {
                if self.#expected_input_times_field.iter().any(|comparison| *comparison != 0) {
                    panic!(#expected_input_times_failure);
                }
            }
        }
        else { quote! {} };
        let return_assert = match &method.sig.output {
            ReturnType::Default => quote! {},
            ReturnType::Type(_, ty) => {
                let returned_times_field = Self::returned_times(&method.sig.ident);
                let returned_times_failure = format!("not all stored returns for {}::{} have been consumed", self.mock.mocked_trait.ident, method.sig.ident);

                quote! {
                    if self.#returned_times_field.iter().any(|generator| *generator != 0) {
                        panic!(#returned_times_failure);
                    }
                }
            },
        };
        
        quote! {
            if self.#expected_calls.is_some() && self.#expected_calls.unwrap() != self.#recorded_calls {
                panic!(#expected_calls_failure, self.#expected_calls.unwrap(), self.#recorded_calls);
            }
            #input_assert
            #return_assert
        }      
    }

    fn quote_reset_expectations(&self, method: &TraitItemMethod) -> TokenStream {
        let expected_calls = Self::expected_calls(&method.sig.ident);
        let recorded_calls = Self::recorded_calls(&method.sig.ident);

        let mut has_inputs = false;
        for input in &method.sig.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => {
                    has_inputs = true;
                    break;
                },
            }
        }
        let input_reset = if has_inputs {
            let expected_input_times_field = Self::expected_input_times(&method.sig.ident);
            let expected_input_field = Self::expected_input(&method.sig.ident);

            quote! {
                self.#expected_input_times_field.clear();
                self.#expected_input_field.clear();
            }
        }
        else { quote! {} };
        let return_reset = match &method.sig.output {
            ReturnType::Default => quote! {},
            ReturnType::Type(_, ty) => {
                let returned_times_field = Self::returned_times(&method.sig.ident);
                let returned_field = Self::returned(&method.sig.ident);

                quote! {
                    self.#returned_times_field.clear();
                    self.#returned_field.clear();
                }
            },
        };
        
        quote! {
            self.#expected_calls = None;
            self.#recorded_calls = 0;
            #input_reset
            #return_reset
        }      
    }
}