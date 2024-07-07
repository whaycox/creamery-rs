mod whey;
mod cli;

use proc_macro::TokenStream;
use syn::{*, parse::*, punctuated::*};
use quote::*;
use std::collections::HashMap;

use whey::*;
use cli::*;

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