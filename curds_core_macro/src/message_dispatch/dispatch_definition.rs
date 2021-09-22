use super::*;

pub struct DispatchDefinition {
    messages: Vec<MessageDefinition>,
    provider_definition: ServiceProviderDefinition,
}

impl Parse for DispatchDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut messages: Vec<MessageDefinition> = Vec::new();
        let attributes = Attribute::parse_outer(&input.fork())?;
        let mut provider_definition: ServiceProviderDefinition = input.parse()?;
        for attribute in attributes {
            if !attribute.path.is_ident(CLONES_IDENTIFIER) &&
                !attribute.path.is_ident(SCOPES_IDENTIFIER) &&
                !attribute.path.is_ident(SCOPES_SINGLETON_IDENTIFIER) &&
                !attribute.path.is_ident(GENERATES_IDENTIFIER) &&
                !attribute.path.is_ident(GENERATES_SINGLETON_IDENTIFIER) &&
                !attribute.path.is_ident(FORWARDS_IDENTIFIER) &&
                !attribute.path.is_ident(FORWARDS_SINGLETON_IDENTIFIER) {
                let message = MessageDefinition::parse(attribute)?;
                provider_definition.generates(message.context_type());
                messages.push(message);
            }
        }

        Ok(Self {
            messages: messages,
            provider_definition: provider_definition,
        })
    }
}

impl DispatchDefinition {
    pub fn quote(self, message_trait: Ident) -> TokenStream {
        let visibility = self.provider_definition.visibility();
        let message_signatures: Vec<TokenStream> = self.messages
            .clone()
            .into_iter()
            .map(|message| message.signature_tokens())
            .collect();
        let provider_definition = self.provider_definition
            .clone()
            .quote();
        let message_traits: Vec<TokenStream> = self.messages
            .clone()
            .into_iter()
            .map(|message| message.trait_tokens(&visibility, &message_trait))
            .collect();
        let provider_name = self.provider_definition.name();
        let message_implementations: Vec<TokenStream> = self.messages
            .clone()
            .into_iter()
            .map(|message| message.implementation_tokens())
            .collect();

        quote! {
            #visibility trait #message_trait {
                #(#message_signatures)*
            }
            #provider_definition

            #(#message_traits)*

            impl #message_trait for #provider_name {
                #(#message_implementations)*
            }
        }
    }
}