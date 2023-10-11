use super::*;

const PIPELINE_DEFAULT: &str = "pipeline_default";
const CHAIN_DEFAULT: &str = "chain_default";

pub struct DispatchDefinition {
    provider_definition: ServiceProviderDefinition,
    messages: Vec<MessageDefinition>,
}

impl Parse for DispatchDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut provider_definition: ServiceProviderDefinition = input.parse()?;
        let message_defaults = Self::parse_message_defaults(provider_definition.item())?;
        let mut messages = Self::parse_messages(provider_definition.item())?;
        messages = messages
            .into_iter()
            .map(|message| message.apply_template(&message_defaults))
            .collect();

        let mut message_contexts: HashSet<Type> = HashSet::new();
        for message in &messages {
            message_contexts.insert(message.context_type());
        }
        for context in message_contexts {
            provider_definition.add_production(context);
        }

        Ok(Self {
            provider_definition,
            messages,
        })
    }
}

impl DispatchDefinition {
    fn parse_message_defaults(provider: &mut ItemStruct) -> Result<MessageDefaults> {
        let mut defaults = MessageDefaults::new();
        let length = provider.attrs.len();
        if length > 0 {
            let mut attribute_index = length - 1;
            loop {
                let attribute = &provider.attrs[attribute_index];
                if attribute.path.is_ident(PIPELINE_DEFAULT) {
                    defaults.pipeline = Some(attribute.parse_args()?);
                    provider.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(CHAIN_DEFAULT) {
                    defaults.chain = Some(attribute.parse_args()?);
                    provider.attrs.remove(attribute_index);
                }
    
                if attribute_index == 0 {
                    break;
                }
                attribute_index = attribute_index - 1;
            }
        }

        Ok(defaults)
    }
    fn parse_messages(provider: &mut ItemStruct) -> Result<Vec<MessageDefinition>> {
        let mut messages: Vec<MessageDefinition> = Vec::new();
        let length = provider.attrs.len();
        if length > 0 {
            let mut attribute_index = length - 1;
            loop {
                let attribute = &provider.attrs[attribute_index];
                match attribute.path.get_ident() {
                    Some(message_name) => {
                        if let Ok(pipeline_definition) = attribute.parse_args::<PipelineDefinition>() {
                            messages.push(MessageDefinition::new(message_name, attribute.path.span(), pipeline_definition.into()));
                            provider.attrs.remove(attribute_index);
                        }
                        else if let Ok(chain_definition) = attribute.parse_args::<ChainDefinition>() {
                            messages.push(MessageDefinition::new(message_name, attribute.path.span(), chain_definition.into()));
                            provider.attrs.remove(attribute_index);
                        }
                    },
                    None => {},
                }
    
                if attribute_index == 0 {
                    break;
                }
                attribute_index = attribute_index - 1;
            }
            messages.reverse();
        }

        Ok(messages)
    }

    pub fn quote(self, message_trait: MessageTraitDefinition) -> TokenStream {
        let visibility = self.provider_definition.visibility();
        let message_trait_name = &message_trait.name;
        let message_signatures: Vec<TokenStream> = self.messages
            .iter()
            .map(|message| message.signature(&message_trait))
            .collect();
        let provider_definition = self.provider_definition.quote();
        let message_traits: Vec<TokenStream> = self.messages
            .iter()
            .map(|message| message.trait_tokens(visibility, &message_trait))
            .collect();
        let provider_name = self.provider_definition.name();
        let (impl_generics, type_generics, where_clause) = self.provider_definition.generics().split_for_impl();
        let message_implementations: Vec<TokenStream> = self.messages
            .iter()
            .map(|message| message.implementation_tokens(&message_trait))
            .collect();

        quote! {
            //#[whey_mock]
            #visibility trait #message_trait_name {
                #(#message_signatures)*
            }
            #provider_definition

            #(#message_traits)*

            impl #impl_generics #message_trait_name for #provider_name #type_generics #where_clause {
                #(#message_implementations)*
            }
        }
    }
}