use super::*;

pub const DEFAULT_RETURN_IDENTIFIER: &str = "mock_default_return";

pub struct WheyMock {
    pub mocked_trait: ItemTrait,
    pub defaulted_returns: HashMap<Ident, TokenStream>,
}

impl Parse for WheyMock {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut mocked_trait: ItemTrait = input.parse()?;
        let defaulted_returns = Self::parse_defaulted(&mut mocked_trait)?;
        
        Ok(WheyMock {
            mocked_trait,
            defaulted_returns,
        })
    }
}

impl WheyMock {
    pub fn testing_name(ident: &Ident) -> Ident { format_ident!("Testing{}", ident) }
    pub fn expect_calls(ident: &Ident) -> Ident { format_ident!("expect_calls_{}", ident) }
    pub fn store_expected_input(ident: &Ident) -> Ident { format_ident!("store_expected_input_{}", ident) }
    pub fn default_return(ident: &Ident) -> Ident { format_ident!("default_return_{}", ident) }
    pub fn store_return(ident: &Ident) -> Ident { format_ident!("store_return_{}", ident) }
    
    fn expected_calls(ident: &Ident) -> Ident { format_ident!("expected_calls_{}", ident) }
    fn recorded_calls(ident: &Ident) -> Ident { format_ident!("recorded_calls_{}", ident) }
    fn default_generator(ident: &Ident) -> Ident { format_ident!("default_generator_{}", ident) }
    fn expected_input(ident: &Ident) -> Ident { format_ident!("expected_input_{}", ident) }
    fn expected_input_times(ident: &Ident) -> Ident { format_ident!("expected_input_times_{}", ident) }
    fn returned(ident: &Ident) -> Ident { format_ident!("returned_{}", ident) }
    fn returned_times(ident: &Ident) -> Ident { format_ident!("returned_times_{}", ident) }

    fn filter_items(item: &TraitItem) -> Option<&TraitItemMethod> {
        match item {
            TraitItem::Method(method) => Some(method),
            _ => None,
        }
    }

