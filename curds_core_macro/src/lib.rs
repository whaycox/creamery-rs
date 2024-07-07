mod whey;
mod cli;

use proc_macro::TokenStream;
use proc_macro_crate::FoundCrate;
use syn::{*, parse::*, punctuated::*};
use quote::*;
use std::collections::HashMap;

use whey::*;
use cli::*;

fn resolve_crate_name() -> proc_macro2::TokenStream {
    match proc_macro_crate::crate_name("curds_core") {
        Ok(found_crate) => match found_crate {
            FoundCrate::Itself => quote! { crate },
            _ => quote! { curds_core },
        },
        Err(_) => quote!{ curds_core },
    }
}

#[proc_macro_attribute]
pub fn whey_mock(_: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as WheyMock)
        .quote()
        .into()
}

#[proc_macro_attribute]
pub fn cli_arguments(_: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as CliArgumentDefinition)
        .quote()
        .into()
}