use super::*;

mod injected_definition;
mod service_provider;
mod service_production;
mod singleton_collection;
mod generated_definition;
mod singleton_description;
mod singleton_identifier;
mod provider_definition;
mod forwarded_definition;

pub use injected_definition::*;
pub use service_provider::*;
pub use service_production::*;
pub use singleton_collection::*;
pub use generated_definition::*;
pub use singleton_description::*;
pub use singleton_identifier::*;
pub use provider_definition::*;
pub use forwarded_definition::*;

use proc_macro2::{TokenStream, Span};
use syn::{ItemStruct};