    fn parse_defaulted(item: &mut ItemTrait) -> Result<HashMap<Ident, TokenStream>> {
        let mut default_return: HashMap<Ident, TokenStream> = HashMap::new();
        for method in &mut item.items {
            match method {
                TraitItem::Method(trait_method) => {
                    let length = trait_method.attrs.len();
                    if length > 0 {
                        let mut attribute_index = 0;
                        while attribute_index < length {
                            let attribute = &trait_method.attrs[attribute_index];
                            if attribute.path.is_ident(DEFAULT_RETURN_IDENTIFIER) {
                                let ident = trait_method.sig.ident.clone();
                                let mut default_value = quote! { Some(std::boxed::Box::new(|| std::default::Default::default())) };
                                if !attribute.tokens.is_empty() {
                                    let generator: WheyExpectation = attribute.parse_args()?;
                                    default_value = quote! { Some(std::boxed::Box::new(#generator)) };
                                }
                                
                                default_return.insert(ident, default_value);
                                trait_method.attrs.remove(attribute_index);
                                break;
                            }

                            attribute_index = attribute_index + 1;
                        }
                    }
                },
                _ => panic!("Only named fields are supported"),
            }
        }

        Ok(default_return)
    }

    fn input_types(inputs: &Vec<Box<Type>>) -> Vec<Box<Type>> {
        let mut input_types: Vec<Box<Type>> = Vec::new();
        for input in inputs {
            match &**input {
                Type::Reference(_) => input_types.push(input.clone()),
                _ => {
                    let value_type = *input.clone();
                    let type_reference = TypeReference {
                        and_token: Default::default(),
                        lifetime: None,
                        mutability: None,
                        elem: Box::new(value_type),
                    };
                    input_types.push(Box::new(Type::Reference(type_reference)));
                },
            }
        }

        input_types
    }

    pub fn quote(self) -> TokenStream {
        let mocked_trait = &self.mocked_trait;
        let testing_mock = self.quote_testing_mock(&mocked_trait);

        quote! {
            #mocked_trait
            #testing_mock
        }
    }
    fn quote_testing_mock(&self, mocked_trait: &ItemTrait) -> TokenStream {
        let vis = &mocked_trait.vis;
        let base_name = &mocked_trait.ident;
        let testing_name = WheyMock::testing_name(&mocked_trait.ident);
        let generics = &mocked_trait.generics;
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        let mocked_items: Vec<&TraitItemMethod> = mocked_trait.items
            .iter()
            .filter_map(|item| Self::filter_items(item))
            .collect();
        let fields: Vec<TokenStream> = mocked_items
            .iter()
            .flat_map(|item| self.quote_fields(item))
            .collect();
        let initializers: Vec<TokenStream> = mocked_items
            .iter()
            .flat_map(|item| self.quote_field_initializers(item))
            .collect();
        let mocked_impls: Vec<TokenStream> = mocked_items
            .iter()
            .map(|item| self.quote_impl(item))
            .collect();
        let setup_expectations: Vec<TokenStream> = mocked_items
            .iter()
            .flat_map(|item| self.quote_setup_expectations(item))
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
            #vis struct #testing_name #generics {
                #(#fields),*
            }

            impl #impl_generics #testing_name #type_generics #where_clause {
                pub fn new() -> Self {
                    Self {
                        #(#initializers),*
                    }
                }

                #(#setup_expectations)*

                pub fn assert(&self) {
                    #(#assert_expectations)*
                    self.reset();
                }

                pub fn reset(&self) {
                    #(#reset_expectations)*
                }
            }

            impl #impl_generics Drop for #testing_name #type_generics #where_clause {
                fn drop(&mut self) {
                    if !std::thread::panicking() {
                        self.assert();
                    }
                }
            }

            impl #impl_generics #base_name #generics for #testing_name #type_generics #where_clause {
                #(#mocked_impls)*
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
            fields.push(self.quote_expected_input_field(&method.sig.ident, Self::input_types(&input_types)));
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
            #expected_calls_field: std::cell::RefCell<std::option::Option<u32>>
        }
    }
    fn quote_recorded_calls_field(&self, method: &TraitItemMethod) -> TokenStream {
        let recorded_calls_field = Self::recorded_calls(&method.sig.ident);
        quote! {
            #recorded_calls_field: std::cell::RefCell<u32>
        }
    }
    fn quote_expected_input_times_field(&self, ident: &Ident) -> TokenStream {
        let expected_input_times_field = Self::expected_input_times(ident);
        quote! {
            #expected_input_times_field: std::cell::RefCell<std::vec::Vec<u32>>
        }
    }
    fn quote_expected_input_field(&self, ident: &Ident, input_types: Vec<Box<Type>>) -> TokenStream {
        let expected_input_field = Self::expected_input(ident);
        quote! {
            #expected_input_field: std::cell::RefCell<std::vec::Vec<std::boxed::Box<dyn Fn(#(#input_types),*) -> bool>>>
        }
    }
    fn quote_default_return_field(&self, ident: &Ident, input_types: &Vec<Box<Type>>, returned_type: &Box<Type>) -> TokenStream {
        let default_generator_field = Self::default_generator(ident);

        quote! {
            #default_generator_field: std::cell::RefCell<std::option::Option<std::boxed::Box<dyn Fn(#(#input_types),*) -> #returned_type>>>
        }
    }
    fn quote_returned_times_field(&self, ident: &Ident) -> TokenStream {
        let returned_times_field = Self::returned_times(ident);

        quote! {
            #returned_times_field: std::cell::RefCell<std::vec::Vec<u32>>
        }
    }
    fn quote_returned_field(&self, ident: &Ident, input_types: &Vec<Box<Type>>, returned_type: &Box<Type>) -> TokenStream {
        let returned_field = Self::returned(ident);

        quote! {
            #returned_field: std::cell::RefCell<std::vec::Vec<std::boxed::Box<dyn Fn(#(#input_types),*) -> #returned_type>>>
        }
    }

    fn quote_field_initializers(&self, method: &TraitItemMethod) -> Vec<TokenStream> {
        let mut initializers: Vec<TokenStream> = Vec::new();

        let expected_calls = Self::expected_calls(&method.sig.ident);
        initializers.push(quote! { #expected_calls: std::default::Default::default() });
        let recorded_calls = Self::recorded_calls(&method.sig.ident);
        initializers.push(quote! { #recorded_calls: std::default::Default::default() });

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
            let expected_input_times = Self::expected_input_times(&method.sig.ident);
            initializers.push(quote! { #expected_input_times: std::default::Default::default() });
            let expected_input = Self::expected_input(&method.sig.ident);
            initializers.push(quote! { #expected_input: std::default::Default::default() });
        }
        match &method.sig.output {
            ReturnType::Default => {},
            ReturnType::Type(_, ty) => {
                let default_generator = Self::default_generator(&method.sig.ident);
                initializers.push(quote! { #default_generator: std::default::Default::default() });

                let returned_times = Self::returned_times(&method.sig.ident);
                initializers.push(quote! { #returned_times: std::default::Default::default() });
                
                let returned = Self::returned(&method.sig.ident);
                initializers.push(quote! { #returned: std::default::Default::default() });
            },
        }

        initializers
    }

    fn quote_impl(&self, method: &TraitItemMethod) -> TokenStream {
        let signature = &method.sig;
        let mut input_names: Vec<&Box<Pat>> = Vec::new();
        for input in &method.sig.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => input_names.push(&ty.pat),
            }
        }

        let compare_input = if input_names.len() > 0 {
            let mut comparer_names: Vec<TokenStream> = Vec::new();
            for input in &method.sig.inputs {
                match input {
                    FnArg::Receiver(_) => {},
                    FnArg::Typed(ty) => {
                        let name = &ty.pat;
                        match *ty.ty {
                            Type::Reference(_) => comparer_names.push(quote! { #name }),
                            _ => comparer_names.push(quote! { &#name }),
                        }
                    }
                }
            }
            let expected_input_times_field = Self::expected_input_times(&method.sig.ident);
            let expected_input_field = Self::expected_input(&method.sig.ident);
            let expected_input_failure = format!("the expected inputs for {}::{} were not supplied", self.mocked_trait.ident, method.sig.ident);

            quote! {
                let length = self.#expected_input_times_field.borrow().len();
                for i in 0..length {
                    if self.#expected_input_times_field.borrow()[i] > 0 {
                        self.#expected_input_times_field.borrow_mut()[i] -= 1;
                        if !(self.#expected_input_field.borrow()[i])(#(#comparer_names),*) {
                            panic!(#expected_input_failure);
                        }
                        break;
                    }
                }
            }
        }
        else { quote! {} };
        
        let generate_return = match &method.sig.output {
            ReturnType::Default => quote! {},
            ReturnType::Type(_, _) => {
                let returned_times_field = Self::returned_times(&method.sig.ident);
                let returned_field = Self::returned(&method.sig.ident);
                let default_generator_field = Self::default_generator(&method.sig.ident);
                let no_return_failure = format!("a return is necessary for {}::{} but none have been supplied", self.mocked_trait.ident, method.sig.ident);

                quote! {
                    let length = self.#returned_times_field.borrow().len();
                    for i in 0..length {
                        if self.#returned_times_field.borrow()[i] > 0 {
                            self.#returned_times_field.borrow_mut()[i] -= 1;
                            return (self.#returned_field.borrow()[i])(#(#input_names),*)
                        }
                    }
                    match &*self.#default_generator_field.borrow() {
                        Some(generator) => return generator(#(#input_names),*),
                        _ => panic!(#no_return_failure),
                    }
                }
            },
        };

        let recorded_calls = Self::recorded_calls(&method.sig.ident);
        quote! {
            #signature {
                *self.#recorded_calls.borrow_mut() += 1;
                #compare_input
                #generate_return
            }
        }
    }
    
    fn quote_setup_expectations(&self, method: &TraitItemMethod) -> Vec<TokenStream> {
        let mut setup_expectations = vec![
            self.quote_expect_calls(method),
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
            setup_expectations.push(self.quote_store_expected_input(&method.sig.ident, Self::input_types(&input_types)));
        }
        match &method.sig.output {
            ReturnType::Default => {},
            ReturnType::Type(_, ty) => {
                setup_expectations.push(self.quote_default_return(&method, &input_types, ty));
                setup_expectations.push(self.quote_store_return(&method, &input_types, ty));
            },
        }

        setup_expectations    
    }
    fn quote_expect_calls(&self, method: &TraitItemMethod) -> TokenStream {
        let expect_calls = Self::expect_calls(&method.sig.ident);
        let expected_calls = Self::expected_calls(&method.sig.ident);
        quote! {
            pub fn #expect_calls(&self, expected: u32) {
                *self.#expected_calls.borrow_mut() = Some(expected);
            }
        }
    }
    fn quote_store_expected_input(&self, ident: &Ident, input_types: Vec<Box<Type>>) -> TokenStream {
        let store_input = Self::store_expected_input(ident);
        let expected_input_times_field = Self::expected_input_times(ident);
        let expected_input_field = Self::expected_input(ident);

        quote! {
            pub fn #store_input<TComparer: 'static + Fn(#(#input_types),*) -> bool>(&self, comparison: TComparer, times: u32) {
                self.#expected_input_times_field.borrow_mut().push(times);
                self.#expected_input_field.borrow_mut().push(std::boxed::Box::new(comparison));
            }
        }
    }
    fn quote_default_return(&self, method: &TraitItemMethod, input_types: &Vec<Box<Type>>, returned_type: &Box<Type>) -> TokenStream {
        let default_return = Self::default_return(&method.sig.ident);
        let default_generator_field = Self::default_generator(&method.sig.ident);

        quote! {
            pub fn #default_return<TGenerator: 'static + Fn(#(#input_types),*) -> #returned_type>(&self, generator: TGenerator) {
                *self.#default_generator_field.borrow_mut() = Some(std::boxed::Box::new(generator));
            }
        }
    }
    fn quote_store_return(&self, method: &TraitItemMethod, input_types: &Vec<Box<Type>>, returned_type: &Box<Type>) -> TokenStream {
        let store_return = Self::store_return(&method.sig.ident);
        let returned_times_field = Self::returned_times(&method.sig.ident);
        let returned_field = Self::returned(&method.sig.ident);

        quote! {
            pub fn #store_return<TGenerator: 'static + Fn(#(#input_types),*) -> #returned_type>(&self, generator: TGenerator, times: u32) {
                self.#returned_times_field.borrow_mut().push(times);
                self.#returned_field.borrow_mut().push(std::boxed::Box::new(generator));
            }
        }
    }

    fn quote_assert_expectations(&self, method: &TraitItemMethod) -> TokenStream {
        let expected_calls = Self::expected_calls(&method.sig.ident);
        let recorded_calls = Self::recorded_calls(&method.sig.ident);
        let expected_calls_failure = format!("expected {{}} calls to {}::{} but recorded {{}} instead", self.mocked_trait.ident, method.sig.ident);
        
        let mut has_inputs = false;
        for input in &method.sig.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(_) => {
                    has_inputs = true;
                    break;
                },
            }
        }
        let input_assert = if has_inputs {
            let expected_input_times_field = Self::expected_input_times(&method.sig.ident);
            let expected_input_failure = format!("not all stored input comparisons for {}::{} have been consumed", self.mocked_trait.ident, method.sig.ident);

            quote! {
                if self.#expected_input_times_field.borrow().iter().any(|comparer| *comparer != 0) {
                    panic!(#expected_input_failure);
                }
            }
        }
        else { quote! {} };
        let return_assert = match &method.sig.output {
            ReturnType::Default => quote! {},
            ReturnType::Type(_, _) => {
                let returned_times_field = Self::returned_times(&method.sig.ident);
                let returned_failure = format!("not all stored returns for {}::{} have been consumed", self.mocked_trait.ident, method.sig.ident);

                quote! {
                    if self.#returned_times_field.borrow().iter().any(|generator| *generator != 0) {
                        panic!(#returned_failure);
                    }
                }
            },
        };
        
        quote! {
            if self.#expected_calls.borrow().is_some() && self.#expected_calls.borrow().unwrap() != *self.#recorded_calls.borrow() {
                panic!(#expected_calls_failure, self.#expected_calls.borrow().unwrap(), self.#recorded_calls.borrow());
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
                FnArg::Typed(_) => {
                    has_inputs = true;
                    break;
                },
            }
        }
        let input_reset = if has_inputs {
            let expected_input_times_field = Self::expected_input_times(&method.sig.ident);
            let expected_input_field = Self::expected_input(&method.sig.ident);

            quote! {
                self.#expected_input_times_field.borrow_mut().clear();
                self.#expected_input_field.borrow_mut().clear();
            }
        }
        else { quote! {} };
        let return_reset = match &method.sig.output {
            ReturnType::Default => quote! {},
            ReturnType::Type(_, _) => {
                let returned_times_field = Self::returned_times(&method.sig.ident);
                let returned_field = Self::returned(&method.sig.ident);

                quote! {
                    self.#returned_times_field.borrow_mut().clear();
                    self.#returned_field.borrow_mut().clear();
                }
            },
        };
        
        quote! {
            *self.#expected_calls.borrow_mut() = None;
            *self.#recorded_calls.borrow_mut() = 0;
            #input_reset
            #return_reset
        }      
    }
}
