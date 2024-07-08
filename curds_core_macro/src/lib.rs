mod whey;
mod cli;

use proc_macro::TokenStream;
use proc_macro_crate::FoundCrate;
use syn::{*, parse::*};
use quote::*;

use whey::*;
use cli::*;

const SELF_NAME: &str = "curds_core";
fn resolve_crate_name() -> proc_macro2::TokenStream {
    match proc_macro_crate::crate_name(SELF_NAME) {
        Ok(FoundCrate::Itself) => if let Ok(crate_name) = std::env::var("CARGO_CRATE_NAME") {
            if crate_name == SELF_NAME { quote! { crate } } else { quote! { curds_core } }
        }
        else { quote! { crate } },
        Ok(FoundCrate::Name(_)) | Err(_) => quote!{ curds_core },
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