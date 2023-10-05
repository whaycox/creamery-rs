use super::*;

pub enum DispatchRouting {
    Pipeline(PipelineDefinition),
}

impl From<PipelineDefinition> for DispatchRouting {
    fn from(value: PipelineDefinition) -> Self { DispatchRouting::Pipeline(value) }
}

impl DispatchRouting {
    pub fn message_type(&self) -> &Type {
        match self {
            DispatchRouting::Pipeline(pipeline) => &pipeline.message,
        }
    }
    pub fn context_type(&self) -> Type {
        match self {
            DispatchRouting::Pipeline(pipeline) => pipeline.context.clone(),
        }
    }
    pub fn return_type(&self) -> &Option<Type> {
        match self {
            DispatchRouting::Pipeline(pipeline) => pipeline.return_type(),
        }
    }

    pub fn trait_tokens(&self, visibility: &Visibility, parent_trait: &Ident, base_name: &Ident) -> TokenStream {
        match self {
            Self::Pipeline(pipeline_definition) => pipeline_definition.trait_tokens(visibility, parent_trait, base_name),
        }
    }

    pub fn implementation_tokens(&self, base_name: &Ident) -> TokenStream {
        match self {
            Self::Pipeline(pipeline_definition) => pipeline_definition.implementation_tokens(base_name),
        }
    }
}