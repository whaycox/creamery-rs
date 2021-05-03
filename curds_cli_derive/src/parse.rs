use proc_macro2::{TokenStream, Ident};
use syn::{DeriveInput, Result, Attribute, Data, DataEnum, Variant};
use quote::quote;
use super::definition::{CliDefinitionTokens, CliVariantTokens};
use curds_cli_definition::ArgumentCollection;

pub fn definition_tokens(input: DeriveInput) -> Result<TokenStream> {
    let definition = CliDefinitionTokens::new(&input);
    match input.data {
        Data::Enum(enum_input) => enum_definition_tokens(definition, enum_input),
        _ => panic!("Unsupported derive type")
    }
}

fn enum_definition_tokens(definition: CliDefinitionTokens, enum_input: DataEnum) -> Result<TokenStream> {    
    let impl_trait_tokens = definition.impl_trait_tokens;
    let impl_type_tokens = definition.impl_type_tokens;

    let mut variant_tokens = Vec::<CliVariantTokens>::new();
    for variant in enum_input.variants {
        variant_tokens.push(CliVariantTokens::new(&definition.type_name, &variant))
    }
    let variant_match_tokens = variant_tokens.iter().map(|variant| &variant.match_tokens);
    let variant_parse_tokens = variant_tokens.iter().map(|variant| &variant.parse_tokens);

    let final_tokens = quote! {
        #impl_trait_tokens {
            fn parse(key: String, arguments: &mut ArgumentCollection) -> Self {
                println!("Parsing with key {}", key);
                match key.to_lowercase().as_str() {
                    #(#variant_match_tokens,)*
                    _ => panic!("Unsupported operation: {}", key)
                }
            }
        }

        #impl_type_tokens {
            #(#variant_parse_tokens)*
        }        
    };
    Ok(final_tokens)
}