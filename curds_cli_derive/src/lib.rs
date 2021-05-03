mod parse;
mod definition;
mod attributes;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CliArguments, attributes(name, description, key, alias))]
pub fn cli_arguments_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    parse::definition_tokens(input)
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}