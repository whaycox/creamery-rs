use super::*;
use std::collections::HashSet;

pub fn field_input_types(inputs: &Vec<Box<Type>>) -> Vec<Box<Type>> {
    let mut input_types: Vec<Box<Type>> = Vec::new();
    for input in inputs {
        match &**input {
            Type::Reference(reference_type) => {
                let value_type = reference_type.elem.clone();
                let modified_reference_type = TypeReference {
                    and_token: Default::default(),
                    lifetime: None,
                    mutability: None,
                    elem: value_type,
                };
                input_types.push(Box::new(Type::Reference(modified_reference_type)));
            },
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

pub fn field_return_type(input: &Type, lifetimes: &HashSet<Ident>) -> Type {
    match input {
        syn::Type::Reference(reference_type) => {
            let value_type = reference_type.elem.clone();
            let lifetime = if let Some(input_lifetime) = &reference_type.lifetime {
                if lifetimes.contains(&input_lifetime.ident) {
                    reference_type.lifetime.clone()
                }
                else { None }
            }
            else { None };
            let modified_reference_type = TypeReference {
                and_token: Default::default(),
                lifetime: lifetime,
                mutability: None,
                elem: value_type,
            };
            syn::Type::Reference(modified_reference_type)
        },
        syn::Type::Path(type_path) => {
            let modified_segments: Punctuated<PathSegment, Colon2> = type_path.path.segments.iter()
                .map(correct_path_segments(lifetimes))
                .collect();
            let modified_path = Path {
                leading_colon: type_path.path.leading_colon.clone(),
                segments: modified_segments,
            };
            syn::Type::Path(TypePath {
                qself: type_path.qself.clone(),
                path: modified_path,
            })
        },
        _ => input.clone(),
    }
}
fn correct_path_segments<'a>(lifetimes: &'a HashSet<Ident>) -> impl Fn(&'a PathSegment) -> PathSegment {
    |segment| {
        match &segment.arguments {
            PathArguments::AngleBracketed(generic_args) => {
                let modified_generic_args: Punctuated<GenericArgument, Comma> = generic_args.args.iter()
                    .map(correct_generic_args(lifetimes))
                    .collect();
                let modified_path_arguments = PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: generic_args.colon2_token.clone(),
                    lt_token: generic_args.lt_token.clone(),
                    args: modified_generic_args,
                    gt_token: generic_args.gt_token.clone(),
                });
                PathSegment {
                    ident: segment.ident.clone(),
                    arguments: modified_path_arguments,
                }
            },
            _ => segment.clone(),
        }
    }
}
fn correct_generic_args<'a>(lifetimes: &'a HashSet<Ident>) -> impl Fn(&'a GenericArgument) -> GenericArgument {
    |arg| {
        match arg {
            GenericArgument::Type(generic_type) => {
                match generic_type {
                    syn::Type::TraitObject(trait_object) => {
                        let modified_bounds: Punctuated<TypeParamBound, Add> = trait_object.bounds.iter()
                            .filter_map(filter_type_params(lifetimes))
                            .collect();
                        let modified_trait_object = syn::Type::TraitObject(TypeTraitObject {
                            dyn_token: trait_object.dyn_token.clone(),
                            bounds: modified_bounds,
                        });
                        GenericArgument::Type(modified_trait_object)
                    },
                    _ => { 
                        GenericArgument::Type(field_return_type(generic_type, lifetimes))
                    },
                }
            },
            _ => arg.clone(),
        }
    }
}
fn filter_type_params<'a>(lifetimes: &'a HashSet<Ident>) -> impl Fn(&'a TypeParamBound) -> Option<TypeParamBound> {
    |bound| {
        if let TypeParamBound::Lifetime(lifetime) = bound {
            if !lifetimes.contains(&lifetime.ident) {
                return None;
            }
            return Some(bound.clone());
        }
        Some(bound.clone())
    }
}

