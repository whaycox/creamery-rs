mod dependency_injection;
mod whey;
//mod message_dispatch;

use proc_macro::TokenStream;
use syn::{*, parse::*, punctuated::*, spanned::*};
use quote::*;
use std::collections::{HashSet, HashMap};
use rand::*;

use dependency_injection::*;
use whey::*;
//use message_dispatch::*;

#[proc_macro_attribute]
pub fn injected(_attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as InjectedDefinition)
        .quote()
        .into()
}

#[proc_macro_attribute]
pub fn service_provider(_attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as ServiceProviderDefinition)
        .quote()
        .into()
}

#[proc_macro_attribute]
pub fn whey_context(attr: TokenStream, item: TokenStream) -> TokenStream {
    let test_type = parse_macro_input!(attr as Ident);
    parse_macro_input!(item as WheyContext)
        .quote(test_type)
        .into()
}

#[proc_macro_attribute]
pub fn whey(_attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as WheyTest)
        .quote()
        .into()
}

/* 
#[proc_macro_attribute]
pub fn message_dispatch(attr: TokenStream, item: TokenStream) -> TokenStream {
    let message_trait = parse_macro_input!(attr as Ident);
    parse_macro_input!(item as DispatchDefinition)
        .quote(message_trait)
        .into()
}

#[proc_macro_attribute]
pub fn whey_mock(_attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as WheyMock)
        .quote()
        .into()
} */