use super::*;

mod forward_provider;
mod service_provider;
mod service_production;
mod transient_map;
mod injected_definition;
mod injected_implementation;
mod dependency_definition;
mod transient_generate;
mod clone_provider;
mod injected_defaults;
mod singleton_generate;

pub use forward_provider::*;
pub use service_provider::*;
pub use service_production::*;
pub use transient_map::*;
pub use injected_definition::*;
pub use injected_implementation::*;
pub use dependency_definition::*;
pub use transient_generate::*;
pub use clone_provider::*;
pub use injected_defaults::*;
pub use singleton_generate::*;

use proc_macro2::{TokenStream, Span};

const SERVICES_LIBRARY_NAME: &str = "_curds_core_services";