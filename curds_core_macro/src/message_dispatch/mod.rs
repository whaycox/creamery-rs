use super::*;

mod dispatch_definition;
mod message_trait_definition;
mod message_definition;
mod pipeline_definition;
mod chain_definition;
mod dispatch_routing;
mod pipeline_stage;
mod chain_stage;
mod message_defaults;
mod pipeline_default;
mod stage_return;

pub use dispatch_definition::*;
pub use message_trait_definition::*;
pub use message_definition::*;
pub use pipeline_definition::*;
pub use chain_definition::*;
pub use dispatch_routing::*;
pub use pipeline_stage::*;
pub use chain_stage::*;
pub use message_defaults::*;
pub use pipeline_default::*;
pub use stage_return::*;

use proc_macro2::{TokenStream, Span};

const HANDLER_NAME: &str = "Handler";