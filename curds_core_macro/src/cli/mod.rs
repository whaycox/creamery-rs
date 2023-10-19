use super::*;

mod argument_definition;
mod enum_argument_definition;
mod struct_argument_definition;

pub use argument_definition::*;
pub use enum_argument_definition::*;
pub use struct_argument_definition::*;

use proc_macro2::{TokenStream, Span};
use std::sync::OnceLock;
use regex::Regex;

static CAMEL_CASE_SPLITTER: OnceLock<Regex> = OnceLock::new();
fn format_argument_name(name: &Ident) -> Ident {
    let splitter = CAMEL_CASE_SPLITTER.get_or_init(|| Regex::new("[A-Z][a-z]*").unwrap());
    let name_string = name.to_string();
    let parts: Vec<String> = splitter.find_iter(&name_string)
        .map(|part| {
            let mut part_string = part.as_str().to_owned();
            if let Some(char) = part_string.get_mut(0..1) {
                char.make_ascii_lowercase();
            }

            part_string
        })
        .collect();
    
    Ident::new(&parts.join("_"), name.span())
}