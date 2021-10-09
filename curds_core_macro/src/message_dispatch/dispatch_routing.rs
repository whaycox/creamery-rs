use super::*;

#[derive(Clone)]
pub struct DispatchRouting {
    pub message_type: Type,
    pub context_type: Type,
    definition: RoutingDefinition, 
}

impl Parse for DispatchRouting {
    fn parse(input: ParseStream) -> Result<Self> {
        let message_type: Type = input.parse()?;
        input.parse::<Token![~]>()?;
        let context_type: Type = input.parse()?;
        let definition: RoutingDefinition = if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            let return_type: Type = input.parse()?;
            RoutingDefinition::Pipeline(PipelineDefinition::new(return_type))
        }
        else if input.peek(Token![&]) {
            input.parse::<Token![&]>()?;
            let pipeline_content;
            braced!(pipeline_content in input);
            let pipeline: PipelineDefinition = pipeline_content.parse()?;
            RoutingDefinition::Pipeline(pipeline)
        }
        else if input.peek(token::Bracket) {
            let chain_content;
            bracketed!(chain_content in input);
            let chain: ChainDefinition = chain_content.parse()?;
            RoutingDefinition::Chain(chain)
        }
        else { RoutingDefinition::default() };

        Ok(Self {
            message_type: message_type,
            context_type: context_type,
            definition: definition,
        })
    }
}

impl DispatchRouting {
    pub fn return_type(&self) -> Option<Type> { self.definition.return_type() }

    pub fn quote(self, base_name: &Ident) -> TokenStream {
        let context_type = self.context_type;
        let stage_implementations = self.definition.implementation_tokens(base_name, &context_type);

        quote! {
            let context = curds_core_abstraction::dependency_injection::ServiceGenerator::<std::rc::Rc<#context_type>>::generate(self);
            #stage_implementations
        }
    }

    pub fn trait_tokens(self, visibility: &Visibility, parent_trait: &Ident, base_name: &Ident) -> TokenStream {
        self.definition.trait_tokens(visibility, parent_trait, base_name, &self.message_type)
    }
}