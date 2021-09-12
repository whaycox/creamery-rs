mod dependency_injection;

use proc_macro::TokenStream;
use syn::{*, parse::*, punctuated::*, spanned::*};
use quote::*;
use std::collections::HashSet;
use rand::*;
use std::cell::RefCell;
use std::borrow::Borrow;
use core::fmt::Debug;
use std::marker::PhantomData;
use syn::token::CustomToken;

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
    parse_macro_input!(item as StructDefinition)
        .quote(Vec::new())
        .into()
}