use super::*;

pub enum DispatchRouting {
    Pipeline(PipelineDefinition),
    Chain(ChainDefinition),
}

impl From<PipelineDefinition> for DispatchRouting {
    fn from(value: PipelineDefinition) -> Self { DispatchRouting::Pipeline(value) }
}
impl From<ChainDefinition> for DispatchRouting {
    fn from(value: ChainDefinition) -> Self { DispatchRouting::Chain(value) }
}

impl DispatchRouting {
    pub fn mutable(&self) -> bool {
        match self {
            Self::Pipeline(pipeline) => pipeline.mutable,
            Self::Chain(chain) => chain.mutable,
        }
    }

    pub fn message_type(&self) -> &Type {
        match self {
            Self::Pipeline(pipeline) => &pipeline.message,
            Self::Chain(chain) => &chain.message,
        }
    }
    pub fn context_type(&self) -> Type {
        match self {
            Self::Pipeline(pipeline) => pipeline.context.clone(),
            Self::Chain(chain) => chain.context.clone(),
        }
    }
    pub fn return_type(&self, error_type: &Type) -> TokenStream {
        match self {
            Self::Pipeline(pipeline) => pipeline.return_type(error_type),
            Self::Chain(chain) => chain.return_type(error_type),
        }
    }

    pub fn trait_tokens(&self, visibility: &Visibility, message_trait: &MessageTraitDefinition, base_name: &Ident) -> TokenStream {
        match self {
            Self::Pipeline(pipeline) => pipeline.trait_tokens(visibility, message_trait, base_name),
            Self::Chain(chain) => chain.trait_tokens(visibility, message_trait, base_name),
        }
    }

    pub fn implementation_tokens(&self, base_name: &Ident) -> TokenStream {
        match self {
            Self::Pipeline(pipeline) => pipeline.implementation_tokens(base_name),
            Self::Chain(chain) => chain.implementation_tokens(base_name),
        }
    }
}