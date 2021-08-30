mod dependency_injection;

use proc_macro::TokenStream;
use syn::{*, parse::*, punctuated::*};
use quote::*;

use curds_core_abstraction::*;
use dependency_injection::*;

#[proc_macro_attribute]
pub fn service_provider(_attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as ServiceProviderDefinition)
        .quote()
        .into()
}

#[proc_macro_attribute]
pub fn injected(_attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as InjectedDefinition)
        .quote()
        .into()
}