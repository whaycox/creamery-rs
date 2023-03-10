use super::*;

pub struct WheyMockExpectation {}

impl  WheyMockExpectation {
    pub fn expectation_name(ident: &Ident, core_name: &Ident) -> Ident { format_ident!("{}_{}_Expectation", core_name, ident) }

    pub fn parse_expectation_input_types(signature: &Signature) -> Option<Vec<Box<Type>>> {
        let mut input_types: Vec<Box<Type>> = Vec::new();
        for input in &signature.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => match &*ty.ty {
                    Type::Reference(ref_type) => {
                        let value_type = Type::from(*ref_type.elem.clone());
                        input_types.push(Box::new(value_type));
                    },
                    _ => input_types.push(ty.ty.clone()),
                },
            }
        }
        if input_types.len() > 0 {
            Some(input_types)
        }
        else {
            None
        }
    }
    pub fn parse_consume_input_types(signature: &Signature) -> Option<Vec<Box<Type>>> {
        let mut input_types: Vec<Box<Type>> = Vec::new();
        for input in &signature.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => match &*ty.ty {
                    Type::Reference(reference_type) => match reference_type.mutability {
                        Some(_) => {
                            let mut immutable_type = reference_type.clone();
                            immutable_type = TypeReference {
                                and_token: immutable_type.and_token,
                                lifetime: immutable_type.lifetime,
                                mutability: None,
                                elem: immutable_type.elem,
                            };
                            input_types.push(Box::new(Type::Reference(immutable_type)))
                        },
                        None => input_types.push(ty.ty.clone()),
                    }
                    _ => input_types.push(ty.ty.clone()),
                },
            }
        }
        if input_types.len() > 0 {
            Some(input_types)
        }
        else {
            None
        }
    }
    pub fn parse_expectation_return_type(signature: &Signature) -> Option<Type> {
        match &signature.output {
            ReturnType::Default => None,
            ReturnType::Type(_, ty) => {
                let mut return_type = *ty.clone();
                match &return_type {
                    Type::Reference(ref_type) => return_type = Type::from(*ref_type.elem.clone()),
                    _ => {},
                }
                Some(return_type)
            }
        }
    }

    pub fn quote_struct(item: &TraitItemMethod, core_name: &Ident) -> TokenStream {
        let name = Self::expectation_name(&item.sig.ident, core_name);
        let fields = Self::quote_fields(item);
        let consume = Self::quote_consume(item);

        quote! {
            #[allow(non_camel_case_types)]
            struct #name {
                #(#fields),*
            }

            impl #name {
                pub fn is_consumed(&self) -> bool { self.times.get() == 0 }
                #consume
            }
        }
    }
    fn quote_fields(method: &TraitItemMethod) -> Vec<TokenStream> {
        let mut fields: Vec<TokenStream> = Vec::new();
        fields.push(quote! { times: std::cell::Cell<u32> });
        Self::add_input_comparison_field(method, &mut fields);
        Self::add_expectation_return_field(method, &mut fields);

        fields
    }
    fn add_input_comparison_field(method: &TraitItemMethod, fields: &mut Vec<TokenStream>) {
        match Self::parse_expectation_input_types(&method.sig) {
            Some(inputs) => fields.push(quote! { input_comparison: (#(#inputs),*) }),
            None => {},
        }
    }
    fn add_expectation_return_field(method: &TraitItemMethod, fields: &mut Vec<TokenStream>) {
        match Self::parse_expectation_return_type(&method.sig) {
            Some(expected_return) => fields.push(quote! { expected_return: #expected_return }),
            None => {},
        }
    }

    fn quote_consume(method: &TraitItemMethod) -> TokenStream {
        let mut consume_signature = vec![];
        match &method.sig.inputs.first().unwrap() {
            FnArg::Receiver(receiver) => match &receiver.mutability {
                Some(_) => consume_signature.push(quote! { &mut self }),
                None => consume_signature.push(quote! { &self }),
            },
            _ => {},
        }
        let mut impl_tokens: Vec<TokenStream> = Vec::new();
        match Self::parse_consume_input_types(&method.sig) {
            Some(types) => {
                consume_signature.push(quote! { expected: (#(#types),*) });
                let mut comparison: Vec<TokenStream> = Vec::new();
                if types.len() > 1 {
                    for i in 0..types.len() {
                        let index = syn::Index::from(i);
                        match &*types[i] {
                            Type::Path(_) => comparison.push(quote! { self.input_comparison.#index }),
                            Type::Reference(_) => comparison.push(quote! { &self.input_comparison.#index }),
                            _ => {},
                        }
                    }
                }
                else {
                    match *types[0] {
                        Type::Path(_) => comparison.push(quote! { self.input_comparison }),
                        Type::Reference(_) => comparison.push(quote! { &self.input_comparison }),
                        _ => {},
                    }
                }
                impl_tokens.push(quote! { let comparison = (#(#comparison),*); });
                impl_tokens.push(quote! { assert_eq!(comparison, expected); });
            },
            None => {},
        }
        impl_tokens.push(quote! { self.times.set(self.times.get() - 1); });
        let mut return_type: Option<TokenStream> = None;
        match &method.sig.output {
            ReturnType::Default => {},
            ReturnType::Type(_, ty) => {
                return_type = Some(quote! { -> #ty });
                match &**ty {
                    Type::Path(_) => impl_tokens.push(quote! { self.expected_return.clone() }),
                    Type::Reference(reference_type) => match &reference_type.mutability {
                        Some(_) => impl_tokens.push(quote! { &mut self.expected_return }),
                        None => impl_tokens.push(quote! { &self.expected_return }),
                    },
                    _ => {},
                }
            },
        };

        quote! {
            pub fn consume(#(#consume_signature),*) #return_type {
                #(#impl_tokens)*
            }
        }
    }
}