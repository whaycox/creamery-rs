use super::*;

mod argument_definition;
mod enum_argument_definition;
mod struct_argument_definition;

pub use argument_definition::*;
pub use enum_argument_definition::*;
pub use struct_argument_definition::*;

use proc_macro2::{TokenStream, Span};
use std::sync::OnceLock;
use regex::Regex;

static CAMEL_CASE_SPLITTER: OnceLock<Regex> = OnceLock::new();
fn format_argument_name(name: &Ident) -> Ident {
    let splitter = CAMEL_CASE_SPLITTER.get_or_init(|| Regex::new("[A-Z][a-z]*").unwrap());
    let name_string = name.to_string();
    let parts: Vec<String> = splitter.find_iter(&name_string)
        .map(|part| {
            let mut part_string = part.as_str().to_owned();
            if let Some(char) = part_string.get_mut(0..1) {
                char.make_ascii_lowercase();
            }

            part_string
        })
        .collect();
    
    Ident::new(&parts.join("_"), name.span())
}

fn parse_fields(type_name: TokenStream, fields: &Fields) -> TokenStream {
    match fields {
        Fields::Unit => quote! { Ok(#type_name) },
        Fields::Unnamed(fields) => {
            let mut unnamed_fields: Vec<TokenStream> = vec![];
            for field in &fields.unnamed {
                let ty = &field.ty;
                unnamed_fields.push(quote! { <#ty as curds_core_abstraction::cli::CliArgumentParse>::parse(arguments)? })
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

                let mut is_bool = false;
                let mut inner_type: Option<&Type> = None;
                let mut is_option = false;
                let mut is_vec = false;
                if let Type::Path(path) = ty {
                    let last_segment = &path.path.segments[path.path.segments.len() - 1];
                    is_bool = last_segment.ident == "bool" && last_segment.arguments == PathArguments::None;
                    if let PathArguments::AngleBracketed(arguments) = &last_segment.arguments {
                        if arguments.args.len() == 1 {
                            let argument = &arguments.args[0];
                            if let GenericArgument::Type(option_type) = argument {
                                inner_type = Some(option_type);
                            }
                        }
                    }

                    if last_segment.ident == "Option" {
                        is_option = true;
                    }
                    else if last_segment.ident == "Vec" {
                        is_vec = true;
                    }
                }
        
                expected_keys.push(quote! { #formatted_name });
                key_parsers.push(if is_bool {
                    quote! {
                        #formatted_name => {
                            argument_map.insert(#formatted_name, std::boxed::Box::new(true));
                            expected_keys.remove(#formatted_name);
                        },
                    } 
                }
                else if is_option {
                    quote! {
                        #formatted_name => {
                            argument_map.insert(#formatted_name, std::boxed::Box::new(Some(<#inner_type as curds_core_abstraction::cli::CliArgumentParse>::parse(arguments)?)));
                            expected_keys.remove(#formatted_name);
                        },
                    } 
                }
                else if is_vec {
                    quote! {
                        #formatted_name => {
                            let mut parsed_vector: Vec<#inner_type> = Vec::new();
                            loop {
                                if arguments.len() > 0 {
                                    let value = arguments.pop().unwrap();
                                    if value == "--" {
                                        break;
                                    }
                                    else {
                                        parsed_vector.push(<#inner_type as curds_core_abstraction::cli::CliArgumentParse>::parse(&mut vec![value])?);
                                    }
                                } 
                                else {
                                    break;
                                }
                            }

                            argument_map.insert(#formatted_name, std::boxed::Box::new(parsed_vector));
                            expected_keys.remove(#formatted_name);
                        },
                    } 
                }
                else {
                    quote! {
                        #formatted_name => {
                            argument_map.insert(#formatted_name, std::boxed::Box::new(<#ty as curds_core_abstraction::cli::CliArgumentParse>::parse(arguments)?));
                            expected_keys.remove(#formatted_name);
                        },
                    }
                });
                field_initializers.push(if is_bool {
                    quote! {
                        #name: match argument_map.remove(#formatted_name) {
                            Some(value) => *value.downcast::<#ty>().unwrap(),
                            None => false,
                        },
                    }
                }
                else if is_option {
                    quote! {
                        #name: match argument_map.remove(#formatted_name) {
                            Some(value) => *value.downcast::<#ty>().unwrap(),
                            None => None,
                        },
                    }
                }
                else {
                    quote! {
                        #name: match argument_map.remove(#formatted_name) {
                            Some(value) => *value.downcast::<#ty>().unwrap(),
                            None => return Err(curds_core_abstraction::cli::CliArgumentParseError::UnrecognizedKey(#formatted_name.to_string())),
                        },
                    }
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