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
        let mut fields: Vec<TokenStream> = mocked_items
            .iter()
            .flat_map(|item| self.quote_fields(item))
            .collect();
        fields.insert(0, self.quote_synchronizer_field());
        let impls: Vec<TokenStream> = mocked_items
            .iter()
            .flat_map(|item| self.quote_impls(item))
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
            impls.push(self.quote_store_expected_input(&method.sig.ident, Self::input_types(&input_types)));
            impls.push(self.quote_consume_expected_input(&method));
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

    fn quote_store_expected_input(&self, ident: &Ident, input_types: Vec<Box<Type>>) -> TokenStream {
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
    fn quote_consume_expected_input(&self, method: &TraitItemMethod) -> TokenStream {
        let consume_input = Self::consume_expected_input(&method.sig.ident);
        let mut signature_inputs: Vec<TokenStream> = vec![ quote! { &mut self } ];
        let mut input_names: Vec<&Box<Pat>> = Vec::new();
        for input in &method.sig.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => match &*ty.ty {
                    Type::Reference(_) => {
                        signature_inputs.push(quote! { #input });
                        input_names.push(&ty.pat);
                    },
                    _ => {
                        let value_type = *ty.ty.clone();
                        let type_reference = TypeReference {
                            and_token: Default::default(),
                            lifetime: None,
                            mutability: None,
                            elem: Box::new(value_type),
                        };
                        let name = &ty.pat;

                        signature_inputs.push(quote! { #name: #type_reference });
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
                        break;
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
        let no_return_failure = format!("a return is necessary for {}::{} but none have been supplied", self.mock.mocked_trait.ident, method.sig.ident);

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
                    _ => panic!(#no_return_failure),
                }
            }
        }
    }
}