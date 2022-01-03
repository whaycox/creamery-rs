use super::*;

#[derive(Clone, Debug)]
pub enum DispatchRouting {
    Pipeline(PipelineDefinition),
    Chain(ChainDefinition)
}
impl Parse for DispatchRouting {
    fn parse(input: ParseStream) -> Result<Self> {
        let message_type: Type = input.parse()?;
        input.parse::<Token![~]>()?;
        let context_type: Type = input.parse()?;
        let mut response_type: Option<Type> = None;
        let parsed: Self = if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            response_type = Some(input.parse()?);
            Self::Pipeline(PipelineDefinition::implicit(RoutingParameters {
                message_type: message_type,
                context_type: context_type,
                response_type: response_type,
            }))
        }
        else if input.peek(Token![&]) {
            input.parse::<Token![&]>()?;
            if input.peek(token::Brace) {
                let route: SerialRoute = SerialRoute::parse_explicit(input)?;
                response_type = route.explicit_return();
                Self::Pipeline(PipelineDefinition::explicit(RoutingParameters {
                    message_type: message_type,
                    context_type: context_type,
                    response_type: response_type,
                }, route))
            }
            else {
                Self::Pipeline(PipelineDefinition::implicit(RoutingParameters {
                    message_type: message_type,
                    context_type: context_type,
                    response_type: response_type,
                }))
            }
        }
        else if input.peek(Token![|]) {
            let route: ParallelRoute = input.parse()?;
            if input.peek(Token![->]) {
                input.parse::<Token![->]>()?;
                response_type = Some(input.parse()?)
            }
            Self::Chain(ChainDefinition::new(RoutingParameters {
                message_type: message_type,
                context_type: context_type,
                response_type: response_type,
            }, Some(route)))
        }
        else {
            Self::Pipeline(PipelineDefinition::implicit(RoutingParameters {
                message_type: message_type,
                context_type: context_type,
                response_type: response_type,
            })) 
        };

        Ok(parsed)
    }
}

impl DispatchRouting {
    pub fn context_type(&self) -> Type { 
        match self {
            Self::Pipeline(definition) => definition.context_type(),
            Self::Chain(definition) => definition.context_type(),
        }
     }

    pub fn expand(self, defaults: &DispatchDefaults) -> Self {
        match self {
            Self::Pipeline(definition) => Self::Pipeline(definition.expand(defaults)),
            Self::Chain(definition) => Self::Chain(definition.expand(defaults)),
        }
    }

    pub fn signature_tokens(self) -> TokenStream {
        match self {
            Self::Pipeline(definition) => definition.signature_tokens(),
            Self::Chain(definition) => definition.signature_tokens(),
        }
    }

    pub fn quote(self, base_name: &Ident) -> TokenStream {
        match self {
            Self::Pipeline(definition) => definition.quote(base_name),
            Self::Chain(definition) => definition.quote(base_name),
        }
    }

    pub fn trait_tokens(self, visibility: &Visibility, parent_trait: &Ident, base_name: &Ident) -> TokenStream {
        match self {
            Self::Pipeline(definition) => definition.trait_tokens(visibility, parent_trait, base_name),
            Self::Chain(definition) => definition.trait_tokens(visibility, parent_trait, base_name),
        }
    }
}