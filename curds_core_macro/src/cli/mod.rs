use super::*;

mod argument_definition;
mod enum_argument_definition;
mod enum_descriptions;
mod struct_argument_definition;

pub use argument_definition::*;
pub use enum_argument_definition::*;
pub use enum_descriptions::*;
pub use struct_argument_definition::*;

use proc_macro2::{TokenStream, Span};
use std::sync::OnceLock;
use regex::Regex;

const DESCRIPTION_IDENTIFIER: &str = "description";

static CAMEL_CASE_SPLITTER: OnceLock<Regex> = OnceLock::new();
fn camel_case_splitter() -> &'static Regex { CAMEL_CASE_SPLITTER.get_or_init(|| Regex::new("[A-Z][a-z]*").unwrap()) }

fn format_argument_name(name: &Ident) -> Ident {
    let splitter = camel_case_splitter();
    let mut name_string = name.to_string();
    let mut parts: Vec<String> = vec![];
    while let Some(part) = splitter.find(&name_string) {
        let mut part: String = name_string.drain(part.start()..part.len()).collect();
        if let Some(char) = part.get_mut(0..1) {
            char.make_ascii_lowercase();
        }
        parts.push(part);
    }
    
    Ident::new(&parts.join("_"), name.span())
}

fn type_is_bool(ty: &Type) -> bool {
    if let Type::Path(path) = ty {
        let last_segment = &path.path.segments[path.path.segments.len() - 1];
        return last_segment.ident == "bool" && last_segment.arguments == PathArguments::None;
    }
    false
}
fn type_is_option(ty: &Type) -> bool {
    if let Type::Path(path) = ty {
        let last_segment = &path.path.segments[path.path.segments.len() - 1];
        if let PathArguments::AngleBracketed(arguments) = &last_segment.arguments {
            return last_segment.ident == "Option" && arguments.args.len() == 1;
        }
    }
    false
}
fn type_is_vec(ty: &Type) -> bool {
    if let Type::Path(path) = ty {
        let last_segment = &path.path.segments[path.path.segments.len() - 1];
        if let PathArguments::AngleBracketed(arguments) = &last_segment.arguments {
            return last_segment.ident == "Vec" && arguments.args.len() == 1;
        }
    }
    false
}
fn extract_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(path) = ty {
        let last_segment = &path.path.segments[path.path.segments.len() - 1];
        if let PathArguments::AngleBracketed(arguments) = &last_segment.arguments {
            if arguments.args.len() == 1 {
                let argument = &arguments.args[0];
                if let GenericArgument::Type(option_type) = argument {
                    return Some(option_type);
                }
            }
        }
    }
    return None;
}