pub fn quote_fields(method: &TraitItemMethod, lifetimes: &HashSet<Ident>) -> Vec<TokenStream> {
    let mut fields = vec![
        quote_expected_calls_field(method),
        quote_recorded_calls_field(method),
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
        fields.push(quote_expected_input_times_field(&method.sig.ident));
        fields.push(quote_expected_input_field(&method.sig.ident, field_input_types(&input_types)));
    }
    match &method.sig.output {
        ReturnType::Default => {},
        ReturnType::Type(_, ty) => {
            fields.push(quote_default_return_field(&method.sig.ident, field_input_types(&input_types), field_return_type(&**ty, lifetimes)));
            fields.push(quote_returned_times_field(&method.sig.ident));
            fields.push(quote_returned_field(&method.sig.ident, field_input_types(&input_types), field_return_type(&**ty, lifetimes)));
        },
    }

    fields
}
fn quote_expected_calls_field(method: &TraitItemMethod) -> TokenStream {
    let expected_calls_field = expected_calls_field(&method.sig.ident);
    quote! {
        #expected_calls_field: std::sync::Mutex<std::option::Option<u32>>
    }
}
fn quote_recorded_calls_field(method: &TraitItemMethod) -> TokenStream {
    let recorded_calls_field = recorded_calls_field(&method.sig.ident);
    quote! {
        #recorded_calls_field: std::sync::Mutex<u32>
    }
}
fn quote_expected_input_times_field(ident: &Ident) -> TokenStream {
    let expected_input_times_field = expected_input_times_field(ident);
    quote! {
        #expected_input_times_field: std::sync::Mutex<std::vec::Vec<u32>>
    }
}
fn quote_expected_input_field(ident: &Ident, input_types: Vec<Box<Type>>) -> TokenStream {
    let expected_input_field = expected_input_field(ident);
    quote! {
        #expected_input_field: std::sync::Mutex<std::vec::Vec<std::boxed::Box<dyn Fn(#(#input_types),*) -> bool + Send + Sync>>>
    }
}
fn quote_default_return_field(ident: &Ident, input_types: Vec<Box<Type>>, returned_type: Type) -> TokenStream {
    let default_generator_field = default_generator_field(ident);

    quote! {
        #default_generator_field: std::sync::Mutex<std::option::Option<std::boxed::Box<dyn Fn(#(#input_types),*) -> #returned_type + Send + Sync>>>
    }
}
fn quote_returned_times_field(ident: &Ident) -> TokenStream {
    let returned_times_field = returned_times_field(ident);

    quote! {
        #returned_times_field: std::sync::Mutex<std::vec::Vec<u32>>
    }
}
fn quote_returned_field(ident: &Ident, input_types: Vec<Box<Type>>, returned_type: Type) -> TokenStream {
    let returned_field = returned_field(ident);

    quote! {
        #returned_field: std::sync::Mutex<std::vec::Vec<std::boxed::Box<dyn Fn(#(#input_types),*) -> #returned_type + Send + Sync>>>
    }
}

pub fn quote_field_initializers(method: &TraitItemMethod) -> Vec<TokenStream> {
    let mut initializers: Vec<TokenStream> = Vec::new();

    let expected_calls = expected_calls_field(&method.sig.ident);
    initializers.push(quote! { #expected_calls: std::default::Default::default() });
    let recorded_calls = recorded_calls_field(&method.sig.ident);
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
        let expected_input_times = expected_input_times_field(&method.sig.ident);
        initializers.push(quote! { #expected_input_times: std::default::Default::default() });
        let expected_input = expected_input_field(&method.sig.ident);
        initializers.push(quote! { #expected_input: std::default::Default::default() });
    }
    match &method.sig.output {
        ReturnType::Default => {},
        ReturnType::Type(_, _) => {
            let default_generator = default_generator_field(&method.sig.ident);
            initializers.push(quote! { #default_generator: std::default::Default::default() });

            let returned_times = returned_times_field(&method.sig.ident);
            initializers.push(quote! { #returned_times: std::default::Default::default() });
            
            let returned = returned_field(&method.sig.ident);
            initializers.push(quote! { #returned: std::default::Default::default() });
        },
    }

    initializers
}