mod dependency_injection;
mod whey;
mod message_dispatch;

use proc_macro::TokenStream;
use syn::{*, parse::*, punctuated::*, spanned::*, token::Trait};
use quote::*;
use std::collections::{HashSet, HashMap};
use rand::*;

use dependency_injection::*;
use whey::*;
use message_dispatch::*;

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

#[proc_macro_derive(Scoped)]
pub fn derive_scoped(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as ScopedItem)
        .quote()
        .into()
}

#[proc_macro_attribute]
pub fn whey_context(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut test_type: Option<Ident> = None;
    if !attr.is_empty() {
        test_type = parse_macro_input!(attr as Option<Ident>);
    }
    
    parse_macro_input!(item as WheyContext)
        .quote(test_type)
        .into()
}

#[proc_macro_attribute]
pub fn whey_mock(_attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as WheyMock)
        .quote()
        .into()
}

/* #[proc_macro_attribute]
pub fn whey(_attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as WheyTest)
        .quote()
        .into()
} */

/* #[proc_macro]
pub fn expect(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as WheyExpectation)
        .quote()
        .into()  
} */

/* #[proc_macro_attribute]
pub fn message_dispatch(attr: TokenStream, item: TokenStream) -> TokenStream {
    let message_trait = parse_macro_input!(attr as Ident);
    parse_macro_input!(item as DispatchDefinition)
        .quote(message_trait)
        .into()
} */