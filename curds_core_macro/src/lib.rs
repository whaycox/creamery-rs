mod dependency_injection;
mod whey;
mod message_dispatch;
mod cli;

use proc_macro::TokenStream;
use syn::{*, parse::*, punctuated::*, spanned::*};
use quote::*;
use std::collections::{HashSet, HashMap};
use rand::*;

use dependency_injection::*;
use whey::*;
use message_dispatch::*;
use cli::*;

#[proc_macro_attribute]
pub fn injected(_: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as InjectedDefinition)
        .quote()
        .into()
}

#[proc_macro_attribute]
pub fn service_provider(_: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as ServiceProviderDefinition)
        .quote()
        .into()
}

#[proc_macro_derive(Scoped)]
pub fn derive_scoped(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as ScopedItem)
        .quote()
        .into()
}

#[proc_macro_attribute]
pub fn whey_mock(_: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as WheyMock)
        .quote()
        .into()
}

#[proc_macro]
pub fn expect_calls(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as WheyExpectedCalls)
        .quote()
        .into()
}

#[proc_macro]
pub fn mock_default_return(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as WheyDefaultReturn)
        .quote()
        .into()
}

#[proc_macro]
pub fn mock_return(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as WheyReturnGenerator)
        .quote()
        .into()
}

#[proc_macro]
pub fn mock_input(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as WheyInputComparison)
        .quote()
        .into()
}

#[proc_macro]
pub fn mock_sequence(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as WheySequence)
        .quote()
        .into()
}

#[proc_macro_attribute]
pub fn message_dispatch(attr: TokenStream, item: TokenStream) -> TokenStream {
    let message_trait = parse_macro_input!(attr as MessageTraitDefinition);
    parse_macro_input!(item as DispatchDefinition)
        .quote(message_trait)
        .into()
}

#[proc_macro_attribute]
pub fn cli_arguments(attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as CliArgumentDefinition)
        .quote()
        .into()
}