fn parse_fields(type_name: TokenStream, fields: &Fields) -> TokenStream {
    let crate_name = resolve_crate_name();
    match fields {
        Fields::Unit => quote! { Ok(#type_name) },
        Fields::Unnamed(fields) => {
            let mut unnamed_fields: Vec<TokenStream> = vec![];
            for field in &fields.unnamed {
                let ty = &field.ty;
                let is_vec = type_is_vec(ty);
                let inner_type = extract_inner_type(ty);

                unnamed_fields.push(if is_vec {
                    quote! {
                        {
                            let mut parsed_vector: Vec<#inner_type> = Vec::new();
                            loop {
                                if arguments.len() > 0 {
                                    let value = arguments.pop().unwrap();
                                    if value == "--" {
                                        break;
                                    }
                                    else {
                                        arguments.push(value);
                                        parsed_vector.push(<#inner_type as #crate_name::cli::CliArgumentParse>::parse(arguments)?);
                                    }
                                } 
                                else {
                                    break;
                                }
                            }
                            parsed_vector
                        }
                    }
                }
                else {
                    quote! { <#ty as #crate_name::cli::CliArgumentParse>::parse(arguments)? }
                });
            }
            let argument_data = quote! { (#(#unnamed_fields),*) };

            quote! { Ok(#type_name #argument_data) }
        },
        Fields::Named(fields) => {
            let mut expected_keys: Vec<TokenStream> = vec![];
            let mut key_parsers: Vec<TokenStream> = vec![];
            let mut field_initializers: Vec<TokenStream> = vec![];
        
            for field in &fields.named {
                let name = field.ident.as_ref().unwrap();
                let formatted_name = format!("-{}", name);
                let ty = &field.ty;

                let is_bool = type_is_bool(ty);
                let is_option = type_is_option(ty);
                let mut is_vec = type_is_vec(ty);
                let mut inner_type = extract_inner_type(ty);
                if is_option {
                    let optional_type = inner_type.unwrap();
                    is_vec = type_is_vec(optional_type);
                    if is_vec {
                        inner_type = extract_inner_type(optional_type);
                    }
                }
        
                expected_keys.push(quote! { #formatted_name });

                let pre_parse = if is_vec {
                    quote! {
                        let mut parsed_vector: Vec<#inner_type> = Vec::new();
                        loop {
                            if arguments.len() > 0 {
                                let value = arguments.pop().unwrap();
                                if value == "--" {
                                    break;
                                }
                                else {
                                    arguments.push(value);
                                    parsed_vector.push(<#inner_type as #crate_name::cli::CliArgumentParse>::parse(arguments)?);
                                }
                            } 
                            else {
                                break;
                            }
                        }
                    }
                }
                else { quote! {} };
                let parse = if is_bool {
                    quote! { true }
                }
                else if is_option && is_vec {
                    quote! { Some(parsed_vector) }
                }
                else if is_option {
                    quote! { Some(<#inner_type as #crate_name::cli::CliArgumentParse>::parse(arguments)?) }
                }
                else if is_vec {
                    quote! { parsed_vector }
                }
                else {
                    quote! { <#ty as #crate_name::cli::CliArgumentParse>::parse(arguments)? }
                };
                key_parsers.push(quote! {
                    #formatted_name => {
                        #pre_parse
                        argument_map.insert(#formatted_name, std::boxed::Box::new(#parse));
                        expected_keys.remove(#formatted_name);
                    },
                });

                let value_not_parsed = if is_bool {
                    quote! { false }
                }
                else if is_option {
                    quote! { None }
                }
                else {
                    quote! { return Err(#crate_name::cli::CliArgumentParseError::UnrecognizedKey(#formatted_name.to_string())) }
                };
                field_initializers.push(quote! {
                    #name: match argument_map.remove(#formatted_name) {
                        Some(value) => *value.downcast::<#ty>().unwrap(),
                        None => #value_not_parsed,
                    },
                });
            }
        
            quote! {
                let mut expected_keys: std::collections::HashSet<&str> = vec![#(#expected_keys),*].into_iter().collect();
                let mut argument_map: std::collections::HashMap<&str, Box<dyn std::any::Any>> = std::collections::HashMap::new();
                loop {
                    if arguments.len() > 0 {
                        let value_key = arguments.pop().unwrap();
                        match value_key.as_str() {
                            #(#key_parsers)*
                            _ => { 
                                arguments.push(value_key);
                                break;
                            },
                        }
                        if expected_keys.is_empty() {
                            break;
                        }
                    }
                    else {
                        break;
                    }
                }
        
                Ok(#type_name {
                    #(#field_initializers)*
                })
            }
        },
    }
}

fn field_usage(argument: Option<String>, fields: &Fields) -> TokenStream {
    let crate_name = resolve_crate_name();
    let mut field_usage: Vec<TokenStream> = vec![];
    if argument.is_some() {
        field_usage.push(quote! { #argument.to_string() });
    }
    match fields {
        Fields::Unit => {},
        Fields::Unnamed(fields) => {
            for field in &fields.unnamed {
                let ty = &field.ty;
                let is_vec = type_is_vec(ty);
                let inner_type = extract_inner_type(ty);

                field_usage.push(if is_vec {
                    quote! { 
                        vec![
                            format!("{}*", <#inner_type as #crate_name::cli::CliArgumentParse>::usage()),
                            "--".to_string(),
                        ].join(" ")
                    }
                }
                else {
                    quote! { <#ty as #crate_name::cli::CliArgumentParse>::usage() }
                });
            }
        },
        Fields::Named(fields) => {
            for field in &fields.named {
                let name = field.ident.as_ref().unwrap();
                let formatted_name = format!("-{}", name);
                let ty = &field.ty;

                let is_bool = type_is_bool(ty);
                let is_option = type_is_option(ty);
                let mut is_vec = type_is_vec(ty);
                let mut inner_type = extract_inner_type(ty);
                if is_option {
                    let optional_type = inner_type.unwrap();
                    is_vec = type_is_vec(optional_type);
                    if is_vec {
                        inner_type = extract_inner_type(optional_type);
                    }
                }

                if is_bool {
                    field_usage.push(quote! { format!("[{}]", #formatted_name.to_string()) });
                }
                else if is_option && is_vec {
                    field_usage.push(quote! { 
                        format!("[{}]", vec![
                            #formatted_name.to_string(),
                            format!("{}*", <#inner_type as #crate_name::cli::CliArgumentParse>::usage()),
                            "--".to_string(),
                        ].join(" "))
                    });
                }
                else if is_option {
                    field_usage.push(quote! { 
                        format!("[{}]", vec![
                            #formatted_name.to_string(),
                            <#inner_type as #crate_name::cli::CliArgumentParse>::usage(),
                        ].join(" "))
                    });
                }
                else if is_vec {
                    field_usage.push(quote! { 
                        vec![
                            #formatted_name.to_string(),
                            format!("{}*", <#inner_type as #crate_name::cli::CliArgumentParse>::usage()),
                            "--".to_string(),
                        ].join(" ")
                    });
                }
                else {
                    field_usage.push(quote! { #formatted_name.to_string() });
                    field_usage.push(quote! { <#ty as #crate_name::cli::CliArgumentParse>::usage() });
                }
            }
        },
    }
    
    let formatted_usage = match argument {
        Some(_) => quote! { format!("[{}]", usage.join(" ")) },
        None => quote! { usage.join(" ") },
    };
    quote! {
        {
            let usage: Vec<String> = vec![
                #(#field_usage),*
            ];

            usages.push(#formatted_usage);
        }
    }
}