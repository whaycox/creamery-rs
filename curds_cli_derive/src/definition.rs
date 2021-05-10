
use super::attributes::parse_attributes;
use syn::{DeriveInput, Attribute, Meta, MetaList, NestedMeta, Lit, Variant, Field, Fields};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident};

#[derive(Debug)]
pub struct CliDefinitionTokens {
    pub type_name: TokenStream,
    pub impl_trait_tokens: TokenStream,
    pub impl_type_tokens: TokenStream,
}
impl CliDefinitionTokens {
    pub fn new(input: &DeriveInput) -> Self {
        let type_ident = &input.ident;
        CliDefinitionTokens {
            type_name: quote!(#type_ident),
            impl_trait_tokens: Self::impl_trait_tokens(&input),
            impl_type_tokens: Self::impl_type_tokens(&input),
        }
    }
    
    fn impl_trait_tokens(input: &DeriveInput) -> TokenStream {
        let type_name = &input.ident;
        quote! {
            impl ::curds_cli_core::CliArgumentDefinition for #type_name
        }
    }

    fn impl_type_tokens(input: &DeriveInput) -> TokenStream {
        let type_name = &input.ident;
        quote! {
            impl #type_name
        }
    }
}

#[derive(Debug)]
pub struct CliVariantTokens {
    pub match_tokens: TokenStream,
    pub parse_tokens: TokenStream,
}
impl CliVariantTokens {
    pub fn new(type_ident: &TokenStream, input: &Variant) -> Self {
        CliVariantTokens {
            match_tokens: Self::match_tokens(input),
            parse_tokens: Self::parse_tokens(type_ident, input),
        }
    }

    fn match_tokens(input: &Variant) -> TokenStream {
        let key_names = parse_attributes("key", &input.attrs);
        if key_names.len() == 0 {
            panic!("Require at least one key to match upon")
        }

        let lower_key_names = key_names.iter().map(|key| key.value().to_lowercase());
        let parse_variant_name = format_ident!("parse_{}", input.ident);
        quote! {
            #(#lower_key_names)|* => Self::#parse_variant_name(arguments)
        }
    }

    fn parse_tokens(type_ident: &TokenStream, variant: &Variant) -> TokenStream {
        let variant_ident = &variant.ident;
        let parse_variant_name = format_ident!("parse_{}", variant.ident);

        match &variant.fields {
            Fields::Unit => {
                quote! {
                    fn #parse_variant_name(arguments: &mut ArgumentCollection) -> curds_cli_core::CliParseResult<Self> {
                        Ok(#type_ident::#variant_ident)
                    }
                }
            },
            Fields::Unnamed(unnamed_fields) => {
                let mut field_tokens = Vec::<TokenStream>::new();
                for field in &unnamed_fields.unnamed {
                    field_tokens.push(Self::parse_field_token(field));
                }

                quote! {
                    fn #parse_variant_name(arguments: &mut ArgumentCollection) -> curds_cli_core::CliParseResult<Self> {
                        Ok(#type_ident::#variant_ident(#(#field_tokens),*))
                    }
                }
            },
            Fields::Named(named_fields) => {
                let mut field_tokens = Vec::<TokenStream>::new();
                for field in &named_fields.named {
                    field_tokens.push(Self::parse_field_token(field));
                }

                quote! {
                    fn #parse_variant_name(arguments: &mut ArgumentCollection) -> curds_cli_core::CliParseResult<Self> {
                        Ok(#type_ident::#variant_ident {
                            #(#field_tokens,)*
                        })
                    }
                }
            }
        }
    }

    fn parse_field_token(field: &Field) -> TokenStream {
        if let Some(field_ident) = &field.ident {
            return quote! {
                #field_ident: arguments.pop()?
            }
        }
        else {
            return quote! {
                arguments.pop()?
            }
        }
    }
}