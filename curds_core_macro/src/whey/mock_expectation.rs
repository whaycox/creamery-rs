use super::*;

pub struct WheyMockExpectation {}

impl  WheyMockExpectation {
    pub fn expectation_name(ident: &Ident) -> Ident { format_ident!("WheyCore_{}_Expectation", ident) }

    pub fn parse_expectation_input_signature(signature: &Signature) -> Vec<PatType> {
        let mut input_signature: Vec<PatType> = Vec::new();
        for input in &signature.inputs {
            match input {
                FnArg::Receiver(_) => {},
                FnArg::Typed(ty) => match &*ty.ty {
                    Type::Reference(ref_type) => {
                        let value_type = Type::from(*ref_type.elem.clone());
                        input_signature.push(PatType { 
                            attrs: Vec::new(),
                            pat: ty.pat.clone(), 
                            colon_token: Default::default(), 
                            ty: Box::new(value_type), 
                        });
                    },
                    _ => input_signature.push(ty.clone()),
                },
            }
        }
        input_signature
    } 
    pub fn parse_expectation_input_types(signature: &Signature) -> Vec<Box<Type>> {
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
        input_types
    }

    pub fn quote_struct(item: &TraitItemMethod) -> TokenStream {
        let name = Self::expectation_name(&item.sig.ident);
        let input_comparison = Self::quote_input_comparison_field(item);
        let expected_return = Self::quote_expectation_return_field(item);
        let consume = Self::quote_consume(item);

        quote! {
            #[allow(non_camel_case_types)]
            pub struct #name {
                times: u32,
                #input_comparison,
                #expected_return
            }

            impl #name {
                pub fn is_consumed(&self) -> bool { self.times == 0 }
                #consume
            }
        }
    }
    fn quote_input_comparison_field(method: &TraitItemMethod) -> TokenStream {
        let input_signature = Self::parse_expectation_input_types(&method.sig);

        quote! {
            input_comparison: (#(#input_signature),*)
        }
    }
    fn quote_expectation_return_field(method: &TraitItemMethod) -> TokenStream {
        match &method.sig.output {
            ReturnType::Default => quote! { },
            ReturnType::Type(_, ty) => {
                let mut return_type = *ty.clone();
                match &return_type {
                    Type::Reference(ref_type) => return_type = Type::from(*ref_type.elem.clone()),
                    _ => {},
                }
                quote! {
                    expected_return: #return_type
                }
            }
        }
    }
    fn quote_consume(method: &TraitItemMethod) -> TokenStream {
        let input_signature = Self::parse_expectation_input_types(&method.sig);

        quote! {
            pub fn consume(&mut self, expected: (#(#input_signature),*)) {
                todo!("consume todo")
            }
        }
    }
}