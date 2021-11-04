use super::*;

#[derive(Clone)]
pub enum RoutingDefinition {
    Pipeline(PipelineDefinition),
    Chain(ChainDefinition),
}
impl Default for RoutingDefinition {
    fn default() -> Self {
        Self::Pipeline(PipelineDefinition::default())
    }
}

impl RoutingDefinition {
    pub fn return_tokens(&self) -> TokenStream {
        match self {
            Self::Pipeline(pipeline_definition) => pipeline_definition.return_tokens(),
            Self::Chain(chain_definition) => chain_definition.return_tokens(),
        }
    }

    pub fn implementation_tokens(self, base_name: &Ident, context_type: &Type) -> TokenStream {
        match self {
            Self::Pipeline(pipeline_definition) => pipeline_definition.implementation_tokens(base_name, context_type),
            Self::Chain(chain_definition) => chain_definition.implementation_tokens(base_name, context_type),
        }
    }

    pub fn trait_tokens(self, visibility: &Visibility, parent_trait: &Ident, base_name: &Ident, message_type: &Type) -> TokenStream {
        match self {
            Self::Pipeline(pipeline_definition) => pipeline_definition.trait_tokens(visibility, parent_trait, base_name, message_type),
            Self::Chain(chain_definition) => chain_definition.trait_tokens(visibility, parent_trait, base_name, message_type),
        }
    }
}