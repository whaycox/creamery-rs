use super::*;

mod dispatch_definition;
mod message_definition;
mod pipeline_definition;
mod chain_definition;
mod dispatch_routing;
mod pipeline_stage;
mod serial_template;
mod parallel_template;
mod stage_return;
mod dispatch_defaults;
mod routing_parameters;
mod serial_route;
mod serial_stage;
mod parallel_route;
mod serial_template_stage;

pub use dispatch_definition::*;
pub use message_definition::*;
pub use pipeline_definition::*;
pub use chain_definition::*;
pub use dispatch_routing::*;
pub use pipeline_stage::*;
pub use serial_template::*;
pub use parallel_template::*;
pub use stage_return::*;
pub use dispatch_defaults::*;
pub use routing_parameters::*;
pub use serial_route::*;
pub use serial_stage::*;
pub use parallel_route::*;
pub use serial_template_stage::*;

use proc_macro2::{TokenStream, Span};

const HANDLER_NAME: &str = "Handler";